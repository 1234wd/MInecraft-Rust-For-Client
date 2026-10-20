//! The `jdk.jfr.consumer.RecordedEvent` field subset read by Minecraft's JFR statistics records.
//!
//! This value helper models named String, int, float, and timespan fields plus the event start
//! instant consumed by Minecraft's current small stat records. It does not implement recording
//! or the rest of the JFR consumer API.

use crate::javacompat::java_time::{Duration, Instant};
use crate::javacompat::throwable::{JavaThrowable, ThrowableRef};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum FieldValue {
    String(Option<String>),
    Int(i32),
    Float(f32),
    TimespanNanoseconds(i64),
}

/// A typed field snapshot consumed by the JFR stat record `from` methods.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordedEvent {
    fields: BTreeMap<String, FieldValue>,
    start_time: Instant,
}

impl RecordedEvent {
    pub fn new(start_time: Instant) -> Self {
        Self {
            fields: BTreeMap::new(),
            start_time,
        }
    }

    pub fn insert_string(&mut self, name: impl Into<String>, value: Option<String>) {
        self.fields.insert(name.into(), FieldValue::String(value));
    }

    pub fn insert_int(&mut self, name: impl Into<String>, value: i32) {
        self.fields.insert(name.into(), FieldValue::Int(value));
    }

    pub fn insert_float(&mut self, name: impl Into<String>, value: f32) {
        self.fields.insert(name.into(), FieldValue::Float(value));
    }

    /// Insert a JFR `long @Timespan(NANOSECONDS)` field, as used by
    /// `ServerTickTimeEvent.averageTickDuration`.
    pub fn insert_duration_nanos(&mut self, name: impl Into<String>, value: i64) {
        self.fields
            .insert(name.into(), FieldValue::TimespanNanoseconds(value));
    }

    /// Port of `RecordedEvent.getStartTime()` for the event snapshot.
    pub fn get_start_time(&self) -> Instant {
        self.start_time
    }

    pub fn get_string(&self, name: Option<&str>) -> Result<Option<String>, ThrowableRef> {
        let value = self.value(name)?;
        match value {
            FieldValue::String(value) => Ok(value.clone()),
            FieldValue::Int(_) => Err(illegal_argument(format!(
                "Attempt to get int field \"{}\" with illegal data type conversion java.lang.String",
                name.unwrap_or("null")
            ))),
            FieldValue::Float(_) => Err(illegal_argument(format!(
                "Attempt to get float field \"{}\" with illegal data type conversion java.lang.String",
                name.unwrap_or("null")
            ))),
            FieldValue::TimespanNanoseconds(_) => Err(illegal_argument(format!(
                "Attempt to get field \"{}\" with illegal data type conversion java.lang.String",
                name.unwrap_or("null")
            ))),
        }
    }

    pub fn get_int(&self, name: Option<&str>) -> Result<i32, ThrowableRef> {
        let value = self.value(name)?;
        match value {
            FieldValue::Int(value) => Ok(*value),
            _ => Err(illegal_argument(format!(
                "Attempt to get field \"{}\" with illegal data type conversion int",
                name.unwrap_or("null")
            ))),
        }
    }

    pub fn get_float(&self, name: Option<&str>) -> Result<f32, ThrowableRef> {
        let value = self.value(name)?;
        match value {
            FieldValue::Float(value) => Ok(*value),
            _ => Err(illegal_argument(format!(
                "Attempt to get field \"{}\" with illegal data type conversion float",
                name.unwrap_or("null")
            ))),
        }
    }

    /// Port of JDK 25 `RecordedObject.getDuration(String)` for the nanosecond timespan fields
    /// used by `TickTimeStat`. Missing and wrong-type field errors follow the JDK messages.
    pub fn get_duration(&self, name: Option<&str>) -> Result<Duration, ThrowableRef> {
        let value = self.value(name)?;
        match value {
            FieldValue::TimespanNanoseconds(value) => Ok(Duration::from_jfr_nanoseconds(*value)),
            FieldValue::Int(0) => Ok(Duration::of_nanos(0)),
            FieldValue::Int(_) => Err(illegal_argument(format!(
                "Attempt to get int field \"{}\" with missing @Timespan",
                name.unwrap_or("null")
            ))),
            _ => Err(illegal_argument(format!(
                "Attempt to get field \"{}\" with illegal data type conversion java.time.Duration",
                name.unwrap_or("null")
            ))),
        }
    }

    fn value(&self, name: Option<&str>) -> Result<&FieldValue, ThrowableRef> {
        let Some(name) = name else {
            return Err(JavaThrowable::new(
                "java.lang.NullPointerException",
                Some("name".to_string()),
            ));
        };
        self.fields
            .get(name)
            .ok_or_else(|| illegal_argument(format!("Could not find field with name {name}")))
    }
}

/// The helpful NPE emitted when a Minecraft `from(event)` method dereferences a null event.
pub fn null_event(method: &str) -> ThrowableRef {
    JavaThrowable::new(
        "java.lang.NullPointerException",
        Some(format!(
            "Cannot invoke \"jdk.jfr.consumer.RecordedEvent.{method}(String)\" because \"event\" is null"
        )),
    )
}

/// Helpful NPE for a null `RecordedEvent` whose first dereference is a no-argument method.
pub fn null_event_no_args(method: &str) -> ThrowableRef {
    JavaThrowable::new(
        "java.lang.NullPointerException",
        Some(format!(
            "Cannot invoke \"jdk.jfr.consumer.RecordedEvent.{method}()\" because \"event\" is null"
        )),
    )
}

fn illegal_argument(message: String) -> ThrowableRef {
    JavaThrowable::new("java.lang.IllegalArgumentException", Some(message))
}
