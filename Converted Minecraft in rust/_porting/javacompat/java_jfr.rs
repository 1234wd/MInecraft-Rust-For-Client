//! The `jdk.jfr.consumer.RecordedEvent` field subset read by Minecraft's JFR statistics records.
//!
//! This value helper models named String, int, and float fields and the JDK 25 getter failures;
//! it does not implement recording, event timing, or the rest of the JFR consumer API.

use crate::javacompat::throwable::{JavaThrowable, ThrowableRef};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum FieldValue {
    String(Option<String>),
    Int(i32),
    Float(f32),
}

/// A typed field snapshot consumed by the JFR stat record `from` methods.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecordedEvent {
    fields: BTreeMap<String, FieldValue>,
}

impl RecordedEvent {
    pub fn new() -> Self {
        Self::default()
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

fn illegal_argument(message: String) -> ThrowableRef {
    JavaThrowable::new("java.lang.IllegalArgumentException", Some(message))
}
