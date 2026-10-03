//! Java standard-library compatibility layer.
//!
//! NOT game code. These modules have no `.java` counterpart in
//! `minecraft-decompiled/`, so per Rule 3 they live in `_porting/javacompat/`
//! rather than in the mirror tree. `lib.rs` pulls them in with `#[path]`.
//!
//! See _porting/DESIGN_DECISIONS.md (#javacompat) for why each one exists and what
//! it would break if we used the Rust equivalent instead.

pub mod golden;
pub mod java_lang;
pub mod java_random;
pub mod md5;