# Contributing to Snub

Thank you for your interest in contributing to **Snub**! This document provides guidelines and information for contributors to help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Architecture](#project-architecture)
- [Contributing Guidelines](#contributing-guidelines)
- [Submitting Changes](#submitting-changes)
- [Issue Guidelines](#issue-guidelines)
- [Code Style](#code-style)
- [Testing](#testing)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow. Please be respectful, inclusive, and constructive in all interactions.

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome and support people of all backgrounds and identities
- **Be collaborative**: Work together constructively and assume good intentions
- **Be patient**: Remember that people have different skill levels and experience

## Getting Started

### Prerequisites

Before contributing, make sure you have the following installed:

- **Node.js** 20.11.0 (see `.nvmrc` file) and **Yarn**
- **Rust** (latest stable version)
- **Xcode Command Line Tools** (for macOS development)
- **Git**

> **Note**: This project uses a `.nvmrc` file to specify the Node.js version. If you're using nvm, run `nvm use` in the project root to automatically switch to the correct version.

### First Time Setup

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/mustafademirtas/snub-app.git
   cd snub-app
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/mustafademirtas/snub-app.git
   ```
4. **Use the correct Node.js version** (if using nvm):
   ```bash
   nvm use
   ```
5. **Install dependencies**:
   ```bash
   yarn install
   ```
6. **Run the app** in development mode:
   ```bash
   yarn tauri dev
   ```

## Development Setup

### Development Commands

- `yarn dev` - Start frontend dev server + Tauri app
- `yarn build` - Build TypeScript and create production bundle
- `yarn tauri dev` - Alternative development command
- `yarn tauri build` - Create production app bundle
- `yarn preview` - Preview production build

### Project Structure

```
src/                      # TypeScript frontend code
├── main.ts              # Main frontend entry point
├── styles.css           # Global styles
└── assets/              # Static assets

src-tauri/               # Rust backend code
├── src/
│   ├── main.rs         # Entry point (calls lib.rs::run())
│   └── lib.rs          # Tauri app setup + command handlers
├── Cargo.toml          # Rust dependencies
├── tauri.conf.json     # App configuration
└── icons/              # App icons and tray icons

```

## Project Architecture

Snub is built with **Tauri v2**, combining:

- **Frontend**: Vanilla TypeScript + Vite (no frameworks)
- **Backend**: Rust with macOS system integration
- **Communication**: Frontend ↔ Rust via `@tauri-apps/api/core.invoke()`

### Key Technologies

- **Tauri v2**: Cross-platform app framework
- **TypeScript**: Frontend logic with strict typing
- **Rust**: Backend for system-level microphone control
- **macOS CoreAudio**: Native audio device management
- **Vite**: Frontend build tool and dev server

## Contributing Guidelines

### Types of Contributions

We welcome various types of contributions:

- 🐛 **Bug fixes** - Fix issues or improve stability
- ✨ **Features** - Add new functionality (discuss first in issues)
- 📚 **Documentation** - Improve docs, README, or code comments
- 🎨 **UI/UX** - Enhance visual design or user experience
- ⚡ **Performance** - Optimize speed or resource usage
- 🧪 **Testing** - Add or improve tests
- 🔧 **Tooling** - Improve build process or developer experience

### Before You Start

1. **Check existing issues** - Look for similar work or discussions
2. **Create an issue** - For significant changes, discuss your approach first
3. **Keep it focused** - One feature/fix per pull request
4. **Follow conventions** - Use existing code style and patterns

## Submitting Changes

### Pull Request Process

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. **Make your changes** following the code style guidelines

3. **Test your changes**:
   ```bash
   yarn tauri dev  # Test in development
   yarn tauri build  # Ensure production build works
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add microphone sensitivity controls"
   # or
   git commit -m "fix: resolve tray icon visibility issue"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a pull request** on GitHub with:
   - Clear title and description
   - Reference to related issues
   - Screenshots/videos for UI changes
   - Testing instructions

### Commit Message Guidelines

Use conventional commits format:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code changes that neither fix bugs nor add features
- `test:` - Adding or updating tests
- `chore:` - Build process or auxiliary tool changes

Examples:
```
feat: add keyboard shortcut for global microphone toggle
fix: resolve audio device detection on macOS Ventura
docs: update installation instructions for Apple Silicon
style: format Rust code with rustfmt
```

## Issue Guidelines

### Reporting Bugs

When reporting bugs, please include:

- **macOS version** and architecture (Intel/Apple Silicon)
- **Snub version** or commit hash
- **Steps to reproduce** the issue
- **Expected behavior** vs. **actual behavior**
- **Screenshots or screen recordings** if applicable
- **Console logs** if available

### Feature Requests

For feature requests, please provide:

- **Use case**: Why is this feature needed?
- **Proposed solution**: How should it work?
- **Alternatives considered**: Other approaches you've thought of
- **Additional context**: Screenshots, mockups, or examples

## Code Style

### TypeScript Frontend

- Use **strict TypeScript** with proper typing
- Follow **ES6+ conventions** (const/let, arrow functions, etc.)
- Use **semicolons** and **double quotes** for strings
- Prefer **functional programming** patterns where appropriate
- **No external frameworks** - keep it vanilla TypeScript

```typescript
// Good
const button = document.querySelector<HTMLButtonElement>("#mic-button");
if (button) {
    button.addEventListener("click", async () => {
        await invoke("toggle_microphone");
    });
}

// Avoid
var button = document.getElementById("mic-button");
button.onclick = function() { /* ... */ };
```

### Rust Backend

- Follow **standard Rust conventions** (rustfmt)
- Use **descriptive error handling** with proper Result types
- Add **documentation comments** for public functions
- Prefer **safe Rust** over unsafe when possible
- Use **serde** for JSON serialization

```rust
/// Toggles the system microphone mute state
#[tauri::command]
async fn toggle_microphone() -> Result<bool, String> {
    match audio_manager::toggle_mute() {
        Ok(is_muted) => Ok(is_muted),
        Err(e) => Err(format!("Failed to toggle microphone: {}", e)),
    }
}
```

### CSS Styles

- Use **CSS custom properties** for theming
- Follow **BEM methodology** for class naming
- Prefer **CSS Grid/Flexbox** over floats
- Use **relative units** (rem, em, %) when appropriate
- Support **dark mode** with system preferences

## Testing

### Manual Testing

Before submitting changes, please test:

1. **Development mode**: `yarn tauri dev` works correctly
2. **Production build**: `yarn tauri build` completes successfully
3. **Core functionality**: Microphone toggle works as expected
4. **UI interactions**: All buttons and interactions work
5. **System integration**: Tray icon and notifications work
6. **Permissions**: App requests microphone permissions properly

### Platform Testing

Since this is a macOS-specific app, test on:
- **macOS versions**: Latest 2-3 major versions (Big Sur, Monterey, Ventura, Sonoma)
- **Hardware**: Both Intel and Apple Silicon Macs if possible
- **Audio devices**: Different microphone types (built-in, USB, Bluetooth)

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions
- **PATCH** version for backwards-compatible bug fixes

### Release Checklist

1. Update version in `package.json` and `Cargo.toml`
2. Update `CHANGELOG.md` with new features and fixes
3. Test production build on clean system
4. Create release tag and GitHub release
5. Attach built app bundle to release

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Code Review**: Tag maintainers in pull requests for review

## Recognition

Contributors will be recognized in:
- **CHANGELOG.md** for significant contributions
- **README.md** contributors section
- **GitHub releases** notes

---

Thank you for contributing to Snub! Your efforts help make this tool better for everyone. 🎤✨
