"""PDF의 균일색·채우기를 보존하는 공통 Cairo raster 명령과 실행 신원."""

import hashlib
import shutil
import subprocess
from functools import lru_cache
from pathlib import Path


PDF_RASTERIZER = "pdftocairo"
PDF_RASTER_POLICY = "poppler_cairo_constant_rgb_v1"


def pdf_raster_commands(pdf, dpi, pdf_prefix, selected_pages):
    """전체 문서와 선택 쪽을 같은 renderer·DPI·원본 쪽 번호로 출력한다."""
    common = ["-r", str(dpi), "-png", str(pdf), str(pdf_prefix)]
    if not selected_pages:
        return [[PDF_RASTERIZER, *common]]
    return [[PDF_RASTERIZER, "-f", str(page), "-l", str(page), *common]
            for page in selected_pages]


@lru_cache(maxsize=1)
def pdf_rasterizer_provenance():
    executable = shutil.which(PDF_RASTERIZER)
    identity = {
        "program": PDF_RASTERIZER,
        "backend": "cairo",
        "policy": PDF_RASTER_POLICY,
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "available": executable is not None,
    }
    if executable is None:
        return identity
    path = Path(executable).resolve()
    identity.update(executable=str(path), executable_sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    result = subprocess.run([str(path), "-v"], capture_output=True, text=True,
                            encoding="utf-8", errors="replace", timeout=10, check=False)
    lines = (result.stdout + "\n" + result.stderr).strip().splitlines()
    identity.update(version=lines[0] if lines else "unknown", version_exit_code=result.returncode)
    return identity
