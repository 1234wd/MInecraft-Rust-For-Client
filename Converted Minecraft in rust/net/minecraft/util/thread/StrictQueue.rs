//! Port of: net/minecraft/util/thread/StrictQueue.java
//! Java class(es): net.minecraft.util.thread.StrictQueue, net.minecraft.util.thread.StrictQueue.FixedPriorityQueue, net.minecraft.util.thread.StrictQueue.QueueStrictQueue, net.minecraft.util.thread.StrictQueue.RunnableWithPriority
//! Status: VERIFIED
//!
//! Session 24. `public interface StrictQueue<T extends Runnable>`: the task queue behind
//! `AbstractConsecutiveExecutor` -- `pop`, `push`, `isEmpty`, `size` -- with its two
//! implementations and the prioritised task record.
//!
//! | Java | here |
//! |---|---|
//! | `interface StrictQueue<T extends Runnable>` | trait [`StrictQueue`] |
//! | `final class FixedPriorityQueue implements StrictQueue<RunnableWithPriority>` | [`FixedPriorityQueue`] |
//! | `final class QueueStrictQueue implements StrictQueue<Runnable>` | [`QueueStrictQueue`] |
//! | `record RunnableWithPriority(int priority, Runnable task) implements Runnable` | [`RunnableWithPriority`] |
//!
//! # Behaviour, with Java's texts
//!
//! * `FixedPriorityQueue(n)` holds `n` FIFO queues; `new Queue[n]` with a negative `n` throws
//!   `NegativeArraySizeException: <n>`. `push` files a task under its priority and throws
//!   `IndexOutOfBoundsException: Priority <p> not supported. Expected range [0-<n-1>]` (formatted
//!   with `Locale.ROOT`, so plain decimals) for a priority outside `0..n`; `pop` takes the oldest
//!   task of the LOWEST-numbered non-empty priority, or nothing. Its `size` is an `AtomicInteger`
//!   kept beside the queues.
//! * `QueueStrictQueue` is a thin wrapper: FIFO `pop`, `push` always `true` (an unbounded
//!   `ConcurrentLinkedQueue`), `isEmpty`, `size`.
//!
//! # Tasks and threads
//!
//! A Java `Runnable` is a [`Task`], `Box<dyn FnMut() + Send>` (as in `TickTask`, session 19). Both
//! queues are shared between the thread that schedules and the executor that drains, so each is
//! `Send + Sync`: a `Mutex<VecDeque<Task>>` per queue stands for `ConcurrentLinkedQueue` (FIFO,
//! non-blocking from the caller's view), and `FixedPriorityQueue` keeps Java's separate
//! `AtomicInteger` count. `pop` returns a [`Task`]; for `FixedPriorityQueue` it is a task that
//! runs the pushed [`RunnableWithPriority`] (whose `run` runs its inner task), as Java's queue
//! hands back the record itself.
//!
//! `QueueStrictQueue(Queue<Runnable> queue)` wraps a queue the caller supplies; its only caller
//! (`ConsecutiveExecutor`) passes a fresh `new ConcurrentLinkedQueue<>()` and keeps no other
//! reference, so the port builds that queue itself in [`QueueStrictQueue::new`].
//!
//! # Not ported
//!
//! `RunnableWithPriority`'s generated `equals`, `hashCode` and `toString` compare and print the
//! `Runnable` -- a lambda -- by identity (OPEN_QUESTIONS #30). No vanilla code compares or prints
//! one.
//!
//! Verified by the `STRICTQ` group of `sweep_probe.txt` (`tests/session24.rs`): push/pop scripts
//! over both queues, the popped tasks run to show which came out, and the error texts.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

/// A Java `Runnable`.
pub type Task = Box<dyn FnMut() + Send>;

/// Port of `net.minecraft.util.thread.StrictQueue<T extends Runnable>`.
pub trait StrictQueue<T> {
    /// Port of `@Nullable Runnable pop()`.
    fn pop(&self) -> Option<Task>;

    /// Port of `boolean push(T t)`.
    fn push(&self, t: T) -> bool;

    /// Port of `boolean isEmpty()`.
    fn is_empty(&self) -> bool;

