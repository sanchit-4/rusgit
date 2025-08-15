use crate::objects::{Commit, GitObject};
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn commit_tree(
    tree_hash: String,
    parent_hash: Option<String>,
    message: String,
) -> Result<[u8; 20]> {
    // 1. For now, we'll hardcode the author and committer info.
    // In real Git, this comes from the user's config.
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    // A typical git author/committer string is "Name <email> timestamp timezone"
    let author = format!("Rusgit Author <author@example.com> {} +0000", now);
    let committer = format!("Rusgit Committer <committer@example.com> {} +0000", now);

    // 2. The `parents` vector will be empty for the first commit, or contain the parent hash.
    let mut parents = Vec::new();
    if let Some(p_hash) = parent_hash {
        parents.push(p_hash);
    }

    // 3. Create the Commit struct.
    let commit = Commit {
        tree_hash,
        parents,
        author,
        committer,
        message,
    };

    // 4. Serialize the commit object to bytes.
    let commit_content = commit.as_bytes();

    // 5. Hash the serialized commit content to get the commit's hash.
    let header = format!("commit {}\0", commit_content.len());
    let mut full_content = header.as_bytes().to_vec();
    full_content.extend_from_slice(&commit_content);

    let mut hasher = Sha1::new();
    hasher.update(&full_content);
    let hash: [u8; 20] = hasher.finalize().into();

    // 6. Write the new commit object to the database (same logic as before).
    let hash_hex = hex::encode(hash);
    let object_dir = Path::new(".git/objects").join(&hash_hex[0..2]);
    fs::create_dir_all(&object_dir)?;
    let object_path = object_dir.join(&hash_hex[2..]);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&full_content)?;
    let compressed_bytes = encoder.finish()?;
    fs::write(&object_path, compressed_bytes)?;

    // 7. Print the hash of the commit we just created.
    println!("{}", hash_hex);
    Ok(hash)
}