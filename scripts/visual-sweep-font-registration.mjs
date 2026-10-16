// 브라우저 DOM의 family 이름과 CSS 인용 문자열을 구분한다.
const metafileFamilies = new WeakMap();

function suppliesMetafile(owner, family) {
  if (typeof owner?.getMetafileFontRequests !== 'function'
      || typeof owner?.registerMetafileFont !== 'function') return false;
  if (!metafileFamilies.has(owner)) {
    const requests = JSON.parse(owner.getMetafileFontRequests());
    metafileFamilies.set(owner, new Set(requests.flatMap(request => request.families)
      .map(name => name.toLowerCase())));
  }
  return metafileFamilies.get(owner).has(family.toLowerCase());
}

export async function installCanvasFont(family, bytes, descriptors, sourceOwner = null) {
  const font = new FontFace(family, bytes, descriptors);
  await font.load();
  document.fonts.add(font);
  // SVG 이미지의 글꼴은 canvas font setter에 나타나지 않는다. 실제 동일 bytes를
  // 별도 paint 세션에 공급하고 본문 측정/HFT slot의 의미는 그대로 둔다.
  if (suppliesMetafile(sourceOwner, family)) {
    sourceOwner.registerMetafileFont(family, new Uint8Array(bytes), 0);
  }
  // 브라우저와 같은 프로그램만 paint 세션에 전달한다.
  if (typeof sourceOwner?.registerHftOutlineFont === 'function'
      && String(descriptors.weight || '400') === '400'
      && (descriptors.style || 'normal') === 'normal') {
    sourceOwner.registerHftOutlineFont(family, new Uint8Array(bytes), 0);
  }
  return font;
}
