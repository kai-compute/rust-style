use super::*;

#[test]
fn ordered_partitions_preserve_text_and_modular_summaries() {
    let input = [("a", u64::MAX), ("", 2), ("b", 3)];
    let summarize = |items: &[(&str, u64)]| -> (Text, ModularSum) {
        fold_map(items, |&(text, number)| {
            (Text(text.to_owned()), ModularSum(Wrapping(number)))
        })
    };
    for split in 0..=input.len() {
        assert_eq!(
            summarize(&input[..split]).combine(summarize(&input[split..])),
            summarize(&input),
        );
    }
    assert_ne!(
        Text("a".into()).combine(Text("b".into())),
        Text("b".into()).combine(Text("a".into()))
    );
}

#[test]
fn checked_signed_and_floating_addition_have_regrouping_counterexamples() {
    let left = i8::MAX.checked_add(1).and_then(|sum| sum.checked_add(-1));
    let right = 1_i8
        .checked_add(-1)
        .and_then(|sum| i8::MAX.checked_add(sum));
    assert_eq!(left, None);
    assert_eq!(right, Some(i8::MAX));
    let (a, b, c) = (1e16_f64, -1e16_f64, 1.0_f64);
    assert_eq!((a + b) + c, 1.0);
    assert_eq!(a + (b + c), 0.0);
}
