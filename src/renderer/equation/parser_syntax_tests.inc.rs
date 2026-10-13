#[cfg(test)]
mod tests {
    use super::symbols::{DecoKind, FontStyleKind};
    use super::*;

    /// [PR #1226] LEFT-RIGHT 구분기호 그룹 뒤 첨자(^/_)가 그룹 전체에 결합돼야 한다.
    /// 기존엔 LEFT 분기가 try_parse_scripts 를 안 거쳐 `|x|^3` 의 ^3 가 base 없는
    /// orphan Superscript{base:Empty} 가 됐다(3 이 superscript 높이로 안 올라감).
    #[test]
    fn left_right_group_binds_trailing_script() {
        // |x|^3 → Superscript{ base: Paren, sup: 3 } (orphan 아님)
        let ast = parse("left | x right | ^3");
        let s = format!("{:?}", ast);
        assert!(
            s.contains("Superscript") && s.contains("Paren") && !s.contains("base: Empty"),
            "|x|^3 의 ^3 가 Paren 그룹에 결합돼야 함(orphan 금지): {s}"
        );

        // |x|_3 → Subscript{ base: Paren, sub: 3 }
        let sub = format!("{:?}", parse("left | x right | _3"));
        assert!(
            sub.contains("Subscript") && sub.contains("Paren") && !sub.contains("base: Empty"),
            "|x|_3 의 _3 가 Paren 그룹에 결합돼야 함: {sub}"
        );

        // 회귀 가드: x^2 는 영향 없음
        let x2 = format!("{:?}", parse("x^2"));
        assert!(x2.contains("Superscript"), "x^2 정상: {x2}");
    }

    #[test]
    fn test_simple_fraction() {
        let ast = parse("1 over 2");
        match &ast {
            EqNode::Fraction { numer, denom } => {
                assert!(matches!(numer.as_ref(), EqNode::Number(n) if n == "1"));
                assert!(matches!(denom.as_ref(), EqNode::Number(n) if n == "2"));
            }
            _ => panic!("Expected Fraction, got {:?}", ast),
        }
    }

    #[test]
    fn test_task1122_over_followed_by_number_parses_as_fraction() {
        for (script, expected_numer, expected_denom) in [
            ("11 over20", "11", "20"),
            ("3 over5", "3", "5"),
            ("7 OVER10", "7", "10"),
            ("{8} over {13}", "8", "13"),
        ] {
            let ast = parse(script);
            match &ast {
                EqNode::Fraction { numer, denom } => {
                    assert!(
                        matches!(numer.as_ref(), EqNode::Number(n) if n == expected_numer),
                        "unexpected numerator for {script:?}: {ast:?}"
                    );
                    assert!(
                        matches!(denom.as_ref(), EqNode::Number(n) if n == expected_denom),
                        "unexpected denominator for {script:?}: {ast:?}"
                    );
                }
                _ => panic!("Expected Fraction for {script:?}, got {:?}", ast),
            }
        }
    }

    #[test]
    fn test_task1122_over_prefix_non_numeric_identifiers_parse_as_text() {
        let ast = parse("overlap");
        assert!(matches!(ast, EqNode::Text(ref t) if t == "overlap"));

        let ast = parse(r"\overline{AB}");
        assert!(
            matches!(
                ast,
                EqNode::Decoration {
                    kind: DecoKind::Overline,
                    ..
                }
            ),
            r"Expected Decoration(Overline), got {:?}",
            ast
        );
    }

    #[test]
    fn test_atop() {
        let ast = parse("a atop b");
        match &ast {
            EqNode::Atop { top, bottom } => {
                assert!(matches!(top.as_ref(), EqNode::Text(t) if t == "a"));
                assert!(matches!(bottom.as_ref(), EqNode::Text(t) if t == "b"));
            }
            _ => panic!("Expected Atop, got {:?}", ast),
        }
    }

