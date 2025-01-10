# Recursive Grep

Recursive Grep is a custom implementation of the `grep` command, enabling recursive search for a string or pattern within a directory tree. It is a fast and efficient tool for analyzing text file content.

---

## Features

- **Recursive Search**: Traverses all files and subdirectories in a given directory.
- **Regular Expression Support**: Allows advanced searches using complex patterns.
- **Ignore case option**: Allows case insensitive searching.
- **Result Highlighting**: Displays matches with highlighted text for better readability.
- **Optimized Performance**: Designed to handle large directories and files efficiently.
- **Cross-Platform Compatibility**: Works on Linux, macOS, and Windows.

<!-- ---

## Project Structure

The project is written in **Rust** and is organized as follows:

- **`src/main.rs`**: Contains the main application logic, including command-line argument handling.
- **`src/lib.rs`**: Includes functions for directory traversal, pattern matching, and text processing.
- **`tests/`**: Provides unit tests for the core functionalities.
- **`Cargo.toml`**: Specifies the project metadata and dependencies.

--- -->

## How to Run the Application

### Step 1: Clone the Repository

Clone the repository to your local system:
```bash
git clone https://github.com/MarianRadu29/Recursive-grep.git
cd Recursive-grep
```

### Step 2: Install [**Cargo**](https://win.rustup.rs/) tool & [**VSCode**](https://code.visualstudio.com/download) with all the extensions below:
      - rust-analyzer (id: rust-lang.rust-analyzer)
      - Even Better TOML (id: tamasfe.even-better-toml)
      - CodeLLDB (id: vadimcn.vscode-lldb)

### Step 3: Open the terminal and run **cargo run**

## Be careful!!!

- In application run `rgrep` command for print all commands and with all their options and command `help` for all available commands.

- Before running rgrep with a specific pattern, you must set the root path from where to start the search, this can be done with the `setpath` command.
- Can modifies prompt program,in my code it's just an example.