# Copilot Instructions for Snub

## Project Purpose

**Snub** is a macOS utility application that provides a simple interface to toggle the system microphone on/off. The app features a minimal UI with a single microphone icon that users can click to toggle microphone state.

### Core Requirements
- **Platform**: macOS-specific system microphone control
- **UI**: Single microphone icon with click-to-toggle functionality
- **Behavior**: Visual feedback for microphone state (muted/unmuted)
- **Simplicity**: Minimal, focused interface without extra features

## Project Architecture

This is a **Tauri v2 application** combining a TypeScript/Vite frontend with a Rust backend. The architecture follows Tauri's dual-process model:

- **Frontend**: Vanilla TypeScript + Vite dev server (port 1420)
- **Backend**: Rust library (`snub_lib`) with Tauri commands
- **Communication**: Frontend calls Rust via `@tauri-apps/api/core` invoke()

## Key Directory Structure

```
src/                         # TypeScript frontend code
src-tauri/src/              # Rust backend code
  ├── main.rs               # Entry point (calls lib.rs::run())
  ├── lib.rs                # Main library entry point
  └── modules/              # Organized module structure
      ├── mod.rs            # Module declarations
      ├── audio.rs          # macOS audio control via AppleScript
      ├── commands.rs       # Tauri command handlers for frontend
      ├── errors.rs         # Error handling and logging
      ├── menu.rs           # Tray menu event handling
      ├── setup.rs          # Application initialization
      ├── tray.rs           # System tray icon management
      └── types.rs          # Data structures and constants
src-tauri/tauri.conf.json   # App configuration
```

## Development Workflow

### Essential Commands
- `yarn dev` - Start frontend dev server + Tauri app
- `yarn build` - Build for production
- `yarn tauri dev` - Alternative dev command
- `yarn tauri build` - Create app bundle

### Frontend-Backend Communication Pattern
All Rust functions exposed to frontend must be:
1. Decorated with `#[tauri::command]`
2. Added to `invoke_handler` in `lib.rs`
3. Called from frontend using `invoke("command_name", { args })`

### Current Implementation Pattern
All Rust commands are organized in `src-tauri/src/modules/commands.rs` and registered in the main `lib.rs` file.

Example from current codebase:
```rust
// src-tauri/src/modules/commands.rs
#[tauri::command]
pub fn get_microphone_state() -> Result<MicrophoneState, String> { ... }

// src-tauri/src/lib.rs
use modules::commands::{get_microphone_state, set_microphone_mute, toggle_microphone};

tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        get_microphone_state,
        toggle_microphone,
        set_microphone_mute
    ])  // Register here
```

```typescript
// src/main.ts
await invoke("get_microphone_state")  // Call from frontend
await invoke("toggle_microphone")
```

## Current Project Status

The application has been fully implemented with:
- ✅ System tray integration with dynamic icons
- ✅ macOS microphone control via AppleScript  
- ✅ One-click toggle functionality
- ✅ Visual feedback for muted/unmuted states
- ✅ Modular Rust backend architecture
- ✅ Frontend-backend communication established
- ✅ Window management and hide-on-close behavior

## Project-Specific Conventions

### TypeScript Frontend
- Uses vanilla TypeScript (no framework) with strict mode enabled
- DOM manipulation through direct `document.querySelector()` calls
- Event handling attached in `DOMContentLoaded` listener
- Assets in `src/assets/` (vite.svg, tauri.svg, typescript.svg)

### Rust Backend
- Library crate pattern: `main.rs` calls `lib.rs::run()`
- Modular architecture: All functionality organized in `src/modules/`
- Serde for JSON serialization (`serde = { features = ["derive"] }`)
- Current plugins: `tauri-plugin-opener` for external links
- macOS integration: Uses AppleScript for microphone control via `osascript`

### Build Configuration
- Vite config optimized for Tauri (fixed port 1420, ignores `src-tauri` changes)
- Uses yarn as package manager
- TypeScript target: ES2020 with bundler module resolution

## Critical Files for Changes

- `src-tauri/src/lib.rs` - Main library entry point
- `src-tauri/src/modules/commands.rs` - Add new Tauri commands here
- `src-tauri/src/modules/` - All modular functionality organized here
- `src-tauri/Cargo.toml` - Rust dependencies and features
- `src-tauri/tauri.conf.json` - App metadata, windows, security
- `src/main.ts` - Frontend logic and API calls
- `package.json` - Frontend dependencies and scripts

## Common Patterns

### Adding New Tauri Commands
1. Define function in `src-tauri/src/modules/commands.rs` with `#[tauri::command]`
2. Add to `generate_handler![]` macro in `lib.rs`
3. Call from frontend with matching parameter names
4. Use `Result<T, String>` for error handling in Rust commands

### Microphone Control Implementation
- Rust backend uses macOS system APIs for audio device control
- Frontend shows visual state (icon changes, colors) for muted/unmuted
- Uses `AppleScript` via `osascript` command for microphone control
- Permission requests for microphone access handled gracefully
- Current implementation: See `src-tauri/src/modules/audio.rs` for macOS audio control

### Window Management
Default window size: 800x600. For mic toggle app, consider smaller window dimensions.
Modify in `tauri.conf.json` under `app.windows[0]`.

### Security
CSP is disabled (`"csp": null`). Enable for production apps handling external content.

### CSS Styling Guidelines
- **Use BEM (Block Element Modifier) methodology** for all CSS class naming
- BEM structure: `block__element--modifier`
- Examples:
  - `.mic-toggle` (block)
  - `.mic-toggle__icon` (element)
  - `.mic-toggle__icon--muted` (modifier)
  - `.mic-toggle--disabled` (block modifier)
- Keep class names descriptive and semantic
- Avoid deep nesting in CSS selectors
- Use consistent naming patterns throughout the application