    #[test]
    fn test_superscript() {
        let ast = parse("E=mc^2");
        // E = mc^2 → Row([Text("E"), Symbol("="), Superscript(Text("mc"), Number("2"))])
        match &ast {
            EqNode::Row(children) => {
                assert!(children.len() >= 3);
                assert!(matches!(&children[2], EqNode::Superscript { .. }));
            }
            _ => panic!("Expected Row, got {:?}", ast),
        }
    }

    #[test]
    fn test_sqrt() {
        let ast = parse("SQRT x");
        match &ast {
            EqNode::Sqrt { index, body } => {
                assert!(index.is_none());
                assert!(matches!(body.as_ref(), EqNode::Text(t) if t == "x"));
            }
            _ => panic!("Expected Sqrt, got {:?}", ast),
        }
    }

    #[test]
    fn test_sqrt_with_index() {
        let ast = parse("SQRT(3) of x");
        match &ast {
            EqNode::Sqrt { index, body } => {
                assert!(index.is_some());
                assert!(matches!(body.as_ref(), EqNode::Text(t) if t == "x"));
            }
            _ => panic!("Expected Sqrt with index, got {:?}", ast),
        }
    }

    #[test]
    fn test_greek() {
        let ast = parse("alpha + beta");
        match &ast {
            EqNode::Row(children) => {
                assert!(matches!(&children[0], EqNode::MathSymbol(s) if s == "α"));
                assert!(matches!(&children[1], EqNode::Symbol(s) if s == "+"));
                assert!(matches!(&children[2], EqNode::MathSymbol(s) if s == "β"));
            }
            _ => panic!("Expected Row, got {:?}", ast),
        }
    }

    #[test]
    fn test_integral() {
        // 적분은 nolimits: MathSymbol + SubSup (일반 첨자)
        let ast = parse("INT_0^{inf}");
        match &ast {
            EqNode::SubSup { base, sub, sup } => {
                assert!(matches!(base.as_ref(), EqNode::MathSymbol(s) if s == "∫"));
            }
            _ => panic!("Expected SubSup, got {:?}", ast),
        }
    }

    #[test]
    fn test_sum() {
        let ast = parse("SUM_{i=0}^n");
        match &ast {
            EqNode::BigOp { symbol, sub, sup } => {
                assert_eq!(symbol, "∑");
                assert!(sub.is_some());
                assert!(sup.is_some());
            }
            _ => panic!("Expected BigOp, got {:?}", ast),
        }
    }

    #[test]
    fn test_limit() {
        let ast = parse("lim_{x->0}");
        match &ast {
            EqNode::Limit { is_upper, sub } => {
                assert!(!is_upper);
                assert!(sub.is_some());
            }
            _ => panic!("Expected Limit, got {:?}", ast),
        }
    }

