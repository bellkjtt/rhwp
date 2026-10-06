import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { launchBrowser, createPage, loadApp, loadHwpFile, closeBrowser } from './helpers.mjs';
// 정상 제품의 loadDocument·font 준비·paint 경로를 실행한다.
// 절대 화면 좌표나 SVG 해시 대신 수식 소유 영역의 내용과 replay 진단을 검사한다.
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const out = process.env.RHWP_ISSUE7593_E2E_OUT ?? path.join(root, 'output/e2e/issue7593');
mkdirSync(out, { recursive: true });
const expectedWasm = createHash('sha256').update(readFileSync(path.join(root, 'pkg/rhwp_bg.wasm'))).digest('hex');
const browser = await launchBrowser();
const records = [];
try {
  for (const backend of ['canvas2d', 'canvaskit']) {
    for (const fixture of ['issue7593/repro-equation-issue7593.hwpx', 'issue7593/repro-dyad-min-no-script.hwpx']) {
      const page = await createPage(browser, 1400, 1600);
      await page.setCacheEnabled(false);
      await page.setBypassServiceWorker(true);
      const errors = [], wasmResponses = [];
      page.on('pageerror', err => errors.push(err.message));
      page.on('response', resp => {
        if ((resp.headers()['content-type'] || '').includes('application/wasm') && resp.url().includes('rhwp_bg.wasm')) {
          wasmResponses.push(resp.buffer().then(bytes => createHash('sha256').update(bytes).digest('hex')));
        }
      });
      await loadApp(page, `?renderer=${backend}&profile=print&canvaskitSurface=software`);
      const loaded = await loadHwpFile(page, fixture);
      await page.evaluate(() => window.__eventBus.emit('zoom-changed', 1));
      await new Promise(resolve => setTimeout(resolve, 1500));
      const result = await page.evaluate(() => {
        const tree = JSON.parse(window.__wasm.doc.getPageRenderTree(0));
        const nodes = [];
        const visit = n => { nodes.push(n); for (const c of n.children || []) visit(c); };
        visit(tree);
        const canvases = [...document.querySelectorAll('#scroll-container canvas')];
        const merged = document.createElement('canvas');
        merged.width = canvases[0].width; merged.height = canvases[0].height;
        const context = merged.getContext('2d');
        context.fillStyle = '#ffffff'; context.fillRect(0, 0, merged.width, merged.height);
        for (const canvas of canvases) context.drawImage(canvas, 0, 0);
        const equations = nodes.filter(n => n.type === 'Equation');
        const pageWidth = (tree.bbox.width ?? tree.bbox.w);
        const content = context.getImageData(0, 0, merged.width, merged.height).data;
        const equationInk = equations.map(eq => {
          const factor = merged.width / pageWidth;
          const box = eq.bbox;
          let count = 0;
          for (let y = Math.floor(box.y * factor); y < Math.ceil((box.y + (box.height ?? box.h)) * factor); y++)
            for (let x = Math.floor(box.x * factor); x < Math.ceil((box.x + (box.width ?? box.w)) * factor); x++) {
              const offset = (y * merged.width + x) * 4;
              if (content[offset] < 200 || content[offset + 1] < 200 || content[offset + 2] < 200) count++;
            }
          return count;
        });
        return {
          composite: merged.toDataURL('image/png'),
          equationInk,
          canvasKitDiagnostics: window.__canvasView.getCanvasKitRenderDiagnostics(0),
          effectiveBackend: window.__renderBackend,
          fallbackReason: window.__renderBackendFallbackReason,
          pageCount: window.__wasm.doc.pageCount(),
          equationCount: nodes.filter(n => n.type === 'Equation').length,
          masterLineCount: nodes.filter(n => n.type === 'MasterPage').flatMap(n => n.children || []).filter(n => n.type === 'Line').length,
          canvases: canvases.map(c => ({ width: c.width, height: c.height, png: c.toDataURL('image/png') })),
        };
      });
      const stem = fixture.includes('equation-issue') ? 'issue' : 'min';
      writeFileSync(`${out}/${backend}-${stem}-composite.png`, Buffer.from(result.composite.split(',')[1], 'base64'));
      delete result.composite;
      for (const [i, canvas] of result.canvases.entries()) {
        writeFileSync(`${out}/${backend}-${stem}-${i}.png`, Buffer.from(canvas.png.split(',')[1], 'base64'));
        delete canvas.png;
      }
      const wasmHashes = await Promise.all(wasmResponses);
      const record = { backend, fixture, loaded, ...result, errors, wasmHashes, expectedWasm };
      records.push(record);
      console.log(JSON.stringify(record));
      assert.equal(loaded.pageCount, 1);
      assert.equal(result.equationCount, stem === 'issue' ? 7 : 4);
      assert.equal(result.effectiveBackend, backend);
      assert.equal(result.fallbackReason, null);
      if (backend === 'canvaskit') {
        assert.equal(result.canvasKitDiagnostics.lastRenderCompleted, true);
        assert.deepEqual(result.canvasKitDiagnostics.lastUnexpectedUnsupportedOps, []);
        assert.equal(result.canvasKitDiagnostics.hiddenCanvas2dOverlayUsed, false);
      }
      if (wasmHashes.length === 0 || wasmHashes.some(hash => hash !== expectedWasm)) throw Error('Studio의 실제 WASM 바이트 불일치');
      if (errors.length) throw Error('브라우저 실행 오류');
      if (result.equationInk.some(count => count === 0)) throw Error('실제 화면의 수식 누락');
      await page.close();
    }
  }
} finally {
  writeFileSync(`${out}/results.json`, JSON.stringify(records, null, 2));
  await closeBrowser(browser);
}
