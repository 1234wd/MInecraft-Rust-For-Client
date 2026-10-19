//! Port of: net/minecraft/commands/execution/Frame.java
//! Java class(es): net.minecraft.commands.execution.Frame, net.minecraft.commands.execution.Frame.FrameControl
//! Status: PARTIAL
//!
//! Session 24. `public record Frame(int depth, CommandResultCallback returnValueConsumer,
//! Frame.FrameControl frameControl)`: one level of the command execution stack. Three methods
//! that delegate, and a nested `@FunctionalInterface FrameControl { void discard(); }`.
//!
//! # Shared references
//!
//! Both object components are SHARED in Java: `ExecutionContext` builds a frame around the
//! caller's callback and a `FrameControl` lambda that captures the context itself, and
//! `CallFunction`/`IsolatedCall` hand one callback to several frames. The port holds them as
//! `Arc<dyn ...>`, as [`CommandResultCallback`] already is (session 19), and `FrameControl` has the
//! same `Send + Sync` bound so the two can live in the same queues.
//!
//! # Record members
//!
//! * the accessors and the three methods are ported and probed;
//! * `equals` is the record's: `depth` by value, the two object components by `Object.equals` --
//!   identity, since neither a lambda nor `CommandResultCallback.EMPTY` (an anonymous class)
//!   overrides it. [`Arc::ptr_eq`] is that comparison;
//! * `hashCode` and `toString` fold in the components' IDENTITY hashes (a lambda's `toString` is
//!   `...$$Lambda/0x...@<hash>`), so they differ on every JVM run and are not ported, per
//!   OPEN_QUESTIONS #30.
//!
//! Verified by the `FRAME` and `FRAMEEQ` groups of `sweep_probe.txt` (`tests/session24.rs`): the
//! callback sequence each method produces, the accessors, and the `equals` matrix.

use std::sync::Arc;

use crate::net::minecraft::commands::CommandResultCallback::CommandResultCallback;

/// Port of `Frame.FrameControl`, a `@FunctionalInterface`.
pub trait FrameControl: Send + Sync {
    /// Port of `void discard()`.
    fn discard(&self);
}

/// A Java lambda or method reference implementing `FrameControl`.
impl<F: Fn() + Send + Sync> FrameControl for F {
    fn discard(&self) {
        self()
    }
}

/// Port of `net.minecraft.commands.execution.Frame`.
#[derive(Clone)]
pub struct Frame {
    depth: i32,
    return_value_consumer: Arc<dyn CommandResultCallback>,
    frame_control: Arc<dyn FrameControl>,
}

impl Frame {
    /// Port of the canonical constructor `Frame(int, CommandResultCallback, Frame.FrameControl)`.
    pub fn new(
        depth: i32,
        return_value_consumer: Arc<dyn CommandResultCallback>,
        frame_control: Arc<dyn FrameControl>,
    ) -> Self {
        Frame {
            depth,
            return_value_consumer,
            frame_control,
        }
    }

    /// Port of the record accessor `int depth()`.
    pub fn depth(&self) -> i32 {
        self.depth
    }

    /// Port of the record accessor `CommandResultCallback returnValueConsumer()`.
    pub fn return_value_consumer(&self) -> &Arc<dyn CommandResultCallback> {
        &self.return_value_consumer
    }

    /// Port of the record accessor `Frame.FrameControl frameControl()`.
    pub fn frame_control(&self) -> &Arc<dyn FrameControl> {
        &self.frame_control
    }

    /// Port of `public void returnSuccess(int value)`: `returnValueConsumer.onSuccess(value)`.
    pub fn return_success(&self, value: i32) {
        self.return_value_consumer.on_success(value);
    }

    /// Port of `public void returnFailure()`: `returnValueConsumer.onFailure()`.
    pub fn return_failure(&self) {
        self.return_value_consumer.on_failure();
    }

    /// Port of `public void discard()`: `frameControl.discard()`.
    pub fn discard(&self) {
        self.frame_control.discard();
    }
}

impl PartialEq for Frame {
    /// Port of the record's `equals`: `depth` by value, the callback and the frame control by
    /// reference (neither overrides `Object.equals`).
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
            && Arc::ptr_eq(&self.return_value_consumer, &other.return_value_consumer)
            && Arc::ptr_eq(&self.frame_control, &other.frame_control)
    }
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Frame")
            .field("depth", &self.depth)
            .field(
                "return_value_consumer",
                &Arc::as_ptr(&self.return_value_consumer),
            )
            .field("frame_control", &Arc::as_ptr(&self.frame_control))
            .finish()
    }
}