    #[test]
    fn test_matrix() {
        let ast = parse("matrix{a & b # c & d}");
        match &ast {
            EqNode::Matrix { rows, style } => {
                assert_eq!(*style, MatrixStyle::Plain);
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0].len(), 2);
                assert_eq!(rows[1].len(), 2);
            }
            _ => panic!("Expected Matrix, got {:?}", ast),
        }
    }

    #[test]
    fn test_left_right() {
        let ast = parse("LEFT ( a over b RIGHT )");
        match &ast {
            EqNode::Paren { left, right, body } => {
                assert_eq!(left, "(");
                assert_eq!(right, ")");
                assert!(matches!(body.as_ref(), EqNode::Fraction { .. }));
            }
            _ => panic!("Expected Paren, got {:?}", ast),
        }
    }

    #[test]
    fn test_decoration() {
        let ast = parse("hat x");
        match &ast {
            EqNode::Decoration { kind, body } => {
                assert_eq!(*kind, DecoKind::Hat);
                assert!(matches!(body.as_ref(), EqNode::Text(t) if t == "x"));
            }
            _ => panic!("Expected Decoration, got {:?}", ast),
        }
    }

    #[test]
    fn test_font_style() {
        let ast = parse("rm abc");
        match &ast {
            EqNode::FontStyle { style, body } => {
                assert_eq!(*style, FontStyleKind::Roman);
            }
            _ => panic!("Expected FontStyle, got {:?}", ast),
        }
    }

    // [#1204-B] 글꼴 명령(rm) body 로 온 decoration(bar)이 Text 로 leak 되지 않고
    // Decoration 으로 파싱되어야 한다.
    #[test]
    fn test_font_style_body_decoration_not_leaked() {
        let ast = parse("rm bar {F prime F}");
        let s = format!("{:?}", ast);
        assert!(
            s.contains("Decoration"),
            "bar 가 Decoration 으로 파싱돼야 함: {s}"
        );
        assert!(
            !s.contains(r#"Text("bar")"#),
            "bar 가 텍스트로 leak 되면 안 됨: {s}"
        );
    }

    // [#1204] 대문자 글꼴/장식 명령(RM/BAR)도 대소문자 무시로 인식 (leak 방지).
    #[test]
    fn test_uppercase_font_and_deco_commands() {
        let s = format!("{:?}", parse("RM {vec{EC}}"));
        assert!(!s.contains(r#"Text("RM")"#), "RM 이 leak 되면 안 됨: {s}");
        assert!(
            s.contains("FontStyle"),
            "RM 이 FontStyle 로 인식돼야 함: {s}"
        );
        let s2 = format!("{:?}", parse("BAR {AB}"));
        assert!(
            s2.contains("Decoration"),
            "BAR 가 Decoration 으로 인식돼야 함: {s2}"
        );
    }

    // [#1204-A] root3 (분리 후 root + 3) 이 Sqrt 로 파싱되어야 한다.
    #[test]
    fn test_root_glued_digit_parses_as_sqrt() {
        let ast = parse("root3 y");
        let s = format!("{:?}", ast);
        assert!(s.contains("Sqrt"), "root3 이 Sqrt 로 파싱돼야 함: {s}");
        assert!(
            !s.contains(r#"Text("root3")"#),
            "root3 이 텍스트로 leak 되면 안 됨: {s}"
        );
    }

    #[test]
    fn test_cases() {
        let ast = parse("CASES{ 1 & x>0 # -1 & x<0 }");
        match &ast {
            EqNode::Cases { rows } => {
                assert_eq!(rows.len(), 2);
            }
            _ => panic!("Expected Cases, got {:?}", ast),
        }
    }

    #[test]
    fn test_sample_eq01_script() {
        // samples/eq-01.hwp의 첫 번째 수식
        let script =
            "평점=입찰가격평가~배점한도 TIMES  LEFT ( {최저입찰가격} over {해당입찰가격} RIGHT )";
        let ast = parse(script);
        // 파싱 실패 없이 AST 생성되면 성공
        match &ast {
            EqNode::Row(children) => {
                assert!(children.len() > 1);
                // TIMES, LEFT-RIGHT 구조 확인
                let has_paren = children.iter().any(|c| matches!(c, EqNode::Paren { .. }));
                assert!(has_paren, "Should contain Paren node");
            }
            _ => {} // 단일 노드도 허용
        }
    }

    #[test]
    fn test_cos_fraction_with_left_right() {
        // cos`left({pi} over {2}+theta right)=`-{1} over {5}`
        // OVER는 바로 앞/뒤 그룹만 분수로 만든다:
        //   LEFT-RIGHT 안: {pi} over {2} → Fraction{π,2}, +θ는 분수 밖
        //   최상위: {1} over {5} → Fraction{1,5}, cos(...)=-는 분수 밖
        let script = " cos ` left({ pi} over {2}+ theta  right)=`-{1} over {5}`";
        let ast = parse(script);
        eprintln!("AST: {:#?}", ast);
        // 최상위는 Row: [cos, Paren{π/2+θ}, =, -, Fraction{1,5}]
        let ast_str = format!("{:?}", ast);
        assert!(ast_str.contains("cos"), "cos가 있어야 함");
        assert!(ast_str.contains("Paren"), "Paren이 있어야 함");
        // Fraction{1,5}가 독립적으로 존재해야 함
        assert!(
            ast_str.contains("Fraction { numer: Number(\"1\"), denom: Number(\"5\")"),
            "Fraction{{1,5}}가 있어야 함: {}",
            ast_str
        );
    }

    #[test]
    fn test_latex_frac() {
        let ast = parse(r"\frac{1}{2}");
        match &ast {
            EqNode::Fraction { numer, denom } => {
                assert!(matches!(numer.as_ref(), EqNode::Number(n) if n == "1"));
                assert!(matches!(denom.as_ref(), EqNode::Number(n) if n == "2"));
            }
            _ => panic!("Expected Fraction, got {:?}", ast),
        }
    }

    #[test]
    fn test_latex_quadratic_slice() {
        let ast = parse(r"x=\frac{-b \pm \sqrt{b^2}}{2a}");
        let ast_str = format!("{:?}", ast);
        assert!(
            ast_str.contains("Fraction"),
            "Fraction이 있어야 함: {}",
            ast_str
        );
        assert!(
            ast_str.contains("MathSymbol(\"±\")"),
            "± 기호가 있어야 함: {}",
            ast_str
        );
        assert!(ast_str.contains("Sqrt"), "Sqrt가 있어야 함: {}", ast_str);
    }

    #[test]
    fn test_latex_sqrt_with_bracket_index() {
        let ast = parse(r"\sqrt[3]{x}");
        match &ast {
            EqNode::Sqrt { index, body } => {
                assert!(index.is_some(), "sqrt index가 있어야 함");
                assert!(matches!(body.as_ref(), EqNode::Text(t) if t == "x"));
            }
            _ => panic!("Expected indexed Sqrt, got {:?}", ast),
        }
    }
}

#[cfg(test)]
#[test]
fn test_lim_fraction() {
    let script = " lim _{h ``rarrow`` 0} {f left(2+h  right)-f left(2  right)} over {h}`";
    let ast = parse(script);
    eprintln!("LIM AST: {:#?}", ast);
    let ast_str = format!("{:?}", ast);
    // lim_{h→0} 가 있어야 함
    assert!(ast_str.contains("Limit"), "Limit가 있어야 함: {}", ast_str);
    // Fraction이 있어야 함
    assert!(
        ast_str.contains("Fraction"),
        "Fraction이 있어야 함: {}",
        ast_str
    );
}

#[cfg(test)]
#[test]
fn test_bar_rm_it() {
    let script = "bar {{rm{AB}} it }< bar {{rm{AC}} it }`";
    let ast = parse(script);
    eprintln!("BAR AST: {:#?}", ast);
    let ast_str = format!("{:?}", ast);
    assert!(ast_str.contains("Decoration"), "Decoration이 있어야 함");
    // }} 가 텍스트로 나오면 안 됨
    assert!(
        !ast_str.contains(r#"Text("}")"#),
        "brace가 텍스트로 나오면 안 됨"
    );
}

#[cfg(test)]
#[test]
fn test_cases_double_amp() {
    let script = "{cases{eqalign{``x^{3}#}&&eqalign{~LEFT(x LEQ 0 RIGHT)#}#``f LEFT(x RIGHT)&&~LEFT(x>0 RIGHT)}}";
    let ast = parse(script);
    eprintln!("CASES AST: {:#?}", ast);
    let s = format!("{:?}", ast);
    assert!(s.contains("Tab"), "Tab이 있어야 함: {}", s);
}

#[cfg(test)]
#[test]
fn test_rm_p_left() {
    let script = "{rm{P}} it  left(A``|` B` right)";
    let ast = parse(script);
    eprintln!("RM_P AST: {:#?}", ast);
    let s = format!("{:?}", ast);
    assert!(s.contains("Paren"), "Paren이 있어야 함: {}", s);
}

// LaTeX 명령어 호환 확장 테스트 (#143 2차)

#[cfg(test)]
#[path = "parser_latex_compat_tests.rs"]
mod latex_compat_tests;
