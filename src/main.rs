// src/main.rs

mod cli;
// mod commands; 
// use std::path;
use rusgit::commands;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => {
            commands::init::init(&path)?;
        }
        Commands::HashObject { file, write } => { // <-- Add this match arm
            commands::hash_object::hash_object(&file, write)?;
        }
        Commands::CatFile { hash, pretty_print } => { // <-- Add this
            commands::cat_file::cat_file(&hash, pretty_print)?;
        }
        Commands::WriteTree => {
            let hash = rusgit::commands::write_tree::write_tree(&Path::new("."))?;
            println!("{}", hex::encode(hash));
        }
        Commands::CommitTree {
            tree_hash,
            parent_hash,
            message,
        } => {
            rusgit::commands::commit_tree::commit_tree(tree_hash, parent_hash, message)?;
        }
        Commands::Add { files } => {
            rusgit::commands::add::add(files)?;
        }
        Commands::Commit { message } => {
            rusgit::commands::commit::commit(message)?;
        }
    }
    Ok(())
}