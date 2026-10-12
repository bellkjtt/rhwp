//! 저장 문서 수식은 한컴 명령과 LaTeX 호환 명령의 생성 문법을 구분한다.

use super::dispatch::{classify_command, EqCommandClass};
use super::tokenizer::{self, Token, TokenType};

/// bare LaTeX 전용 이름은 문서에서 일반 문자다. 인자와 첨자는 원래 토큰을
/// 유지하며, 명시적 백슬래시 확장은 공개 tokenizer의 호환 동작을 따른다.
pub(crate) fn tokenize(script: &str) -> Vec<Token> {
    let normalized = super::legacy_hwpeq::normalize(script);
    let script = normalized.as_deref().unwrap_or(script);
    let mut tokens = tokenizer::Tokenizer::new(script).tokenize();
    let chars: Vec<char> = script.chars().collect();
    for token in &mut tokens {
        if token.ty != TokenType::Command || chars.get(token.pos) == Some(&'\\') {
            continue;
        }
        if matches!(
            classify_command(&token.value),
            EqCommandClass::LatexFraction
                | EqCommandClass::RomanText
                | EqCommandClass::LatexSpacing
                | EqCommandClass::BeginEnv
                | EqCommandClass::EndEnv
        ) || token.value.eq_ignore_ascii_case("STACKREL")
        {
            token.ty = TokenType::Text;
        }
    }
    tokens
}
