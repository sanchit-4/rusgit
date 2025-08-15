// src/commands/cat_file.rs

use crate::objects::{Blob, GitObject};
use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn cat_file(hash: &str, pretty_print: bool) -> Result<()> {
    // The `-p` flag is what we care about for now.
    if !pretty_print {
        bail!("Only the -p flag is supported for cat-file");
    }

    // 1. Find the object file path from the hash.
    let object_dir = &hash[0..2];
    let object_file = &hash[2..];
    let object_path = Path::new(".git/objects").join(object_dir).join(object_file);

    // 2. Read the compressed content from the object file.
    let compressed_content = fs::read(&object_path)
        .with_context(|| format!("Failed to read object file: {:?}", object_path))?;

    // 3. Decompress the content using zlib.
    let mut decoder = ZlibDecoder::new(&compressed_content[..]);
    let mut decompressed_content = Vec::new();
    decoder.read_to_end(&mut decompressed_content)?;

    // 4. Find the null byte separator.
    let null_byte_pos = decompressed_content
        .iter()
        .position(|&b| b == 0)
        .context("Invalid object format: missing null byte")?;

    // 5. The content is everything after the null byte.
    let content = &decompressed_content[null_byte_pos + 1..];

    // For now, we assume every object is a blob. We'll parse the type later.
    let blob = Blob {
        content: content.to_vec(),
    };
    let git_object = GitObject::Blob(blob);

    // 6. Use our Display implementation to print the object's content.
    print!("{}", git_object);

    Ok(())
}