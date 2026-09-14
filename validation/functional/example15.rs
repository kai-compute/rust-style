use super::*;

#[test]
fn successful_left_choice_does_not_evaluate_the_alternative() {
    let alternative = Parser::new(|_| -> ParseResult<'_, ()> {
        panic!("a successful left branch must suppress the alternative")
    });
    assert_eq!(literal("a").or(alternative).parse_all("a"), Ok(()));
}

#[test]
fn farther_diagnostics_keep_the_attempted_right_branches_commitment() {
    let left = literal("abc").and_then(|_| literal("x")).attempt();
    let right = literal("a").and_then(|_| literal("z"));
    let parser = left.or(right).or(literal("abc?"));
    assert_eq!(
        parser.parse_all("abc?"),
        Err(ParseError {
            offset: 3,
            expected: "x",
            committed: true
        }),
    );
}

#[test]
fn unicode_offsets_and_repetition_boundaries_are_explicit() {
    assert_eq!(
        literal("\u{03bb}")
            .and_then(|_| literal("!"))
            .parse_all("\u{03bb}?"),
        Err(ParseError {
            offset: 2,
            expected: "!",
            committed: true
        }),
    );
    assert_eq!(literal("a").repeat_at_most(0).parse_all(""), Ok(vec![]));
    assert_eq!(
        literal("").repeat_at_most(1).parse_all(""),
        Err(ParseError {
            offset: 0,
            expected: "input progress",
            committed: true
        }),
    );
    assert_eq!(
        literal("a")
            .repeat_at_most(1)
            .parse_all("aa")
            .unwrap_err()
            .offset,
        1,
    );
}
