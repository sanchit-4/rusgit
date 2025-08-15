// src/repository.rs

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

/// Reads the .git/HEAD file to find the hash of the latest commit.
pub fn get_head_commit_hash(git_dir: &Path) -> Result<Option<String>> {
    let head_path = git_dir.join("HEAD");
    if !head_path.exists() {
        // No HEAD means no commits yet. This is a valid state for the first commit.
        return Ok(None);
    }

    let head_content = fs::read_to_string(&head_path)?
        .trim().to_string();

    // The content of HEAD should be "ref: refs/heads/main"
    if let Some(ref_path_str) = head_content.strip_prefix("ref: ") {
        let ref_path = git_dir.join(ref_path_str);
        if ref_path.exists() {
            // Read the actual hash from the branch file (e.g., .git/refs/heads/main)
            let commit_hash = fs::read_to_string(ref_path)?.trim().to_string();
            Ok(Some(commit_hash))
        } else {
            // The branch file doesn't exist yet, so there are no commits.
            Ok(None)
        }
    } else {
        // This would be a "detached HEAD" state, which we won't support for now.
        bail!("Detached HEAD state is not supported");
    }
}

/// Updates the current branch file to point to a new commit hash.
pub fn update_head(git_dir: &Path, commit_hash: &str) -> Result<()> {
    let head_path = git_dir.join("HEAD");
    let head_content = fs::read_to_string(&head_path)?.trim().to_string();

    if let Some(ref_path_str) = head_content.strip_prefix("ref: ") {
        let ref_path = git_dir.join(ref_path_str);
        fs::write(ref_path, commit_hash.as_bytes())
            .context("Failed to update branch reference")?;
        Ok(())
    } else {
        bail!("Cannot update in detached HEAD state");
    }
}