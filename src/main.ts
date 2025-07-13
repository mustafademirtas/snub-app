/**
 * Snub - macOS Microphone Control Application Frontend
 * 
 * This module handles the frontend UI for the Snub application, providing
 * a simple interface to control microphone state with visual feedback.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

// Constants for better maintainability
const ELEMENT_IDS = {
  MIC_TOGGLE: "#mic-toggle",
  MIC_ICON: "#mic-icon", 
  MIC_OFF_ICON: "#mic-off-icon",
  CLOSE_BUTTON: "#close-button",
  DRAGGABLE_AREA: "#draggable-area",
  MIC_STATUS_DESCRIPTION: "#mic-status-description",
} as const;

const CSS_CLASSES = {
  // BEM modifiers for microphone control states
  MUTED: "mic-control__button--muted",
  HIDDEN: "mic-control__icon--hidden",
} as const;

const KEYBOARD_SHORTCUTS = {
  ESCAPE: "Escape",
  SPACE: "Space",
} as const;

const TAURI_COMMANDS = {
  GET_MICROPHONE_STATE: "get_microphone_state",
  TOGGLE_MICROPHONE: "toggle_microphone",
} as const;

const EVENTS = {
  MICROPHONE_STATE_CHANGED: "microphone-state-changed",
} as const;

// Type definitions
interface MicrophoneState {
  is_muted: boolean;
}

interface AppElements {
  micButton: HTMLButtonElement;
  micIcon: HTMLElement;
  micOffIcon: HTMLElement;
  closeButton: HTMLElement;
  draggableArea: HTMLElement;
  statusDescription: HTMLElement;
}

// Application state
class MicrophoneApp {
  private elements: AppElements | null = null;
  private unlisten: UnlistenFn | null = null;
  private readonly appWindow = getCurrentWindow();

  /**
   * Initialize the application
   */
  public async initialize(): Promise<void> {
    try {
      this.elements = this.getRequiredElements();
      this.setupEventListeners();
      await this.setupTauriEventListener();
      await this.loadInitialState();
    } catch (error) {
      console.error("Failed to initialize application:", error);
    }
  }

  /**
   * Clean up resources when the app is destroyed
   */
  public async cleanup(): Promise<void> {
    if (this.unlisten) {
      this.unlisten();
    }
  }

  /**
   * Get all required DOM elements and validate they exist
   */
  private getRequiredElements(): AppElements {
    const elements = {
      micButton: document.querySelector(ELEMENT_IDS.MIC_TOGGLE) as HTMLButtonElement,
      micIcon: document.querySelector(ELEMENT_IDS.MIC_ICON) as HTMLElement,
      micOffIcon: document.querySelector(ELEMENT_IDS.MIC_OFF_ICON) as HTMLElement,
      closeButton: document.querySelector(ELEMENT_IDS.CLOSE_BUTTON) as HTMLElement,
      draggableArea: document.querySelector(ELEMENT_IDS.DRAGGABLE_AREA) as HTMLElement,
      statusDescription: document.querySelector(ELEMENT_IDS.MIC_STATUS_DESCRIPTION) as HTMLElement,
    };

    // Validate all elements exist
    const missingElements = Object.entries(elements)
      .filter(([_, element]) => !element)
      .map(([name]) => name);

    if (missingElements.length > 0) {
      throw new Error(`Missing required elements: ${missingElements.join(', ')}`);
    }

    console.log("All required elements found:", {
      micButton: !!elements.micButton,
      micIcon: !!elements.micIcon,
      micOffIcon: !!elements.micOffIcon,
      closeButton: !!elements.closeButton,
      draggableArea: !!elements.draggableArea,
      statusDescription: !!elements.statusDescription,
    });

    return elements;
  }

  /**
   * Setup all DOM event listeners
   */
  private setupEventListeners(): void {
    if (!this.elements) return;

    // Microphone toggle functionality
    this.elements.micButton.addEventListener("click", () => this.handleMicrophoneToggle());

    // Window close functionality
    this.elements.closeButton.addEventListener("click", (e) => this.handleCloseWindow(e));

    // Window dragging functionality
    this.elements.draggableArea.addEventListener("mousedown", (e) => this.handleDragStart(e));

    // Keyboard shortcuts
    document.addEventListener("keydown", (e) => this.handleKeyboardShortcuts(e));
  }

  /**
   * Setup Tauri event listener for microphone state changes
   */
  private async setupTauriEventListener(): Promise<void> {
    this.unlisten = await listen<MicrophoneState>(
      EVENTS.MICROPHONE_STATE_CHANGED,
      (event) => this.updateMicrophoneUI(event.payload.is_muted)
    );
  }

  /**
   * Load and display the initial microphone state
   */
  private async loadInitialState(): Promise<void> {
    const initialMutedState = await this.getMicrophoneState();
    this.updateMicrophoneUI(initialMutedState);
  }

  /**
   * Update the UI based on microphone mute state
   */
  private updateMicrophoneUI(isMuted: boolean): void {
    if (!this.elements) return;

    const { micButton, micIcon, micOffIcon, statusDescription } = this.elements;

    if (isMuted) {
      micButton.classList.add(CSS_CLASSES.MUTED);
      micIcon.classList.add(CSS_CLASSES.HIDDEN);
      micOffIcon.classList.remove(CSS_CLASSES.HIDDEN);
      
      // Update accessibility attributes
      micButton.setAttribute('aria-label', 'Unmute microphone');
      micButton.setAttribute('title', 'Unmute microphone (Space)');
      statusDescription.textContent = 'Microphone is now muted';
    } else {
      micButton.classList.remove(CSS_CLASSES.MUTED);
      micIcon.classList.remove(CSS_CLASSES.HIDDEN);
      micOffIcon.classList.add(CSS_CLASSES.HIDDEN);
      
      // Update accessibility attributes
      micButton.setAttribute('aria-label', 'Mute microphone');
      micButton.setAttribute('title', 'Mute microphone (Space)');
      statusDescription.textContent = 'Microphone is now active';
    }
  }

  /**
   * Get current microphone state from backend
   */
  private async getMicrophoneState(): Promise<boolean> {
    try {
      const state: MicrophoneState = await invoke(TAURI_COMMANDS.GET_MICROPHONE_STATE);
      return state.is_muted;
    } catch (error) {
      console.error("Failed to get microphone state:", error);
      return false;
    }
  }

  /**
   * Toggle microphone mute state
   */
  private async handleMicrophoneToggle(): Promise<void> {
    try {
      const newState: MicrophoneState = await invoke(TAURI_COMMANDS.TOGGLE_MICROPHONE);
      this.updateMicrophoneUI(newState.is_muted);
    } catch (error) {
      console.error("Failed to toggle microphone:", error);
    }
  }

  /**
   * Handle window close button click
   */
  private async handleCloseWindow(event: Event): Promise<void> {
    event.preventDefault();
    event.stopPropagation();
    console.log("Close button clicked");
    
    try {
      await this.appWindow.close();
    } catch (error) {
      console.error("Failed to close window:", error);
    }
  }

  /**
   * Handle drag start for window movement
   */
  private handleDragStart(event: MouseEvent): void {
    const target = event.target as HTMLElement;
    const isCloseButton = target.closest(ELEMENT_IDS.CLOSE_BUTTON);
    const isMicButton = target.closest(ELEMENT_IDS.MIC_TOGGLE);
    
    // Only start dragging if we're not clicking on interactive elements
    if (!isCloseButton && !isMicButton) {
      console.log("Starting drag");
      this.appWindow.startDragging();
    }
  }

  /**
   * Handle keyboard shortcuts
   */
  private async handleKeyboardShortcuts(event: KeyboardEvent): Promise<void> {
    switch (event.key) {
      case KEYBOARD_SHORTCUTS.ESCAPE:
        console.log("Escape key pressed");
        try {
          await this.appWindow.close();
        } catch (error) {
          console.error("Failed to close window with Escape:", error);
        }
        break;
        
      case KEYBOARD_SHORTCUTS.SPACE:
        event.preventDefault();
        await this.handleMicrophoneToggle();
        break;
    }
  }
}

/**
 * Application entry point
 * Sets up the microphone control application when DOM is loaded
 */
window.addEventListener("DOMContentLoaded", async () => {
  try {
    const app = new MicrophoneApp();
    await app.initialize();
    
    // Store app instance globally for potential cleanup
    (window as any).microphoneApp = app;
    
    console.log("Snub application initialized successfully");
  } catch (error) {
    console.error("Failed to initialize Snub application:", error);
  }
});

/**
 * Handle page unload to clean up resources
 */
window.addEventListener("beforeunload", async () => {
  const app = (window as any).microphoneApp as MicrophoneApp;
  if (app) {
    await app.cleanup();
  }
});
