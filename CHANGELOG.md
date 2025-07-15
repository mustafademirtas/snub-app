# Changelog

All notable changes to Snub will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-07-16

### Added
- **Audio Feedback**
  - Sound playback when toggling microphone mute/unmute
  - Uses native macOS system sounds (`Tink`, `Pop`, `Morse`) for audible feedback
  - Graceful fallback system if preferred sounds are unavailable
  - Silent operation on non-macOS platforms

- **Settings Infrastructure**
  - Basic settings system with telemetry control
  - Settings window with modern UI matching main application design
  - Persistent settings storage (in-memory for now, expandable to file-based)
  - Settings accessible from system tray menu

### Technical Improvements
- **Sound Module**: New `macos_sound` module for system sound playback
- **Settings Module**: Foundation for future configuration options
- **Command Extensions**: Added `get_settings` and `set_telemetry_enabled` commands
- **Error Handling**: Improved error handling for audio playback operations

### User Experience
- **Enhanced Feedback**: Users now receive audio confirmation when toggling microphone
- **Accessibility**: Audio cues complement visual feedback for better user experience
- **Settings Access**: Convenient access to application settings from tray menu

---

## [1.0.0] - 2025-07-13

### Added
- **Core Functionality**
  - One-click microphone toggle with system-level mute/unmute
  - Real-time microphone state detection using macOS system APIs
  - Visual feedback with different icons for muted/unmuted states

- **User Interface**
  - Minimal 100x100px floating window with rounded corners
  - Glassmorphism design with transparency and backdrop blur
  - Modern close button that appears on hover
  - Smooth animations and transitions throughout the UI
  - Dark mode support that automatically adapts to system theme

- **Window Management**
  - Draggable window that can be moved anywhere on screen
  - Always-on-top functionality to keep controls accessible
  - Custom titlebar with no decorations for clean appearance
  - Window hiding instead of closing to maintain background operation

- **System Integration**
  - System tray icon with context menu
  - Dynamic tray icon that reflects current microphone state
  - macOS dock hiding - app runs as background utility
  - Native macOS activation policy for proper utility app behavior

- **Keyboard Controls**
  - Escape key to close/hide the window
  - Spacebar to toggle microphone when window is focused
  - Full keyboard navigation support

- **Menu System**
  - System tray menu with "Toggle Microphone" option
  - "Show Window" option to bring back the floating interface
  - "Quit" option for clean application exit

- **Technical Features**
  - Built with Tauri v2 for optimal performance and security
  - Rust backend for system-level microphone control
  - TypeScript frontend with Vite for fast development
  - Native macOS APIs via osascript for reliable microphone control
  - Cocoa framework integration for proper dock hiding

### Technical Details
- **Frontend**: TypeScript + Vite + Vanilla CSS
- **Backend**: Rust with Tauri v2
- **Platform**: macOS-specific system integration
- **Architecture**: Native desktop app with web technologies
- **Build System**: Cargo + Yarn with Tauri CLI

### Dependencies
- Tauri v2 with macOS private API support
- Cocoa framework for native macOS integration
- Image processing for tray icon management
- Serde for data serialization

---

## Release Notes

### v1.1.0 - Audio Feedback Update

This release introduces audio feedback to enhance the user experience when toggling the microphone. The app now plays subtle system sounds to confirm mute/unmute actions, providing both visual and auditory feedback.

**New Features:**
- 🔊 Audio feedback for microphone toggle actions
- ⚙️ Settings window with telemetry controls
- 🎵 Native macOS system sound integration
- 🔧 Expandable settings infrastructure

**Technical Enhancements:**
- New sound playback module with graceful fallbacks
- Settings system foundation for future configuration options
- Enhanced error handling for audio operations
- Cross-platform compatibility maintained

**User Benefits:**
- Immediate audio confirmation of microphone state changes
- Better accessibility with multi-sensory feedback
- Preparation for future customization options

---

### v1.0.0 - Initial Release

This is the initial stable release of Snub, a minimal macOS microphone toggle utility. The app provides a clean, distraction-free way to control your system microphone with a floating interface that stays out of your way.

**Key Highlights:**
- 🎤 Instant microphone control with visual feedback
- 🪟 Beautiful floating window with modern design
- 🔕 Background operation without dock clutter
- 🎨 System theme integration and smooth animations
- ⌨️ Convenient keyboard shortcuts
- 📱 System tray integration for quick access

**Platform Support:**
- macOS (required for system microphone control)

**Installation:**
Download from releases or build from source using the provided instructions in README.md.
