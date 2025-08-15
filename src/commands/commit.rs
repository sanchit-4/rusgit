// src/commands/commit.rs

use crate::index::Index;
use crate::repository;
use anyhow::Result;
use std::path::Path;

pub fn commit(message: String) -> Result<()> {
    let git_dir = Path::new(".git");
    let index_path = git_dir.join("index");

    // 1. Read the index.
    let index = Index::from_path(&index_path)?;
    if index.entries.is_empty() {
        println!("Nothing to commit, working tree clean");
        return Ok(());
    }

    // 2. Get the parent commit hash from HEAD.
    let parent_hash = repository::get_head_commit_hash(&git_dir)?;

    // 3. Write a tree object from the index.
    // We will reuse the write_tree logic, but in a real app, this would be a
    // dedicated function `write_tree_from_index`. For now, `write_tree` from the
    // filesystem is a close enough approximation if `add` was just run.
    let tree_hash_bytes = crate::commands::write_tree::write_tree(&Path::new("."))?;
    let tree_hash = hex::encode(tree_hash_bytes);

    // 4. Create the commit object, using our plumbing command's logic.
    let commit_hash_bytes =
        crate::commands::commit_tree::commit_tree(tree_hash, parent_hash, message)?;
    let commit_hash = hex::encode(commit_hash_bytes);

    // 5. Update the current branch (HEAD) to point to the new commit.
    repository::update_head(&git_dir, &commit_hash)?;

    println!("Committed to main with hash {}", commit_hash);

    // Optional: Clear the index after a successful commit.
    // A real Git implementation does this.
    let _ = std::fs::remove_file(&index_path);

    Ok(())
}