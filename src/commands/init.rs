// src/commands/init.rs

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

pub fn init(path: &Path) -> Result<()> {
    // 1. Create the main `.git` directory.
    let git_dir = path.join(".git");
    fs::create_dir(&git_dir)
        .with_context(|| format!("Failed to create directory at: {:?}", git_dir))?;

    // 2. Create the `objects` directory.
    let objects_dir = git_dir.join("objects");
    fs::create_dir(&objects_dir)
        .with_context(|| format!("Failed to create directory at: {:?}", objects_dir))?;

    // 3. Create the `refs` directory and its subdirectory `heads`.
    let refs_dir = git_dir.join("refs");
    fs::create_dir(&refs_dir)
        .with_context(|| format!("Failed to create directory at: {:?}", refs_dir))?;
    
    let heads_dir = refs_dir.join("heads");
    fs::create_dir(&heads_dir)
        .with_context(|| format!("Failed to create directory at: {:?}", heads_dir))?;

    // 4. Create the `HEAD` file.
    let head_file_path = git_dir.join("HEAD");
    // This file points to the default branch, which doesn't exist yet, but we
    // set it up to point to where `main` will be.
    fs::write(&head_file_path, "ref: refs/heads/main\n")
        .with_context(|| format!("Failed to write to HEAD file at: {:?}", head_file_path))?;
    
    println!("Initialized empty Rusgit repository in {:?}", git_dir);

    Ok(())
}