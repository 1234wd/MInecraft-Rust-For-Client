//! Narrow `java.util.concurrent` interfaces used by Minecraft.

use crate::javacompat::throwable::ThrowableRef;

/// The Java contract `Callable<V>`, with checked and unchecked throwables represented as errors.
pub trait Callable<V> {
    fn call(&self) -> Result<V, ThrowableRef>;
}