    /// Port of `int size()`.
    fn size(&self) -> i32;
}

/// Port of `record StrictQueue.RunnableWithPriority(int priority, Runnable task) implements Runnable`.
pub struct RunnableWithPriority {
    priority: i32,
    task: Task,
}

impl RunnableWithPriority {
    /// Port of the canonical constructor.
    pub fn new(priority: i32, task: Task) -> Self {
        RunnableWithPriority { priority, task }
    }

    /// Port of the record accessor `int priority()`.
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Port of the record accessor `Runnable task()`.
    pub fn task(&mut self) -> &mut Task {
        &mut self.task
    }

    /// Port of `public void run()`: `this.task.run()`.
    pub fn run(&mut self) {
        (self.task)();
    }
}

/// Locks a queue, carrying on past a poisoned lock as a `ConcurrentLinkedQueue` would.
fn lock(q: &Mutex<VecDeque<Task>>) -> std::sync::MutexGuard<'_, VecDeque<Task>> {
    q.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Port of `StrictQueue.FixedPriorityQueue`.
pub struct FixedPriorityQueue {
    queues: Vec<Mutex<VecDeque<Task>>>,
    size: AtomicI32,
}

impl FixedPriorityQueue {
    /// Port of `public FixedPriorityQueue(int size)`: one empty queue per priority.
    pub fn new(size: i32) -> Self {
        if size < 0 {
            panic!("java.lang.NegativeArraySizeException: {size}");
        }
        FixedPriorityQueue {
            queues: (0..size).map(|_| Mutex::new(VecDeque::new())).collect(),
            size: AtomicI32::new(0),
        }
    }
}

impl StrictQueue<RunnableWithPriority> for FixedPriorityQueue {
    /// Port of `pop()`: the first task of the lowest-numbered non-empty queue.
    fn pop(&self) -> Option<Task> {
        for queue in &self.queues {
            if let Some(task) = lock(queue).pop_front() {
                self.size.fetch_sub(1, Ordering::SeqCst);
                return Some(task);
            }
        }
        None
    }

    /// Port of `push(RunnableWithPriority task)`.
    fn push(&self, task: RunnableWithPriority) -> bool {
        let priority = task.priority;
        if priority < self.queues.len() as i32 && priority >= 0 {
            let mut task = task;
            lock(&self.queues[priority as usize]).push_back(Box::new(move || task.run()));
            self.size.fetch_add(1, Ordering::SeqCst);
            true
        } else {
            panic!(
                "java.lang.IndexOutOfBoundsException: Priority {priority} not supported. Expected range [0-{}]",
                self.queues.len() as i32 - 1
            );
        }
    }

    /// Port of `isEmpty()`: `size.get() == 0`.
    fn is_empty(&self) -> bool {
        self.size.load(Ordering::SeqCst) == 0
    }

    /// Port of `size()`.
    fn size(&self) -> i32 {
        self.size.load(Ordering::SeqCst)
    }
}

/// Port of `StrictQueue.QueueStrictQueue`.
pub struct QueueStrictQueue {
    queue: Mutex<VecDeque<Task>>,
}

impl Default for QueueStrictQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueStrictQueue {
    /// Port of `public QueueStrictQueue(Queue<Runnable> queue)` over a fresh
    /// `ConcurrentLinkedQueue`, the one queue vanilla passes it.
    pub fn new() -> Self {
        QueueStrictQueue {
            queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl StrictQueue<Task> for QueueStrictQueue {
    /// Port of `pop()`: `queue.poll()`.
    fn pop(&self) -> Option<Task> {
        lock(&self.queue).pop_front()
    }

    /// Port of `push(Runnable t)`: `queue.add(t)`, always `true` for an unbounded queue.
    fn push(&self, t: Task) -> bool {
        lock(&self.queue).push_back(t);
        true
    }

    /// Port of `isEmpty()`.
    fn is_empty(&self) -> bool {
        lock(&self.queue).is_empty()
    }

    /// Port of `size()`.
    fn size(&self) -> i32 {
        lock(&self.queue).len() as i32
    }
}
