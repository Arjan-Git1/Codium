# Codium

A Vim-inspired code editor for the terminal.

Built with Rust and designed to be fast, lightweight, and easy to use. Codium provides a clean terminal editing experience with syntax highlighting and keyboard-driven navigation. A graphical version is planned for the future.

## Tech Stack

### Rust
Main programming language used to build Codium.

### Crossterm
Used for:
- Terminal control
- Keyboard input handling
- Mouse events
- Cross-platform terminal support

### Ratatui
Used for:
- Terminal user interface
- Layout management
- Widgets and rendering

### Syntect
Used for:
- Syntax highlighting
- Language detection
- Theme support

---

# Installation

## Download

Download the latest Codium executable for your operating system from the releases page.

---

## Linux

### 1. Make the executable runnable

```bash
chmod +x codium
```

### 2. Move it to a directory in your PATH

```bash
sudo mv codium /usr/local/bin/
```

### 3. Verify the installation

```bash
codium --version
```

If a version number appears, Codium has been installed successfully.

---

## macOS

### 1. Make the executable runnable

```bash
chmod +x codium
```

### 2. Move it to a directory in your PATH

```bash
sudo mv codium /usr/local/bin/
```

### 3. Verify the installation

```bash
codium --version
```

---

## Windows

### 1. Download `codium.exe`

### 2. Create a folder

Example:

```text
C:\Tools\Codium
```

### 3. Place `codium.exe` inside the folder

### 4. Add the folder to PATH

1. Press `Win + R`
2. Type `sysdm.cpl`
3. Open the **Advanced** tab
4. Click **Environment Variables**
5. Under **System Variables**, select **Path**
6. Click **Edit**
7. Click **New**
8. Add:

```text
C:\Tools\Codium
```

9. Save all changes

### 5. Restart your terminal

### 6. Verify installation

```powershell
codium --version
```

If a version number is displayed, the installation was successful.

---

# Building From Source

## Prerequisites

Install Rust from:

https://rust-lang.org

Verify installation:

```bash
rustc --version
cargo --version
```

---

## Clone the Repository

```bash
git clone https://github.com/yourusername/codium.git
```

Move into the project directory:

```bash
cd codium
```

---

## Build a Release Version

```bash
cargo build --release
```

The compiled binary will be located at:

```text
target/release/codium
```

---

## Run Codium

```bash
cargo run
```

or

```bash
./target/release/codium
```

---

# Usage

Open a file:

```bash
codium main.rs
```

Create a new file:

```bash
codium notes.txt
```

Open the current directory:

```bash
codium .
```

Open a project folder:

```bash
codium ~/Projects/my-project
```

---

# Contributing

Contributions are welcome and appreciated.

Whether you want to fix bugs, improve documentation, optimize performance, or add new features, your help is always welcome.

## How to Contribute

### 1. Fork the Repository

Create your own fork of the project.

### 2. Clone Your Fork

```bash
git clone https://github.com/yourusername/codium.git
```

### 3. Create a Branch

```bash
git checkout -b feature/my-feature
```

### 4. Make Your Changes

Implement your improvements, fixes, or new features.

### 5. Test Your Changes

Ensure the project builds successfully:

```bash
cargo build
cargo test
```

### 6. Commit Your Changes

```bash
git commit -m "Add my feature"
```

### 7. Push Your Branch

```bash
git push origin feature/my-feature
```

### 8. Open a Pull Request

Submit a Pull Request describing your changes and why they should be included.

---

## Contribution Guidelines

- Keep code clean and readable.
- Follow Rust best practices.
- Document new features when necessary.
- Test your changes before submitting.
- Keep pull requests focused on a single change whenever possible.

---

# Reporting Bugs

Found a bug?

Please open an issue and include:

- Operating system
- Codium version
- Steps to reproduce
- Expected behavior
- Actual behavior
- Screenshots (if applicable)

---

# Feature Requests

Have an idea for Codium?

Open an issue describing:

- The problem you're trying to solve
- Your proposed solution
- Alternative solutions you've considered

Feature requests and feedback are always welcome.

---

# License

This project is licensed under the MIT License.

Feel free to use, modify, and distribute Codium according to the terms of the license.
