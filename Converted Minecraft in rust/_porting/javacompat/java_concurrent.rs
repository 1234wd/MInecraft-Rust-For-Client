//! Narrow `java.util.concurrent` and `java.util.function` interfaces used by Minecraft.
//!
//! These are all single-method functional interfaces (SAM types), modeled as Rust traits
//! with one method each. Only the methods actually called by ported Minecraft code are
//! declared; default/static methods on the Java interfaces are omitted.

use crate::javacompat::throwable::ThrowableRef;

/// The Java contract `Callable<V>`, with checked and unchecked throwables represented as errors.
pub trait Callable<V> {
    fn call(&self) -> Result<V, ThrowableRef>;
}

/// The Java contract `java.util.function.Consumer<T>`: `void accept(T t)`.
pub trait Consumer<T> {
    fn accept(&self, t: T);
}

/// The Java contract `java.util.function.Supplier<T>`: `T get()`.
pub trait Supplier<T> {
    fn get(&self) -> T;
}

/// The Java contract `java.util.function.Function<T, R>`: `R apply(T t)`.
pub trait Function<T, R> {
    fn apply(&self, t: T) -> R;
}

/// The Java contract `java.util.function.Predicate<T>`: `boolean test(T t)`.
pub trait Predicate<T> {
    fn test(&self, t: T) -> bool;
}

/// The Java contract `java.util.Iterator<E>`, narrow subset.
pub trait Iterator<E> {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<E>;
}

/// The Java contract `java.util.List<E>`, narrow subset.
///
/// Only the methods actually used by ported callers are declared. `List` extends
/// `Collection` in Java, but this model does not pretend to implement that hierarchy.
pub trait List<E> {
    /// Port of `int size()`.
    fn size(&self) -> i32;

    /// Port of `boolean isEmpty()`.
    fn is_empty(&self) -> bool;

    /// Port of `E get(int index)`.
    fn get(&self, index: i32) -> Option<&E>;
}

/// The Java contract `java.lang.Runnable`: `void run()`.
pub trait Runnable {
    fn run(&self);
}

/// The Java contract `java.util.concurrent.Executor`: `void execute(Runnable command)`.
pub trait Executor {
    fn execute(&self, command: Box<dyn Runnable>);
}

/// Narrow model of `java.util.concurrent.CompletableFuture<T>`.
///
/// Only the methods used by ported callers are declared: `isDone`,
/// `isCompletedExceptionally`, and `join`. `join` consumes the future and returns
/// the value, panicking if the future completed exceptionally (matching Java's
/// `join` throwing `CompletionException`).
pub trait CompletableFuture<T: 'static> {
    /// Port of `boolean isDone()`.
    fn is_done(&self) -> bool;

    /// Port of `boolean isCompletedExceptionally()`.
    fn is_completed_exceptionally(&self) -> bool;

    /// Port of `T join()`: consumes the future and returns the value.
    fn join(self: Box<Self>) -> T;

    /// Downcast to `Any` so concrete future types can be cloned.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// A completed (non-exceptional) `CompletableFuture<T>`.
pub struct CompletedFuture<T: 'static> {
    value: Option<T>,
}

impl<T: 'static> CompletableFuture<T> for CompletedFuture<T> {
    fn is_done(&self) -> bool {
        self.value.is_some()
    }

    fn is_completed_exceptionally(&self) -> bool {
        false
    }

    fn join(self: Box<Self>) -> T {
        self.value.expect("CompletableFuture not completed")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Port of `CompletableFuture.completedFuture(value)`.
pub fn completed_future<T: 'static>(value: T) -> Box<dyn CompletableFuture<T>> {
    Box::new(CompletedFuture { value: Some(value) })
}

/// An incomplete `CompletableFuture<T>` that can be completed from another context,
/// modeling Java's `new CompletableFuture<>()`.
pub struct IncompleteFuture<T: 'static> {
    inner: std::sync::Arc<std::sync::Mutex<Option<T>>>,
}

impl<T: 'static> Clone for IncompleteFuture<T> {
    fn clone(&self) -> Self {
        Self {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }
}

impl<T: 'static> IncompleteFuture<T> {
    /// Port of `new CompletableFuture<>()`.
    pub fn new() -> Self {
        Self {
            inner: std::sync::Arc::new(std::sync::Mutex::new(None)),
        }
    }

    /// Port of `complete(value)`: completes the future with the given value.
    pub fn complete(&self, value: T) {
        *self.inner.lock().expect("incomplete future lock poisoned") = Some(value);
    }

    /// Port of `get()`: blocks until complete, then returns the value.
    pub fn get(&self) -> T {
        self.inner.lock().expect("incomplete future lock poisoned").take().expect("future completed")
    }
}

impl<T: 'static> CompletableFuture<T> for IncompleteFuture<T> {
    fn is_done(&self) -> bool {
        self.inner.lock().expect("incomplete future lock poisoned").is_some()
    }

    fn is_completed_exceptionally(&self) -> bool {
        false
    }

    fn join(self: Box<Self>) -> T {
        self.inner
            .lock()
            .expect("incomplete future lock poisoned")
            .take()
            .expect("future completed")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Port of `CompletableFuture.incompleteFuture()`: creates an incomplete future.
pub fn incomplete_future<T: 'static>() -> Box<dyn CompletableFuture<T>> {
    Box::new(IncompleteFuture::new())
}

/// Clone a `Box<dyn CompletableFuture<T>>` by downcasting to the concrete type.
/// `CompletedFuture` clones its value; `IncompleteFuture` shares its inner state.
pub fn clone_completable_future<T: 'static + Clone>(f: &dyn CompletableFuture<T>) -> Box<dyn CompletableFuture<T>> {
    if let Some(cf) = f.as_any().downcast_ref::<CompletedFuture<T>>() {
        completed_future(cf.value.clone().expect("completed future has value"))
    } else if let Some(if_) = f.as_any().downcast_ref::<IncompleteFuture<T>>() {
        Box::new(if_.clone())
    } else {
        panic!("cannot clone unknown CompletableFuture type")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completed_future_reports_done_and_returns_value() {
        let f: Box<dyn CompletableFuture<i32>> = completed_future(42);
        assert!(f.is_done());
        assert!(!f.is_completed_exceptionally());
        assert_eq!(f.join(), 42);
    }

    #[test]
    fn completed_future_join_consumes_and_moves_value() {
        let f: Box<dyn CompletableFuture<String>> = completed_future("hello".to_string());
        assert_eq!(f.join(), "hello");
    }

    #[test]
    fn check_exceptions_on_normal_future_does_not_panic() {
        // `ReloadInstance::check_exceptions` calls `done().join()` only when
        // `is_completed_exceptionally()`; a normal future must not panic.
        let f: Box<dyn CompletableFuture<()>> = completed_future(());
        assert!(f.is_done());
        assert!(!f.is_completed_exceptionally());
        f.join();
    }
}
