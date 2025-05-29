use core::task::{ Context, Poll };
use core::pin::Pin;
use core::time::Duration;
use std::time::Instant;
use pin_project::pin_project;


/// A `Future` which times out at a given point in time.
#[pin_project]
pub struct Timeout<F>
where
    F : Future
{
    #[pin]
    fut     : F,
    timeout : Instant
}

impl<F> Future for Timeout<F>
where
    F : Future
{

    type Output = Result<<F as Future>::Output, TimeoutError>;

    fn poll(self : Pin<&mut Self>, ctx : &mut Context<'_>) -> Poll<Self::Output> {
        let self1 = self.project();
        match (self1.fut.poll(ctx)) {
            Poll::Ready(out) => Poll::Ready(Ok(out)),
            Poll::Pending    => {
                if (Instant::now() >= *self1.timeout) {
                    Poll::Ready(Err(TimeoutError))
                } else { Poll::Pending }
            },
        }
    }

}


/// A [`Timeout`] `Future` failed to complete in time.
pub struct TimeoutError;


/// Wrap a `Future` in [`Timeout`], cancelling it after the given duration.
pub fn timeout<F>(dur : Duration, fut : F) -> Timeout<F>
where
    F : Future
{ Timeout { fut, timeout : Instant::now() + dur } }

/// Wrap a `Future` in [`Timeout`], cancelling it at the given point in time.
pub fn timeout_at<F>(timeout : Instant, fut : F) -> Timeout<F>
where
    F : Future
{ Timeout { fut, timeout } }
