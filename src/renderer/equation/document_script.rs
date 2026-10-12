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
        if token.ty != TokenType::Command {
            continue;
        }
        if chars.get(token.pos) == Some(&'\\') {
            if token.value.eq_ignore_ascii_case("sqrt") || token.value.eq_ignore_ascii_case("root")
            {
                token.value.insert(0, '\\');
            }
            continue;
        }
        let class = classify_command(&token.value);
        // 독립 한컴 Print에서 혼합 대소문자의 SQRT/ROOT는 일반 문자다.
        // 모든 대문자·소문자 및 공개 LaTeX tokenizer의 계약은 유지한다.
        let mixed_root = class == EqCommandClass::Sqrt
            && token.value != token.value.to_ascii_uppercase()
            && token.value != token.value.to_ascii_lowercase();
        if mixed_root
            || matches!(
                class,
                EqCommandClass::LatexFraction
                    | EqCommandClass::RomanText
                    | EqCommandClass::LatexSpacing
                    | EqCommandClass::BeginEnv
                    | EqCommandClass::EndEnv
            )
            || token.value.eq_ignore_ascii_case("STACKREL")
        {
            token.ty = TokenType::Text;
        }
    }
    tokens
}
