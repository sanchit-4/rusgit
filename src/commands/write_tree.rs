// src/commands/write_tree.rs

use crate::objects::{Blob, GitObject, Tree, TreeEntry};
use anyhow::{Context, Result};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn write_tree(path: &Path) -> Result<[u8; 20]> {
    let mut entries = Vec::new();

    // 1. Iterate over the files and directories in the given path.
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("Invalid UTF-8 in file name"))?;

        // 2. Skip the .git directory itself.
        if entry_name == ".git" {
            continue;
        }

        let file_type = entry.file_type()?;
        let hash = if file_type.is_dir() {
            // 3. If it's a directory, recursively call write_tree.
            write_tree(&entry_path)?
        } else if file_type.is_file() {
            // 4. If it's a file, hash it as a blob object.
            let content = fs::read(&entry_path)?;
            let blob = Blob { content };
            let git_object = GitObject::Blob(blob);

            // This is very similar to hash-object's logic.
            let object_content = format!("blob {}\0{}", git_object.to_string().len(), git_object);
            let object_bytes = object_content.as_bytes();
            
            let mut hasher = Sha1::new();
            hasher.update(object_bytes);
            hasher.finalize().into() // Convert GenericArray to [u8; 20]
        } else {
            // We don't handle symlinks or other file types.
            continue;
        };

        // 5. Determine the file mode. For simplicity, we'll use 100644 for files
        // and 040000 for directories.
        let mode = if file_type.is_dir() { "40000" } else { "100644" };
        entries.push(TreeEntry {
            mode: mode.to_string(),
            name: entry_name,
            hash,
        });
    }

    // 6. Sort the entries by name, as required by Git.
    entries.sort();

    // 7. Create the Tree object and serialize it.
    let tree = Tree { entries };
    let tree_content = tree.as_bytes();

    // 8. Hash the serialized tree content to get the tree's hash.
    let header = format!("tree {}\0", tree_content.len());
    let mut full_content = header.as_bytes().to_vec();
    full_content.extend_from_slice(&tree_content);

    let mut hasher = Sha1::new();
    hasher.update(&full_content);
    let hash: [u8; 20] = hasher.finalize().into();

    // 9. Write the new tree object to the database.
    let hash_hex = hex::encode(hash);
    let object_dir = Path::new(".git/objects").join(&hash_hex[0..2]);
    fs::create_dir_all(&object_dir)?;
    let object_path = object_dir.join(&hash_hex[2..]);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&full_content)?;
    let compressed_bytes = encoder.finish()?;
    fs::write(&object_path, compressed_bytes)?;

    // 10. Return the hash of the tree we just created.
    Ok(hash)
}