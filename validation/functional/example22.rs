use std::cell::Cell;
use std::pin::Pin;
use std::rc::Rc;

use super::*;

struct PendingOnce {
    waited: bool,
    value: u32,
}

impl Future for PendingOnce {
    type Output = Result<u32, &'static str>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.waited {
            Poll::Ready(Ok(self.value))
        } else {
            self.waited = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[test]
fn pending_stages_preserve_order_and_call_the_continuation_once() {
    let calls = Cell::new(0);
    let future = and_then_async(
        PendingOnce {
            waited: false,
            value: 7,
        },
        |value| {
            calls.set(calls.get() + 1);
            PendingOnce {
                waited: false,
                value: value * 2,
            }
        },
    );
    let waker = Waker::from(Arc::new(ReadyFixtureWake));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(calls.get(), 0);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(calls.get(), 1);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(Ok(14)));
    assert_eq!(calls.get(), 1);
}

struct PendingResource(Rc<Cell<bool>>);

impl Future for PendingResource {
    type Output = Result<u32, &'static str>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for PendingResource {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

#[test]
fn cancellation_drops_the_owned_pending_input_without_running_the_next_stage() {
    let dropped = Rc::new(Cell::new(false));
    let calls = Cell::new(0);
    let future = and_then_async(PendingResource(Rc::clone(&dropped)), |value| {
        calls.set(calls.get() + 1);
        ready(Ok(value))
    });
    let waker = Waker::from(Arc::new(ReadyFixtureWake));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    drop(future);
    assert!(dropped.get());
    assert_eq!(calls.get(), 0);
}
