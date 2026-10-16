//! Port of: net/minecraft/ExitCodes.java
//! Java class(es): net.minecraft.ExitCodes
//! Status: VERIFIED
//!
//! Process exit codes passed to `System.exit` by the crash/shutdown paths. A pure constant
//! holder: all eight `public static final int` fields are checked against the jar's constant
//! dump by `_porting/tests/session19_e.rs`. Every code is negative, so on POSIX the process
//! status the OS reports is the low byte (`-1` -> `255`).

/// Java: `ExitCodes.EXIT_CODE_CRASH_DEFAULT` = `-1`.
pub const EXIT_CODE_CRASH_DEFAULT: i32 = -1;
/// Java: `ExitCodes.EXIT_CODE_CRASH_WITHOUT_REPORT` = `-2`.
pub const EXIT_CODE_CRASH_WITHOUT_REPORT: i32 = -2;
/// Java: `ExitCodes.EXIT_CODE_CRASH_EARLY_NO_MODULES` = `-3`.
pub const EXIT_CODE_CRASH_EARLY_NO_MODULES: i32 = -3;
/// Java: `ExitCodes.EXIT_CODE_CRASH_EARLY_ARGUMENT_LIBRARY_LOAD` = `-4`.
pub const EXIT_CODE_CRASH_EARLY_ARGUMENT_LIBRARY_LOAD: i32 = -4;
/// Java: `ExitCodes.EXIT_CODE_CRASH_EARLY_ARGUMENT_PARSE` = `-5`.
pub const EXIT_CODE_CRASH_EARLY_ARGUMENT_PARSE: i32 = -5;
/// Java: `ExitCodes.EXIT_CODE_CRASH_SHUTDOWN` = `-6`.
pub const EXIT_CODE_CRASH_SHUTDOWN: i32 = -6;
/// Java: `ExitCodes.EXIT_CODE_VERSION_PARSING_FAIL` = `-7`.
pub const EXIT_CODE_VERSION_PARSING_FAIL: i32 = -7;
/// Java: `ExitCodes.EXIT_CODE_CLIENT_WATCHDOG` = `-8`.
pub const EXIT_CODE_CLIENT_WATCHDOG: i32 = -8;
