// src/commands/add.rs

use crate::index::Index;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use sha1::{Digest, Sha1};

pub fn add(files: Vec<PathBuf>) -> Result<()> {
    let index_path = PathBuf::from(".git/index");
    let mut index = Index::from_path(&index_path)?;

    for file_path in files {
        // 1. Run the `hash-object` logic to get the blob hash.
        let content = fs::read(&file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;
        
        let header = format!("blob {}\0", content.len());
        let mut full_content = header.as_bytes().to_vec();
        full_content.extend_from_slice(&content);
        
        let hash: [u8; 20] = sha1::Sha1::digest(&full_content).into();

        // 2. Write the blob object to the database (if it doesn't already exist).
        // Note: For simplicity, we are not checking for existence first.
        // A real implementation would.
        let _ = crate::commands::hash_object::hash_object(&file_path, true)?;

        // 3. Add the file to the index.
        // We'll use a standard file mode. `0o` prefix indicates octal.
        index.add(file_path, hash, 0o100644);
        println!("Added file to index.");
    }

    // 4. Write the updated index back to disk.
    index.write(&index_path)?;

    Ok(())
}