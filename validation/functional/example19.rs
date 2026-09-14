use super::*;

#[test]
fn applicative_identity_and_associativity_preserve_ordered_errors() {
    let cases = [
        Validation::Valid(3),
        Validation::Invalid(Errors::one("a")),
        Validation::Invalid(Errors::one("b")),
    ];
    for a in &cases {
        assert_eq!(Validation::Valid(()).map2(a.clone(), |(), value| value), *a);
        assert_eq!(a.clone().map2(Validation::Valid(()), |value, ()| value), *a);
        for b in &cases {
            for c in &cases {
                let left = a
                    .clone()
                    .map2(b.clone(), |x, y| (x, y))
                    .map2(c.clone(), |xy, z| (xy, z));
                let right = a
                    .clone()
                    .map2(b.clone().map2(c.clone(), |y, z| (y, z)), |x, yz| (x, yz))
                    .map(|(x, (y, z))| ((x, y), z));
                assert_eq!(left, right);
            }
        }
    }
}

#[test]
fn traversal_accumulates_every_error_in_input_order() {
    let result = traverse_validation(["first", "second", "third"], |value| {
        Validation::<(), _>::Invalid(Errors::one(value))
    });
    assert_eq!(
        result,
        Validation::Invalid(Errors {
            first: "first",
            rest: vec!["second", "third"]
        })
    );
    assert_eq!(
        traverse_validation(Vec::<u8>::new(), Validation::<_, &str>::Valid),
        Validation::Valid(vec![])
    );
}

#[test]
fn fusing_validation_phases_can_change_the_first_error() {
    let first = |value| {
        if value == 2 {
            Err("first-phase")
        } else {
            Ok(value)
        }
    };
    let second = |value| {
        if value == 1 {
            Err("second-phase")
        } else {
            Ok(value)
        }
    };
    let phased: Result<Vec<_>, _> = [1, 2]
        .into_iter()
        .map(first)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|values| values.into_iter().map(second).collect());
    let fused: Result<Vec<_>, _> = [1, 2]
        .into_iter()
        .map(|value| first(value).and_then(second))
        .collect();
    assert_eq!(phased, Err("first-phase"));
    assert_eq!(fused, Err("second-phase"));
}
