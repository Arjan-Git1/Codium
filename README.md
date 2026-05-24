# Codium

A Vim-inspired code editor for the terminal.

Codium is a fast, lightweight, keyboard-first text editor built in Rust. It is designed for developers who prefer working directly in the terminal while still enjoying modern editor features such as syntax highlighting, responsive rendering, and an efficient editing workflow.

The project aims to provide a clean editing experience with minimal resource usage and maximum productivity.

> GUI version coming soon.

---

# Table of Contents

- Introduction
- Why Codium?
- Philosophy
- Features
- Tech Stack
- Installation
- Building From Source
- Usage
- Project Structure
- Architecture
- Rendering System
- Input Handling
- Syntax Highlighting
- Configuration
- Performance Goals
- Development Setup
- Coding Standards
- Testing
- Troubleshooting
- FAQ
- Roadmap
- Contributing
- Changelog
- Credits
- License

---

# Introduction

Modern code editors have become increasingly feature-rich. While those features are useful, they often come at the cost of startup speed, memory usage, and simplicity.

Codium takes a different approach.

The editor is designed around a simple idea:

> A code editor should feel fast, responsive, and comfortable regardless of the machine it runs on.

Whether you are writing Rust, Python, JavaScript, C++, or configuration files, Codium aims to provide an efficient terminal-based editing experience.

---

# Why Codium?

Many developers spend most of their day inside a terminal.

For these developers, a terminal editor offers several advantages:

- Faster startup times
- Lower memory consumption
- Easy remote server access
- Keyboard-driven workflows
- Minimal distractions

Codium combines these benefits with modern Rust tooling.

---

# Philosophy

## Speed First

Editors should never become a bottleneck.

Every feature should be designed with performance in mind.

---

## Keyboard Driven

The keyboard is often the fastest way to interact with software.

Codium embraces keyboard-first workflows.

---

## Simplicity

Complexity should only be added when it provides real value.

---

## Reliability

Editors should behave predictably and consistently.

---

## Extensibility

Future versions of Codium should allow users to customize and extend functionality.

---

# Features

## Current Features

### Terminal-Based Editing

Works directly inside your terminal.

### Cross Platform

Supports:

- Linux
- Windows
- macOS

### Syntax Highlighting

Powered by Syntect.

### Fast Rendering

Efficient screen updates.

### Lightweight Design

Low memory usage.

### Rust-Powered

Safe and performant.

---

## Planned Features

### Multiple Cursors

Edit multiple locations simultaneously.

### LSP Support

Language Server Protocol integration.

### Auto Completion

Smarter coding assistance.

### Git Integration

Built-in Git tools.

### File Explorer

Navigate projects more easily.

### Plugin System

Extend Codium with community plugins.

### Themes

Custom color themes.

### GUI Version

Native graphical interface.

---

# Tech Stack

## Rust

Main programming language.

Benefits:

- Memory safety
- Performance
- Reliability
- Strong tooling

---

## Crossterm

Provides:

- Keyboard input
- Mouse input
- Terminal manipulation
- Event handling

---

## Ratatui

Provides:

- User interface rendering
- Layout management
- Widgets
- Terminal graphics

---

## Syntect

Provides:

- Syntax highlighting
- Language detection
- Theme support

---

# Installation

## Download

Download the latest release from the project's release page.

---

## Linux Installation

### Step 1

Download the binary.

### Step 2

Make it executable.

```bash
chmod +x codium
```

### Step 3

Move it into PATH.

```bash
sudo mv codium /usr/local/bin/
```

### Step 4

Verify installation.

```bash
codium --version
```

---

## macOS Installation

### Step 1

Download the binary.

### Step 2

Make it executable.

```bash
chmod +x codium
```

### Step 3

Move it into PATH.

```bash
sudo mv codium /usr/local/bin/
```

### Step 4

Verify installation.

```bash
codium --version
```

---

## Windows Installation

### Step 1

Download `codium.exe`.

### Step 2

Create a folder.

```text
C:\Tools\Codium
```

### Step 3

Place the executable inside.

### Step 4

Add the folder to PATH.

### Step 5

Restart terminal.

### Step 6

Verify installation.

```powershell
codium --version
```

---

# Building From Source

## Install Rust

Visit:

https://rust-lang.org

Verify installation:

```bash
rustc --version
cargo --version
```

---

## Clone Repository

```bash
git clone https://github.com/yourusername/codium.git
```

---

## Enter Project

```bash
cd codium
```

---

## Build

```bash
cargo build --release
```

---

## Run

```bash
cargo run
```

---

# Usage

Open a file:

```bash
codium main.rs
```

---

Open a project:

```bash
codium .
```

---

Create a file:

```bash
codium notes.txt
```

---

# Project Structure

