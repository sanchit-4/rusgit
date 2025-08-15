# Rusgit: A Git Clone in Rust

Rusgit is a functional clone of the Git version control system, built from the ground up in Rust. This project was undertaken as a learning exercise to deeply understand the internal data model and architecture of Git. It implements the fundamental "plumbing" and "porcelain" commands that power Git's core functionality.

Table of Contents

How Git Works Under the Hood
Project Architecture
Features
Setup and Installation
How to Use (End-to-End Test)

How Git Works Under the Hood

Before using rusgit, it's helpful to understand the simple yet powerful model it's based on. Git is not a system of "diffs" or "changesets"; it is fundamentally a content-addressed filesystem.

There are three core ideas:

    Objects: Everything in Git is stored as an object. There are three main types:

        Blob: Stores the raw content of a file. A blob knows nothing about its filename or location; it is pure data.

        Tree: Represents a directory. It contains a list of entries, where each entry points to a blob (for a file) or another tree (for a subdirectory), along with its name and permissions. A tree allows for the reconstruction of a project's entire file structure at a specific moment.

        Commit: A snapshot of your project at a point in time. It points to a single tree object (the root of your project), one or more parent commits (which forms the history), and metadata like the author, committer, and commit message.

    Content-Addressing: Every object is stored in a directory named after its own SHA-1 hash. The hash is calculated from the object's content (e.g., blob <size>\0<file_content>). This means identical files are only stored once, and any change to a file results in a completely new object.

    Branches and Pointers: A branch (like main) is simply a file located at .git/refs/heads/main that contains the 40-character SHA-1 hash of the latest commit on that branch. The HEAD file (.git/HEAD) is a pointer that tells Git which branch you are currently on.

Project Architecture

rusgit is a command-line application built with a clean separation between its interface, core logic, and data models.

The project is structured as follows:

    src/main.rs: The main entry point. This file is the Dispatcher. Its only job is to parse command-line arguments using clap and delegate the work to the appropriate command module.

    src/lib.rs: The library root. It declares and exports all the public modules of our application, defining the library's public API.

    src/cli.rs: Defines the entire command-line interface using clap's derive macros. This file is the "public contract" of our application, describing all available commands and their arguments.

    src/objects.rs: Defines the core Git object types (Blob, Tree, Commit) as Rust structs and enums. It also contains the logic for serializing these objects into the byte format that Git expects on disk.

    src/index.rs: Implements the logic for reading and writing Git's index file (.git/index). The index acts as the staging area, a bridge between the working directory and the next commit.

    src/repository.rs: Contains helper functions for interacting with the repository state, such as reading and updating HEAD.

    src/commands/: A directory containing the specialist workers. Each file implements the logic for a single command (e.g., init.rs, add.rs, commit.rs). This keeps the business logic for each feature isolated and organized.

Features

rusgit implements the following commands:
Plumbing Commands (Low-Level)

    init: Initializes a new .git directory.

    hash-object: Hashes a file and optionally writes it to the object database as a blob.

    cat-file: Reads an object from the database by its hash and prints its content.

    write-tree: Creates a tree object from the current directory.

    commit-tree: Creates a commit object from a tree, parent, and message.

Porcelain Commands (User-Friendly)

    add: Adds file contents to the staging area (the index).

    commit: Creates a new commit from the staged files in the index, updating the current branch.

Setup and Installation

To get rusgit running, you'll need the Rust toolchain installed on your system.

Clone the repository:
```
git clone https://github.com/sanchit-4/rusgit
cd rusgit
```
  

Build the project:
```
cargo build
```
      

For an optimized release build, use cargo build --release. The executable will be located at target/release/rusgit.

# How to Use (End-to-End Test)

You can use rusgit to initialize a repository, add files, and create a commit history. All commands should be run from the project's root directory.

Here is a complete workflow to test all functionality:

```
mkdir test_repo
cd test_repo
```
  

Initialize the Repository:
```
cargo run -- init .
```
  

Create a File and Stage It:
``` 
echo "My First Commit" > README.md
cargo run -- add README.md
```
  

Make Your First Commit:
```   
cargo run -- commit -m "Initial commit of README"
```
  

Make Changes and a Second Commit:
```
echo "An update." >> README.md
echo "fn main() {}" > main.rs
```

Add both files to the staging area
```
cargo run -- add README.md main.rs
```
Commit the changes
```
cargo run -- commit -m "Add main.rs and update README"
```
  

Verify the History (Manually):
``` 
cat .git/refs/heads/main
```
  

Inspect that commit:
Take the hash from the previous command and run:
```
cargo run -- cat-file <latest-commit-hash> -p
```
  

You will see that its parent field points to the hash of your first commit, proving that the history is correctly linked.
