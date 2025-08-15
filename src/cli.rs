// src/cli.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new, empty Rusgit repository
    Init {
        /// Where to create the repository. Defaults to the current directory.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    HashObject {
        /// The file to be hashed
        file: PathBuf,
        /// Actually write the object to the database
        #[arg(short)]
        write: bool,
    },

    CatFile {
        /// The hash of the object to display
        hash: String,
        /// Pretty-print the object's content
        #[arg(short)]
        pretty_print: bool,
    },

    CommitTree {
        /// The hash of the tree object
        tree_hash: String,
        /// The hash of the parent commit
        #[arg(short)]
        parent_hash: Option<String>,
        /// The commit message
        #[arg(short)]
        message: String,
    },

    Add {
        /// The file(s) to add
        files: Vec<PathBuf>,
    },

    Commit {
        /// The commit message
        #[arg(short)]
        message: String,
    },

    WriteTree,

}
