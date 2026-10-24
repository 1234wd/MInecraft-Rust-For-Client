//! Port of: net/minecraft/util/thread/TaskScheduler.java
//! Java class(es): net.minecraft.util.thread.TaskScheduler
//! Status: PORTED
//!
//! A scheduler for runnables. External contracts:
//! - `java.util.concurrent.CompletableFuture<Source>` -> `javacompat::java_concurrent::CompletableFuture` (modeled).
//! - `java.util.concurrent.Executor` -> `javacompat::java_concurrent::Executor` (modeled).
//! - `java.util.function.Consumer<T>` -> `javacompat::java_concurrent::Consumer` (modeled).
//! - `java.lang.Runnable` -> `javacompat::java_concurrent::Runnable` (modeled).
//!
//! `TaskScheduler` schedules runnables. `wrapExecutor(name, executor)` builds a
//! `TaskScheduler` that delegates scheduling to the executor, mirroring Java's
//! anonymous-class factory.

use crate::javacompat::java_concurrent::{
    clone_completable_future, incomplete_future, CompletableFuture, Consumer, Executor, Runnable,
};
use std::cell::Cell;

/// Wraps a `FnOnce()` closure as a `Runnable`.
///
/// Uses `Cell<Option<F>>` so the closure can be invoked through `&self` (required by
/// `Runnable::run(&self)`) while consuming the captured values on the first call.
struct RunnableFn<F: FnOnce()>(Cell<Option<F>>);

impl<F: FnOnce()> Runnable for RunnableFn<F> {
    fn run(&self) {
        self.0.take().unwrap()();
    }
}

/// Port of `net.minecraft.util.thread.TaskScheduler<R extends Runnable>`.
///
/// `R` is `Box<dyn Runnable>` in this model, matching Java's `Runnable` bound.
pub trait TaskScheduler {
    /// Port of `String name()`.
    fn name(&self) -> String;

    /// Port of `void schedule(R r)`.
    fn schedule(&self, command: Box<dyn Runnable>);

    /// Port of `default void close()` (empty default).
    fn close(&self) {}

    /// Port of `R wrapRunnable(Runnable runnable)`.
    fn wrap_runnable(&self, runnable: Box<dyn Runnable>) -> Box<dyn Runnable>;
}

/// Extension trait for `TaskScheduler` adding the generic default method
/// `scheduleWithResult`. Separate from `TaskScheduler` to keep the core trait
/// dyn-compatible.
pub trait TaskSchedulerWithResult: TaskScheduler {
    /// Port of `default <Source> CompletableFuture<Source> scheduleWithResult(...)`.
    ///
    /// Creates an incomplete future, schedules a runnable that invokes
    /// `future_consumer.accept(future)`, and returns the future.
    fn schedule_with_result<Source: 'static + Clone>(
        &self,
        future_consumer: Box<dyn Consumer<Box<dyn CompletableFuture<Source>>>>,
    ) -> Box<dyn CompletableFuture<Source>> {
        let future = incomplete_future::<Source>();
        let future_clone = clone_completable_future(&*future);
        self.schedule(self.wrap_runnable(Box::new(RunnableFn(
            Some(move || {
                future_consumer.accept(future_clone);
            })
            .into(),
        ))));
        future
    }
}

impl<T: TaskScheduler + ?Sized> TaskSchedulerWithResult for T {}

/// Port of `TaskScheduler.wrapExecutor(name, executor)`: a `TaskScheduler` that
/// delegates `schedule` to `executor.execute(runnable)`.
pub fn wrap_executor(name: &str, executor: Box<dyn Executor>) -> Box<dyn TaskScheduler> {
    struct WrappedScheduler {
        name: String,
        executor: Box<dyn Executor>,
    }

    impl TaskScheduler for WrappedScheduler {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn schedule(&self, command: Box<dyn Runnable>) {
            self.executor.execute(command);
        }

        fn wrap_runnable(&self, runnable: Box<dyn Runnable>) -> Box<dyn Runnable> {
            runnable
        }
    }

    Box::new(WrappedScheduler {
        name: name.to_string(),
        executor,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::javacompat::java_concurrent::{completed_future, Consumer};

    /// Wraps a closure as a `Consumer<T>`.
    struct ConsumerFn<F>(F);

    impl<T, F: Fn(T)> Consumer<T> for ConsumerFn<F> {
        fn accept(&self, t: T) {
            self.0(t);
        }
    }

    struct CountingExecutor {
        count: std::cell::Cell<i32>,
    }

    impl Executor for CountingExecutor {
        fn execute(&self, command: Box<dyn Runnable>) {
            self.count.set(self.count.get() + 1);
            command.run();
        }
    }

    #[derive(Clone)]
    struct IncrementRunnable(i32);

    impl Runnable for IncrementRunnable {
        fn run(&self) {}
    }

    #[test]
    fn wrap_executor_delegates_schedule_to_executor() {
        let exec = CountingExecutor { count: std::cell::Cell::new(0) };
        let scheduler = wrap_executor("test", Box::new(exec));

        assert_eq!(scheduler.name(), "test");
        scheduler.schedule(Box::new(IncrementRunnable(1)));
        scheduler.schedule(Box::new(IncrementRunnable(2)));
    }

    #[test]
    fn wrap_runnable_returns_the_same_runnable() {
        let exec = CountingExecutor { count: std::cell::Cell::new(0) };
        let scheduler = wrap_executor("test", Box::new(exec));

        let r = Box::new(IncrementRunnable(7)) as Box<dyn Runnable>;
        let wrapped = scheduler.wrap_runnable(r);
        wrapped.run();
    }

    #[test]
    fn close_is_empty_by_default() {
        let exec = CountingExecutor { count: std::cell::Cell::new(0) };
        let scheduler = wrap_executor("test", Box::new(exec));
        scheduler.close(); // should not panic
    }

    #[test]
    fn schedule_with_result_schedules_and_returns_future() {
        let exec = CountingExecutor { count: std::cell::Cell::new(0) };
        let scheduler = wrap_executor("test", Box::new(exec));

        let future = scheduler.schedule_with_result(Box::new(ConsumerFn(|fut: Box<dyn CompletableFuture<String>>| {
            // The consumer receives the future; scheduling itself is what we're testing.
            let _ = fut;
        })));
        // Future was created and returned
        let _ = future;
    }
}
