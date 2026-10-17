//! Sub-batch A: the exception classes. Groups `EXC`, `EXCFS`, `EXCSTACK`, `BCE`, `BCREASON`.
//!
//! Each `EXC` row is one Java construction: `id`, the index into the probe's message pool (or -1
//! for a constructor with no message), the index into its cause pool (or -1 for a constructor
//! with no cause), then what Java reported. The test makes the same construction in Rust and
//! compares every reported field.

use crate::small_probe::*;

use minecraft_rust::javacompat::throwable::Cause;

use minecraft_rust::com::mojang::blaze3d::systems::BackendCreationException::{
    self as bce_mod, BackendCreationException, Reason,
};
use minecraft_rust::com::mojang::blaze3d::systems::SurfaceException::{self as surface_mod, SurfaceException};
use minecraft_rust::com::mojang::blaze3d::vulkan::glsl::ShaderCompileException::{
    self as shader_mod, ShaderCompileException,
};
use minecraft_rust::com::mojang::blaze3d::GpuDeviceLossException::{self as gdl_mod, GpuDeviceLossException};
use minecraft_rust::com::mojang::blaze3d::GpuOutOfMemoryException::{self as gom_mod, GpuOutOfMemoryException};
use minecraft_rust::com::mojang::realmsclient::exception::RealmsHttpException::{
    self as realms_mod, RealmsHttpException,
};
use minecraft_rust::net::minecraft::client::main::SilentInitException::{self as silent_mod, SilentInitException};
use minecraft_rust::net::minecraft::server::jsonrpc::methods::EncodeJsonRpcException::{
    self as enc_mod, EncodeJsonRpcException,
};
use minecraft_rust::net::minecraft::server::jsonrpc::methods::InvalidParameterJsonRpcException::{
    self as ip_mod, InvalidParameterJsonRpcException,
};
use minecraft_rust::net::minecraft::server::jsonrpc::methods::InvalidRequestJsonRpcException::{
    self as ir_mod, InvalidRequestJsonRpcException,
};
use minecraft_rust::net::minecraft::server::jsonrpc::methods::MethodNotFoundJsonRpcException::{
    self as mnf_mod, MethodNotFoundJsonRpcException,
};
use minecraft_rust::net::minecraft::server::RunningOnDifferentThreadException::{
    self as rodt_mod, RunningOnDifferentThreadException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSDirectoryNotEmptyException::{
    self as dne_mod, CowFSDirectoryNotEmptyException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSFileAlreadyExistsException::{
    self as fae_mod, CowFSFileAlreadyExistsException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSFileSystemException::{
    self as fse_mod, CowFSFileSystemException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSIllegalArgumentException::{
    self as iae_mod, CowFSIllegalArgumentException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSNoSuchFileException::{
    self as nsf_mod, CowFSNoSuchFileException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSNotDirectoryException::{
    self as nd_mod, CowFSNotDirectoryException,
};
use minecraft_rust::net::minecraft::util::CryptException::{self as crypt_mod, CryptException};

/// The probe's `MSGS`, in order.
const MSGS: [Option<&str>; 5] = [Some("boom"), Some(""), None, Some("tab\there"), Some("\u{e9}\u{4e2d}\u{1F600}")];

/// A Java cause, rebuilt from the `toString()` the probe recorded for it -- which is the only thing
/// a cause contributes to the outer exception's message.
#[derive(Debug)]
struct JavaCause(String);

impl std::fmt::Display for JavaCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for JavaCause {}

fn cause_from(to_string: Option<String>) -> Option<Cause> {
    to_string.map(|s| Box::new(JavaCause(s)) as Cause)
}

/// What a Rust exception reports, in the probe's field order.
struct Got {
    class_name: &'static str,
    superclass: &'static str,
    message: Option<String>,
    to_string: String,
    cause: Option<String>,
}

fn got<E: std::fmt::Display>(
    class_name: &'static str,
    superclass: &'static str,
    e: &E,
    message: Option<&str>,
    cause: Option<&Cause>,
) -> Got {
    Got {
        class_name,
        superclass,
        message: message.map(str::to_string),
        to_string: e.to_string(),
        cause: cause.map(|c| c.to_string()),
    }
}

fn build(id: &str, msg: i32, cause_idx: i32, cause: Option<Cause>) -> Got {
    let m = if msg >= 0 { MSGS[msg as usize] } else { None };
    match (id, cause_idx >= 0) {
        ("GpuDeviceLossException", false) => {
            let e = GpuDeviceLossException::new(m);
            got(gdl_mod::CLASS_NAME, gdl_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("GpuOutOfMemoryException", false) => {
            let e = GpuOutOfMemoryException::new(m);
            got(gom_mod::CLASS_NAME, gom_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("ShaderCompileException", false) => {
            let e = ShaderCompileException::new(m);
            got(shader_mod::CLASS_NAME, shader_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("EncodeJsonRpcException", false) => {
            let e = EncodeJsonRpcException::new(m);
            got(enc_mod::CLASS_NAME, enc_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("InvalidParameterJsonRpcException", false) => {
            let e = InvalidParameterJsonRpcException::new(m);
            got(ip_mod::CLASS_NAME, ip_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("InvalidRequestJsonRpcException", false) => {
            let e = InvalidRequestJsonRpcException::new(m);
            got(ir_mod::CLASS_NAME, ir_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("MethodNotFoundJsonRpcException", false) => {
            let e = MethodNotFoundJsonRpcException::new(m);
            got(mnf_mod::CLASS_NAME, mnf_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("CowFSIllegalArgumentException", false) => {
            let e = CowFSIllegalArgumentException::new(m);
            got(iae_mod::CLASS_NAME, iae_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("CowFSFileSystemException", false) => {
            let e = CowFSFileSystemException::new(m);
            got(fse_mod::CLASS_NAME, fse_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("CowFSDirectoryNotEmptyException", false) => {
            let e = CowFSDirectoryNotEmptyException::new(m);
            got(dne_mod::CLASS_NAME, dne_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("CowFSFileAlreadyExistsException", false) => {
            let e = CowFSFileAlreadyExistsException::new(m);
            got(fae_mod::CLASS_NAME, fae_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("CowFSNoSuchFileException", false) => {
            let e = CowFSNoSuchFileException::new(m);
            got(nsf_mod::CLASS_NAME, nsf_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("CowFSNotDirectoryException", false) => {
            let e = CowFSNotDirectoryException::new(m);
            got(nd_mod::CLASS_NAME, nd_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("SilentInitException", false) => {
            let e = SilentInitException::new(m);
            got(silent_mod::CLASS_NAME, silent_mod::SUPERCLASS, &e, e.get_message(), e.get_cause())
        }
        ("SilentInitException", true) => {
            let e = SilentInitException::with_cause(m, cause);
            got(silent_mod::CLASS_NAME, silent_mod::SUPERCLASS, &e, e.get_message(), e.get_cause())
        }
        ("RealmsHttpException", true) => {
            let e = RealmsHttpException::new(m, cause);
            got(realms_mod::CLASS_NAME, realms_mod::SUPERCLASS, &e, e.get_message(), e.get_cause())
        }
        ("SurfaceException", false) => {
            let e = SurfaceException::new(m);
            got(surface_mod::CLASS_NAME, surface_mod::SUPERCLASS, &e, e.get_message(), e.get_cause())
        }
        ("SurfaceException", true) => {
            let e = SurfaceException::from_cause(cause);
            got(surface_mod::CLASS_NAME, surface_mod::SUPERCLASS, &e, e.get_message(), e.get_cause())
        }
        ("CryptException", true) => {
            let e = CryptException::new(cause);
            got(crypt_mod::CLASS_NAME, crypt_mod::SUPERCLASS, &e, e.get_message(), e.get_cause())
        }
        ("RunningOnDifferentThreadException", false) => {
            let e = RunningOnDifferentThreadException::RUNNING_ON_DIFFERENT_THREAD;
            got(rodt_mod::CLASS_NAME, rodt_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        // The probe's two BackendCreationException constructions: message 0 through the
        // three-argument constructor, message 2 (null) through the two-argument one.
        ("BackendCreationException", false) if msg == 0 => {
            let e = BackendCreationException::new(m, Reason::VulkanNoDevice, &["a".to_string(), "b".to_string()]);
            got(bce_mod::CLASS_NAME, bce_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        ("BackendCreationException", false) if msg == 2 => {
            let e = BackendCreationException::new_without_capabilities(m, Reason::Other);
            got(bce_mod::CLASS_NAME, bce_mod::SUPERCLASS, &e, e.get_message(), None)
        }
        other => panic!("no Rust construction for probe case {other:?} (msg {msg}, cause {cause_idx})"),
    }
}

/// `instanceof RuntimeException`, decided from the superclass the port records.
fn is_unchecked(superclass: &str) -> bool {
    matches!(superclass, "java.lang.RuntimeException" | "java.lang.IllegalArgumentException")
}

#[test]
fn every_exception_construction_matches_the_jar() {
    let rows = rows("EXC");
    let mut ids = std::collections::BTreeSet::new();
    for r in &rows {
        let (id, msg, cause_idx) = (r[1], i32f(r[2]), i32f(r[3]));
        let cause = if cause_idx >= 0 { cause_from(unesc(r[9])) } else { None };
        let g = build(id, msg, cause_idx, cause);
        let ctx = format!("{id} msg={msg} cause={cause_idx}");
        assert_eq!(g.class_name, r[4], "{ctx}: class name");
        assert_eq!(g.superclass, r[5], "{ctx}: superclass");
        assert_eq!(is_unchecked(g.superclass), boolf(r[6]), "{ctx}: instanceof RuntimeException");
        assert_eq!(g.message, unesc(r[7]), "{ctx}: getMessage()");
        assert_eq!(Some(g.to_string), unesc(r[8]), "{ctx}: toString()");
        assert_eq!(g.cause, unesc(r[9]), "{ctx}: getCause().toString()");
        ids.insert(id);
    }
    // 19 classes, and the probe exercised each one.
    assert_eq!(ids.len(), 19, "EXC covers {ids:?}");
    assert_eq!(rows.len(), 126, "EXC row count changed; update this test with the probe");
}

fn owned(file: Option<&str>, other: Option<&str>, reason: Option<&str>) -> (Option<String>, Option<String>, Option<String>) {
    (file.map(str::to_string), other.map(str::to_string), reason.map(str::to_string))
}

#[test]
fn file_system_exceptions_store_their_argument_as_the_file() {
    for r in rows("EXCFS") {
        let m = MSGS[i32f(r[2]) as usize];
        let (file, other, reason) = match r[1] {
            "CowFSFileSystemException" => {
                let e = CowFSFileSystemException::new(m);
                owned(e.get_file(), e.get_other_file(), e.get_reason())
            }
            "CowFSDirectoryNotEmptyException" => {
                let e = CowFSDirectoryNotEmptyException::new(m);
                owned(e.get_file(), e.get_other_file(), e.get_reason())
            }
            "CowFSFileAlreadyExistsException" => {
                let e = CowFSFileAlreadyExistsException::new(m);
                owned(e.get_file(), e.get_other_file(), e.get_reason())
            }
            "CowFSNoSuchFileException" => {
                let e = CowFSNoSuchFileException::new(m);
                owned(e.get_file(), e.get_other_file(), e.get_reason())
            }
            "CowFSNotDirectoryException" => {
                let e = CowFSNotDirectoryException::new(m);
                owned(e.get_file(), e.get_other_file(), e.get_reason())
            }
            other => panic!("unexpected EXCFS id {other}"),
        };
        assert_eq!(file, unesc(r[3]), "{}: getFile()", r[1]);
        assert_eq!(other, unesc(r[4]), "{}: getOtherFile()", r[1]);
        assert_eq!(reason, unesc(r[5]), "{}: getReason()", r[1]);
    }
}

#[test]
fn running_on_different_thread_has_no_stack_trace_and_fills_in_to_itself() {
    let r = &rows("EXCSTACK")[0];
    let e = RunningOnDifferentThreadException::RUNNING_ON_DIFFERENT_THREAD;
    assert_eq!(r[1], "RunningOnDifferentThreadException");
    assert_eq!(e.get_stack_trace_len(), i32f(r[2]));
    assert_eq!(e.fill_in_stack_trace().get_stack_trace_len(), i32f(r[3]));
    assert_eq!(std::ptr::eq(e.fill_in_stack_trace(), &e), boolf(r[4]));
}

#[test]
fn backend_creation_reason_constants_match_the_jar() {
    let rows = rows("BCREASON");
    assert_eq!(rows.len(), Reason::VALUES.len(), "a Reason constant was added or dropped");
    for r in rows {
        let ordinal = i32f(r[2]);
        let reason = Reason::VALUES[ordinal as usize];
        assert_eq!(reason.name(), r[1], "ordinal {ordinal}");
        assert_eq!(reason.ordinal(), ordinal);
        assert_eq!(Some(reason.display_name().to_string()), unesc(r[3]), "{}", r[1]);
        assert_eq!(reason.to_string(), r[1], "{}: toString is name()", r[1]);
    }
}

#[test]
fn backend_creation_copies_its_capability_list() {
    let rows = rows("BCE");
    let find = |case: &str| rows.iter().find(|r| r[1] == case).unwrap_or_else(|| panic!("no BCE `{case}` row"));

    // The probe builds the exception from ["a", "b"], then adds "c" to the SOURCE list.
    let mut caps = vec!["a".to_string(), "b".to_string()];
    let three = BackendCreationException::new(MSGS[0], Reason::VulkanNoDevice, &caps);
    caps.push("c".to_string());
    let r = find("three-arg");
    assert_eq!(three.get_reason().name(), r[2]);
    assert_eq!(three.get_missing_capabilities().join(","), r[3]);
    // Java's list is immutable (`add` throws); a `&[String]` cannot be added to at all.
    assert!(boolf(r[4]), "the jar's capability list is expected to be immutable");

    let two = BackendCreationException::new_without_capabilities(MSGS[2], Reason::Other);
    let r = find("two-arg");
    assert_eq!(two.get_reason().name(), r[2]);
    assert_eq!(two.get_missing_capabilities().join(","), r[3]);
    assert_eq!(two.get_missing_capabilities().is_empty(), boolf(r[4]));

    // `List.copyOf` rejects a null element. A Rust `String` cannot be null, so there is nothing
    // to port; this pins that the jar really does throw, which is why the Rust type forbids it.
    assert_eq!(find("null-element")[2], "java.lang.NullPointerException");
}

#[test]
fn throw_panics_with_the_java_to_string() {
    let text = |f: fn()| {
        let err = std::panic::catch_unwind(f).expect_err("throw() must panic");
        err.downcast_ref::<String>().cloned().expect("panic payload is a String")
    };
    assert_eq!(text(|| gdl_mod::throw("x")), "com.mojang.blaze3d.GpuDeviceLossException: x");
    assert_eq!(text(|| nsf_mod::throw("f")), "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSNoSuchFileException: f");
    assert_eq!(text(|| rodt_mod::throw()), "net.minecraft.server.RunningOnDifferentThreadException");
    assert_eq!(
        text(|| crypt_mod::throw(Box::new(JavaCause("java.lang.RuntimeException: inner".into())))),
        "net.minecraft.util.CryptException: java.lang.RuntimeException: inner"
    );
    assert_eq!(
        text(|| bce_mod::throw("m", Reason::Other, &[])),
        "com.mojang.blaze3d.systems.BackendCreationException: m"
    );
}