```text
src/
├── main.rs
├── editor.rs
├── buffer.rs
├── renderer.rs
├── syntax.rs
├── commands.rs
├── config.rs
└── utils.rs
```

---

## main.rs

Application entry point.

Responsible for:

- Startup
- Initialization
- Event loop setup

---

## editor.rs

Editor state management.

Responsible for:

- Cursor tracking
- File operations
- Buffer interactions

---

## buffer.rs

Text storage layer.

Responsible for:

- Text insertion
- Text deletion
- Line management

---

## renderer.rs

Screen rendering.

Responsible for:

- UI drawing
- Layout updates
- Refresh logic

---

## syntax.rs

Syntax highlighting system.

Responsible for:

- Parsing
- Highlight generation
- Theme handling

---

# Architecture

Codium follows a modular architecture.

Components communicate through clearly defined interfaces.

Benefits:

- Easier maintenance
- Better testing
- Improved scalability

---

# Rendering System

Rendering is handled by Ratatui.

Process:

1. Read editor state
2. Generate UI components
3. Render frame
4. Flush output

---

# Input Handling

Input is handled by Crossterm.

Supported events:

- Keyboard
- Mouse
- Resize events

---

# Syntax Highlighting

Syntax highlighting is powered by Syntect.

Supported languages include:

- Rust
- Python
- JavaScript
- TypeScript
- C
- C++
- Go
- Java
- Kotlin
- Swift
- Shell
- HTML
- CSS
- JSON
- YAML
- TOML
- Markdown

---

# Configuration

Future versions will support configuration files.

Example:

```toml
theme = "dark"
tab_width = 4
line_numbers = true
```

---

# Performance Goals

Codium is designed with the following targets:

- Fast startup
- Low memory usage
- Smooth scrolling
- Responsive input
- Efficient rendering

---

# Development Setup

Install Rust.

Clone repository.

Build project.

Run tests.

Start developing.

---

# Coding Standards

## Naming

Use descriptive names.

---

## Formatting

Use:

```bash
cargo fmt
```

---

## Linting

Use:

```bash
cargo clippy
```

---

## Documentation

Document public APIs.

---

# Testing

Run tests:

```bash
cargo test
```

Run all checks:

```bash
cargo fmt --check
cargo clippy
cargo test
```

---

# Troubleshooting

## Command Not Found

Ensure Codium is in PATH.

---

## Permission Denied

Make executable:

```bash
chmod +x codium
```

---

## Rust Missing

Install Rust and verify:

```bash
rustc --version
```

---

## Build Failure

Update dependencies:

```bash
cargo update
```

---

# FAQ

## Why Rust?

Rust provides performance and safety.

---

## Why Terminal?

Many developers prefer terminal workflows.

---

## Is Codium Open Source?

Yes.

---

## Is Codium Free?

Yes.

---

## Does It Support Plugins?

Planned.

---

## Does It Support LSP?

Planned.

---

## Does It Work On Windows?

Yes.

---

## Does It Work On Linux?

Yes.

---

## Does It Work On macOS?

Yes.

---

## Is A GUI Planned?

Yes.

---

# Roadmap

## Phase 1

- Basic editor
- Syntax highlighting
- File loading
- File saving

---

## Phase 2

- Search
- Replace
- Configuration files

---

## Phase 3

- Multiple cursors
- Themes
- Plugin system

---

## Phase 4

- LSP support
- Git integration
- File explorer

---

## Phase 5

- GUI release

---

# Contributing

Contributions are welcome.

---

## Fork

Create a fork.

---

## Clone

```bash
git clone https://github.com/yourusername/codium.git
```

---

## Branch

```bash
git checkout -b feature/my-feature
```

---

## Commit

```bash
git commit -m "Add feature"
```

---

## Push

```bash
git push origin feature/my-feature
```

---

## Pull Request

Open a pull request.

Describe:

- What changed
- Why it changed
- How it was tested

---

# Contribution Guidelines

- Keep code readable.
- Follow Rust best practices.
- Write tests when appropriate.
- Document new functionality.
- Keep pull requests focused.

---

# Changelog

## v0.1.0

- Initial project
- Basic editor
- File loading

---

## v0.2.0

- Syntax highlighting
- Improved rendering

---

## v0.3.0

- Performance improvements
- Better input handling

---

# Credits

Special thanks to:

- Rust
- Crossterm
- Ratatui
- Syntect
- Contributors
- Testers
- Open source community

---

# License

Licensed under the MIT License.

You are free to use, modify, distribute, and contribute to Codium according to the terms of the license.

---

# Final Note

Thank you for checking out Codium.

Whether you are fixing bugs, improving documentation, testing features, suggesting ideas, or contributing code, your support helps make the project better for everyone.

Happy coding!
