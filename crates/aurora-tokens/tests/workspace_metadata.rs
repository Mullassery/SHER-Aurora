//! Guard against workspace metadata and the README's stated project status
//! drifting apart.
//!
//! `aurora-core`, `aurora-qt`, and `aurora-web` are intentional placeholder
//! crates with no real implementation (each says so in its own `lib.rs`).
//! This test pins three facts so they cannot silently change:
//!
//! 1. each placeholder is still a workspace member (so `-p aurora-qt` works),
//! 2. each is *excluded* from `default-members` (so a bare `cargo build` /
//!    `cargo test` does not compile the no-op crates), and
//! 3. the README documents them as placeholders.

use std::fs;
use std::path::PathBuf;

const PLACEHOLDERS: [&str; 3] = ["aurora-core", "aurora-qt", "aurora-web"];

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is <root>/crates/aurora-tokens.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("crate is two levels below the workspace root")
        .to_path_buf()
}

#[test]
fn placeholder_crates_are_members_but_not_default_members() {
    let manifest = fs::read_to_string(workspace_root().join("Cargo.toml"))
        .expect("workspace Cargo.toml must be readable");

    // Isolate the `default-members = [ ... ]` array contents. The explanatory
    // comment above it mentions the crate names, but that comment precedes the
    // key, so the slice after `default-members` starts at the key itself.
    let default_members = manifest
        .split("default-members")
        .nth(1)
        .expect("workspace must declare default-members")
        .split(']')
        .next()
        .unwrap();

    for name in PLACEHOLDERS {
        assert!(
            manifest.contains(&format!("\"crates/{name}\"")),
            "{name} must remain a workspace member"
        );
        assert!(
            !default_members.contains(name),
            "{name} must be excluded from default-members"
        );
    }
}

#[test]
fn readme_documents_placeholder_crates() {
    let readme = fs::read_to_string(workspace_root().join("README.md"))
        .expect("README.md must be readable")
        .to_lowercase();

    assert!(
        readme.contains("placeholder"),
        "README must state that some crates are placeholders"
    );
    for name in PLACEHOLDERS {
        assert!(
            readme.contains(name),
            "README must mention the placeholder crate {name}"
        );
    }
}
