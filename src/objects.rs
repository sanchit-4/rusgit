// src/objects.rs

use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeEntry {
    // We use a string for mode for simplicity. Git uses octal.
    pub mode: String,
    pub name: String,
    // The hash is stored as raw bytes for efficiency.
    pub hash: [u8; 20],
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub tree_hash: String,
    // A commit can have zero or more parents.
    pub parents: Vec<String>,
    pub author: String,
    pub committer: String,
    pub message: String,
}

#[derive(Debug)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

// A Blob is just a wrapper around a byte vector. It represents file content.
#[derive(Debug)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Tree {
    // This function will convert the Tree struct into the byte format for hashing.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for entry in &self.entries {
            // Format is: `mode<space>name\0<raw_hash_bytes>`
            bytes.extend_from_slice(entry.mode.as_bytes());
            bytes.push(b' '); // space
            bytes.extend_from_slice(entry.name.as_bytes());
            bytes.push(b'\0'); // null byte
            bytes.extend_from_slice(&entry.hash);
        }
        bytes
    }
}

impl Commit {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Format is:
        // tree <tree_hash>\n
        // parent <parent_hash>\n  (optional, repeated for multiple parents)
        // author <author_info>\n
        // committer <committer_info>\n
        // \n
        // <commit_message>
        bytes.extend_from_slice(format!("tree {}\n", self.tree_hash).as_bytes());
        for parent in &self.parents {
            bytes.extend_from_slice(format!("parent {}\n", parent).as_bytes());
        }
        bytes.extend_from_slice(format!("author {}\n", self.author).as_bytes());
        bytes.extend_from_slice(format!("committer {}\n", self.committer).as_bytes());
        bytes.push(b'\n');
        bytes.extend_from_slice(self.message.as_bytes());
        bytes
    }
}


impl fmt::Display for GitObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitObject::Blob(blob) => write!(f, "{}", String::from_utf8_lossy(&blob.content)),
            GitObject::Tree(tree) => {
                // For a tree, "displaying" it means printing its entries, similar to `ls -l`.
                for entry in &tree.entries {
                    writeln!(
                        f,
                        "{} {} {}",
                        entry.mode,
                        // Convert the raw hash bytes to a hex string for display.
                        hex::encode(&entry.hash),
                        entry.name
                    )?;
                }
                Ok(())
            }
            GitObject::Commit(commit) => {
                // For a commit, "displaying" it means printing its metadata and message.
                writeln!(f, "tree {}", commit.tree_hash)?;
                for parent in &commit.parents {
                    writeln!(f, "parent {}", parent)?;
                }
                writeln!(f, "author {}", commit.author)?;
                writeln!(f, "committer {}", commit.committer)?;
                writeln!(f, "")?; // Blank line
                writeln!(f, "{}", commit.message)?;
                Ok(())
            }
        }
    }
}