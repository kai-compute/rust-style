use std::cell::Cell;

use super::*;

struct Payload(Rc<Cell<usize>>);

impl Drop for Payload {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn cloned_versions_share_non_clone_payloads_until_the_last_owner_drops() {
    let dropped = Rc::new(Cell::new(0));
    let stack = PersistentStack::new().prepend(Payload(Rc::clone(&dropped)));
    let snapshot = stack.clone();
    assert!(std::ptr::eq(
        stack.first().unwrap(),
        snapshot.first().unwrap()
    ));
    drop(stack);
    assert_eq!(dropped.get(), 0);
    drop(snapshot);
    assert_eq!(dropped.get(), 1);
}

#[test]
fn a_long_unique_chain_is_reclaimed_without_recursive_node_destruction() {
    let dropped = Rc::new(Cell::new(0));
    let mut stack = PersistentStack::new();
    for _ in 0..100_000 {
        stack = stack.prepend(Payload(Rc::clone(&dropped)));
    }
    assert_eq!(stack.iter().count(), 100_000);
    drop(stack);
    assert_eq!(dropped.get(), 100_000);
}
