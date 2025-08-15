// src/commands/hash_object.rs

use crate::objects::{Blob, GitObject}; // Use the object types we just defined
use anyhow::{Context, Result};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn hash_object(file_path: &Path, write: bool) -> Result<()> {
    // 1. Read the content of the file into a byte vector.
    let content = fs::read(file_path)
        .with_context(|| format!("Failed to read file: {:?}", file_path))?;

    // 2. Construct the blob object.
    let blob = Blob { content };
    let git_object = GitObject::Blob(blob);

    // 3. Format the object content for hashing.
    // The format is: `type <size>\0content`
    let object_content = format!(
        "blob {}\0{}",
        git_object.to_string().len(),
        git_object
    );
    let object_bytes = object_content.as_bytes();

    // 4. Calculate the SHA-1 hash of the formatted content.
    let mut hasher = Sha1::new();
    hasher.update(object_bytes);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    // 5. If the `-w` flag is present, write the object to the database.
    if write {
        // The path is `.git/objects/<first-2-hash-chars>/<remaining-hash-chars>`
        let object_dir = Path::new(".git/objects").join(&hash_hex[0..2]);
        fs::create_dir_all(&object_dir)
            .with_context(|| format!("Failed to create object directory: {:?}", object_dir))?;

        let object_path = object_dir.join(&hash_hex[2..]);

        // 6. Compress the content using zlib and write to the object file.
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(object_bytes)?;
        let compressed_bytes = encoder.finish()?;

        fs::write(&object_path, compressed_bytes)
            .with_context(|| format!("Failed to write object file: {:?}", object_path))?;
    }

    // 7. Print the calculated hash to standard output.
    println!("{}", hash_hex);

    Ok(())
}