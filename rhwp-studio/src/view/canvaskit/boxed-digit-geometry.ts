// Rust와 같은 동봉 em 메트릭을 소비한다. 화면 px 핀이나 사유 글리프는 포함하지 않는다.
import metrics from '../../../../assets/fonts/boxed-digit-metrics.json';

export function singleDigitGeometry(number: number, fontSize: number) {
  const digit = metrics.digits[number];
  if (!digit) return null;
  const [left, bottom, right, top] = digit.source;
  const [targetLeft, targetBottom, targetRight, targetTop] = digit.target;
  const unit = fontSize / metrics.unitsPerEm;
  const scaleX = (targetRight - targetLeft) / (right - left);
  const scaleY = (targetTop - targetBottom) / (top - bottom);
  return {
    face: metrics.numberFace,
    frameX: metrics.frame[0] * unit,
    frameY: metrics.frame[1] * unit,
    frameWidth: metrics.frame[2] * unit,
    frameHeight: metrics.frame[3] * unit,
    strokeWidth: metrics.frame[4] * unit,
    scaleX, scaleY,
    numberX: (targetLeft - left * scaleX) * unit,
    numberY: (-targetBottom + bottom * scaleY) * unit,
  };
}
