//! Port of: net/minecraft/network/BandwidthDebugMonitor.java
//! Java class(es): net.minecraft.network.BandwidthDebugMonitor
//! Status: VERIFIED
//!
//! Session 24. `public class BandwidthDebugMonitor`: counts the bytes a connection receives and,
//! once per tick, logs the count to the debug screen's bandwidth chart and starts again from 0.
//!
//! ```java
//! private final AtomicInteger bytesReceived = new AtomicInteger();
//! public void onReceive(int bytes) { bytesReceived.getAndAdd(bytes); }
//! public void tick() { bandwidthLogger.logSample(bytesReceived.getAndSet(0)); }
//! ```
//!
//! # Threads and sharing
//!
//! `onReceive` runs on the netty thread and `tick` on the client thread, which is why the count
//! is an `AtomicInteger`; the port uses [`AtomicI32`], whose `fetch_add` wraps on overflow exactly
//! as `getAndAdd` does. The logger is the debug overlay's own `bandwidthLogger`, handed to the
//! connection by reference (`Connection.setBandwidthLogger`), so the overlay reads what this class
//! writes. The port therefore holds it as `Arc<Mutex<LocalSampleLogger>>`, which keeps the
//! monitor `Send + Sync` like the Java object it replaces.
//!
//! The `int` count is widened to `long` by `logSample`, so a count that wrapped negative is logged
//! negative, as in Java.
//!
//! Verified by the `BANDWIDTH` group of `sweep_probe.txt` (`tests/session24.rs`): scripts of
//! receives (overflow included) and ticks, read back through the logger.

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use crate::net::minecraft::util::debugchart::LocalSampleLogger::LocalSampleLogger;
use crate::net::minecraft::util::debugchart::SampleLogger::SampleLogger;

/// Port of `net.minecraft.network.BandwidthDebugMonitor`.
#[derive(Debug)]
pub struct BandwidthDebugMonitor {
    bytes_received: AtomicI32,
    bandwidth_logger: Arc<Mutex<LocalSampleLogger>>,
}

impl BandwidthDebugMonitor {
    /// Port of `public BandwidthDebugMonitor(LocalSampleLogger bandwidthLogger)`.
    pub fn new(bandwidth_logger: Arc<Mutex<LocalSampleLogger>>) -> Self {
        BandwidthDebugMonitor {
            bytes_received: AtomicI32::new(0),
            bandwidth_logger,
        }
    }

    /// Port of `public void onReceive(int bytes)`: `bytesReceived.getAndAdd(bytes)`.
    pub fn on_receive(&self, bytes: i32) {
        self.bytes_received.fetch_add(bytes, Ordering::SeqCst);
    }

    /// Port of `public void tick()`: `bandwidthLogger.logSample(bytesReceived.getAndSet(0))`.
    pub fn tick(&self) {
        let count = self.bytes_received.swap(0, Ordering::SeqCst);
        self.bandwidth_logger
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .log_sample(i64::from(count));
    }
}
