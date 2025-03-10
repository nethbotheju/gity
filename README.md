# Gity

A command-line interface client for Git operations implemented in Rust. This tool aims to provide a simplified and user-friendly interface for common Git operations.

---

## Features

🚧 **Currently in development:**

- ✅ **Repository initialization**: `gity init .` - Initialize a new Git repository in the current directory.
- ✅ **Basic Git operations**:
  - `gity add .` - Track newly added files (only new files, not removed files).
  - `gity commit -m "<message>"` - Commit changes with a short commit title (no description support yet).
  - `gity push` - Push local commits to a remote repository.
- 🚧 **Branch management** (Planned): Ability to create, switch, and delete branches.
- 🚧 **Status checking** (Planned): View the status of changes in the repository.
- 🚧 **Remote repository handling** (Planned): Manage remotes, including adding and removing remote repositories.

---

## Installation

Precompiled versions of the Gity CLI are available for **macOS** and **Windows**.

### **macOS Installation:**

1. Download the `gity` executable from the [Releases Page](https://github.com/nethbotheju/gity/releases).
2. Move the downloaded file to a directory of your choice.
3. Add the directory to your system’s `PATH` for easy access.
4. You can now use `gity` from the terminal.

### **Windows Installation:**

1. Download the `gity.exe` executable from the [Releases Page](https://github.com/nethbotheju/gity/releases).
2. Move `gity.exe` to a directory of your choice.
3. Add the directory to your system’s `PATH`.
4. You can now use `gity` from the Command Prompt or PowerShell.

---

## Development

This project is built using:

- **Rust** 2024 edition
- **Cargo** package manager

---

## Contributing

Contributions are welcome! If you find a bug or have an idea for a new feature, please feel free to open an issue or submit a pull request.