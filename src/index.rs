// src/index.rs

use anyhow::{bail, Context, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use sha1::{Digest, Sha1};

// Represents a single entry in the index file.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    // We'll simplify and only store what's essential for now.
    // Real Git index entries have more metadata (timestamps, etc.).
    pub mode: u32,
    pub hash: [u8; 20],
    pub path: PathBuf,
}

// Represents the entire index.
#[derive(Debug, Default)]
pub struct Index {
    pub entries: Vec<IndexEntry>,
}

impl Index {
    /// Read the index file from disk and parse it into an Index struct.
    pub fn from_path(path: &Path) -> Result<Self> {
        let mut index = Index::default();

        if !path.exists() {
            return Ok(index); // Return an empty index if the file doesn't exist.
        }

        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let mut data = &data[..]; // Create a slice to read from

        // 1. Read the header (12 bytes).
        let signature = data.read_u32::<BigEndian>()?;
        if signature != 0x44495243 { // "DIRC" in ASCII
            bail!("Invalid index signature");
        }
        let version = data.read_u32::<BigEndian>()?;
        if version != 2 {
            bail!("Unsupported index version: {}", version);
        }
        let entry_count = data.read_u32::<BigEndian>()?;

        // 2. Read the entries.
        for _ in 0..entry_count {
            // We'll skip ctime, mtime, dev, ino, uid, gid, size (60 bytes total)
            // A real implementation would parse these.
            data.read_exact(&mut [0u8; 8])?; // ctime
            data.read_exact(&mut [0u8; 8])?; // mtime
            data.read_exact(&mut [0u8; 4])?; // dev
            data.read_exact(&mut [0u8; 4])?; // ino
            let mode = data.read_u32::<BigEndian>()?;
            data.read_exact(&mut [0u8; 4])?; // uid
            data.read_exact(&mut [0u8; 4])?; // gid
            data.read_exact(&mut [0u8; 4])?; // size

            let mut hash = [0u8; 20];
            data.read_exact(&mut hash)?;

            let flags = data.read_u16::<BigEndian>()?;
            let path_len = (flags & 0x0FFF) as usize;

            let mut path_bytes = vec![0u8; path_len];
            data.read_exact(&mut path_bytes)?;
            let path_str = std::str::from_utf8(&path_bytes)?;
            let path = PathBuf::from(path_str);

            index.entries.push(IndexEntry { mode, hash, path });

            // Entries are padded with null bytes to align to 8-byte boundaries.
            let entry_len = 62 + path_len; // 62 bytes of metadata + path
            let padding = (8 - (entry_len % 8)) % 8;
            data.read_exact(&mut vec![0u8; padding])?;
        }

        // We're skipping the checksum at the end for simplicity.
        Ok(index)
    }

    /// Write the Index struct back to a binary file on disk.
    pub fn write(&self, path: &Path) -> Result<()> {
        let mut file_content = Vec::new();

        // 1. Write the header.
        file_content.write_u32::<BigEndian>(0x44495243)?; // "DIRC"
        file_content.write_u32::<BigEndian>(2)?; // Version 2
        file_content.write_u32::<BigEndian>(self.entries.len() as u32)?;

        // 2. Write the entries.
        for entry in &self.entries {
            // Again, we're writing zero for most metadata fields.
            file_content.write_u32::<BigEndian>(0)?; // ctime seconds
            file_content.write_u32::<BigEndian>(0)?; // ctime nanoseconds
            file_content.write_u32::<BigEndian>(0)?; // mtime seconds
            file_content.write_u32::<BigEndian>(0)?; // mtime nanoseconds
            file_content.write_u32::<BigEndian>(0)?; // dev
            file_content.write_u32::<BigEndian>(0)?; // ino
            file_content.write_u32::<BigEndian>(entry.mode)?;
            file_content.write_u32::<BigEndian>(0)?; // uid
            file_content.write_u32::<BigEndian>(0)?; // gid
            file_content.write_u32::<BigEndian>(0)?; // size

            file_content.write_all(&entry.hash)?;

            let path_bytes = entry.path.to_str().context("Non-UTF8 path")?.as_bytes();
            file_content.write_u16::<BigEndian>(path_bytes.len() as u16)?;
            file_content.write_all(path_bytes)?;

            // Pad with null bytes.
            let entry_len = 62 + path_bytes.len();
            let padding = (8 - (entry_len % 8)) % 8;
            file_content.write_all(&vec![0u8; padding])?;
        }

        // 3. Calculate and write the checksum.
        let mut hasher = sha1::Sha1::new();
        hasher.update(&file_content);
        let checksum = hasher.finalize();
        file_content.write_all(&checksum)?;
        
        fs::write(path, &file_content)?;
        Ok(())
    }

    /// A helper to add or update an entry in the index.
    pub fn add(&mut self, path: PathBuf, hash: [u8; 20], mode: u32) {
        // Remove the old entry if it exists.
        self.entries.retain(|e| e.path != path);
        // Add the new entry.
        self.entries.push(IndexEntry { path, hash, mode });
        // Keep the index sorted by path, as Git requires.
        self.entries.sort_by(|a, b| a.path.cmp(&b.path));
    }
}