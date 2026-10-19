//! Port of: net/minecraft/CrashReportDetail.java
//! Java class(es): net.minecraft.CrashReportDetail
//! Status: VERIFIED

use crate::javacompat::java_concurrent::Callable;

/// Java's generic interface, which adds no members beyond `Callable<V>`.
pub trait CrashReportDetail<V>: Callable<V> {}
