# Full-repository audit scan

Source of truth: `git ls-files --cached` (14824 tracked paths).
Repository root: `C:\Users\Desktop-JuniorBlade\Documents\Converted rust Minecraft For Client`.

## Totals

- Tracked paths: **14824**
- Total bytes: **353,143,232**
- Total lines: **4,337,196**
- Scan status: all `14824` paths scanned; exceptions: 0

## Manifest status (Java sources)

PARTIAL=83 | PORTED=55 | SKELETON=6587 | VERIFIED=330

## File-category counts

- build_lockfile: **1**
- build_metadata: **1**
- generated: **13**
- java_oracle: **20**
- java_source: **7,056**
- javacompat: **26**
- porting_doc: **4**
- porting_manifest: **1**
- porting_other: **1**
- porting_test: **62**
- porting_tool: **9**
- root_file: **9**
- rust_crate_root: **1**
- rust_mirror: **7,055**
- rust_mod: **551**
- rust_other: **2**
- test_data: **12**

## Untracked / user files (not part of scan, recorded for awareness)

- Converted Minecraft in rust/_porting/generated/full_repo_audit.md
- Converted Minecraft in rust/_porting/generated/full_repo_scan.csv
- Converted Minecraft in rust/_porting/tools/audit_scan.py

## Notes

- Scan method: `read+sha256` for every readable tracked file; `stat-only` only if unreadable.
- `manifest_status` is read from `_porting/MANIFEST.csv` (the authority, derived from `//! Status:` headers by `mirror.py`).
- `rust_mirror` is the same relative path with `.java` -> `.rs`.
- `generated/` files are produced by `mirror.py`, `small_inventory.py`, `gen_argb_tables.py`, `gen_tables.py`.
- Java 25 oracle regeneration is NOT RUN locally (JDK is 21; the Minecraft jar is not tracked); committed golden data is used instead.
