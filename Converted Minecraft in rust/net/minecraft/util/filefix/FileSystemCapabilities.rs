//! Port of: net/minecraft/util/filefix/FileSystemCapabilities.java
//! Java class(es): net.minecraft.util.filefix.FileSystemCapabilities
//! Status: VERIFIED

use crate::javacompat::java_nio::{CopyOptionValue, LinkOption, StandardCopyOption};
use crate::javacompat::java_object::{hash_bool, record_hash};

/// Java record `FileSystemCapabilities(boolean atomicMove, boolean hardLinks)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileSystemCapabilities {
    atomic_move: bool,
    hard_links: bool,
}

impl FileSystemCapabilities {
    /// The canonical record constructor.
    pub const fn new(atomic_move: bool, hard_links: bool) -> Self {
        Self {
            atomic_move,
            hard_links,
        }
    }

    /// Java record accessor `atomicMove()`.
    pub const fn atomic_move(self) -> bool {
        self.atomic_move
    }

    /// Java record accessor `hardLinks()`.
    pub const fn hard_links(self) -> bool {
        self.hard_links
    }

    /// Returns the JDK options used for a move. Java allocates a new array on each call.
    pub fn get_move_options(self) -> Vec<CopyOptionValue> {
        if self.atomic_move {
            vec![
                StandardCopyOption::ATOMIC_MOVE.into(),
                LinkOption::NOFOLLOW_LINKS.into(),
            ]
        } else {
            vec![LinkOption::NOFOLLOW_LINKS.into()]
        }
    }

    /// Java's generated record `hashCode`, folding component hashes from zero.
    pub fn hash_code(self) -> i32 {
        record_hash(&[hash_bool(self.atomic_move), hash_bool(self.hard_links)])
    }
}

impl std::fmt::Display for FileSystemCapabilities {
    /// Java's generated record `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FileSystemCapabilities[atomicMove={}, hardLinks={}]",
            self.atomic_move, self.hard_links
        )
    }
}
