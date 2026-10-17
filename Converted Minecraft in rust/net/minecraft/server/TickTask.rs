//! Port of: net/minecraft/server/TickTask.java
//! Java class(es): net.minecraft.server.TickTask
//! Status: VERIFIED
//!
//! Session 22. `public class TickTask implements Runnable`: a task tagged with the server tick it
//! was scheduled on. `run()` simply runs the wrapped `Runnable`, once per call.
//!
//! The `Runnable` becomes a boxed `FnMut() + Send` -- tasks are handed to the server thread from
//! other threads -- and `run` therefore takes `&mut self`.
//!
//! Verified by the `TICKTASK` group of `small_probe.txt` (`tests/session22.rs`): the tick for five
//! values (both `int` extremes), and two `run()` calls running the runnable exactly twice.

/// Port of `net.minecraft.server.TickTask`.
pub struct TickTask {
    tick: i32,
    runnable: Box<dyn FnMut() + Send>,
}

impl TickTask {
    /// Port of `new TickTask(int tick, Runnable runnable)`.
    pub fn new(tick: i32, runnable: impl FnMut() + Send + 'static) -> Self {
        TickTask {
            tick,
            runnable: Box::new(runnable),
        }
    }

    /// Port of `getTick()`.
    pub const fn get_tick(&self) -> i32 {
        self.tick
    }

    /// Port of `run()`: runs the wrapped runnable.
    pub fn run(&mut self) {
        (self.runnable)();
    }
}

impl std::fmt::Debug for TickTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickTask").field("tick", &self.tick).finish_non_exhaustive()
    }
}
