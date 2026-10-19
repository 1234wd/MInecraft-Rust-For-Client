//! Port of: net/minecraft/util/ExceptionCollector.java
//! Java class(es): net.minecraft.util.ExceptionCollector
//! Status: VERIFIED

use crate::javacompat::throwable::{JavaThrowable, ThrowableRef};

/// Retains the first throwable and attaches every later one as suppressed.
#[derive(Default)]
pub struct ExceptionCollector {
    result: Option<ThrowableRef>,
}

impl ExceptionCollector {
    /// Java's implicit public no-argument constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// `add(Throwable)`: Java accepts null while empty; with a primary value, null and self-add
    /// follow `Throwable.addSuppressed` and return its exception as an error.
    pub fn add(&mut self, throwable: Option<ThrowableRef>) -> Result<(), ThrowableRef> {
        if self.result.is_none() {
            self.result = throwable;
            Ok(())
        } else {
            JavaThrowable::add_suppressed(
                self.result.as_ref().expect("result is present"),
                throwable,
            )
        }
    }

    /// `throwIfPresent()`: `Err` carries the original throwable object reference unchanged.
    pub fn throw_if_present(&self) -> Result<(), ThrowableRef> {
        self.result
            .as_ref()
            .map_or(Ok(()), |result| Err(result.clone()))
    }
}
