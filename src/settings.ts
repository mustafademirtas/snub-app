import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

// Interface for settings
interface Settings {
  telemetry_enabled: boolean;
}

// Get current window reference
const appWindow = getCurrentWindow();

// DOM elements
let closeButton: HTMLButtonElement;
let telemetryToggle: HTMLInputElement;
let settingsHeader: HTMLElement;

// Initialize the settings application
document.addEventListener('DOMContentLoaded', async () => {
  // Get DOM elements
  closeButton = document.getElementById('close-button') as HTMLButtonElement;
  telemetryToggle = document.getElementById('telemetry-toggle') as HTMLInputElement;
  settingsHeader = document.querySelector('.settings-header') as HTMLElement;

  // Validate required elements
  if (!closeButton || !telemetryToggle || !settingsHeader) {
    console.error('Required DOM elements not found');
    return;
  }

  // Load current settings
  await loadSettings();

  // Event listeners
  closeButton.addEventListener('click', handleCloseClick);
  telemetryToggle.addEventListener('change', handleTelemetryToggle);
  settingsHeader.addEventListener('mousedown', (event) => handleDragStart(event));

  // Keyboard shortcuts
  document.addEventListener('keydown', handleKeyDown);
});

/**
 * Load current settings from the backend
 */
async function loadSettings(): Promise<void> {
  try {
    const settings = await invoke<Settings>('get_settings');
    telemetryToggle.checked = settings.telemetry_enabled;
  } catch (error) {
    console.error('Failed to load settings:', error);
    // Set default values on error
    telemetryToggle.checked = false;
  }
}

/**
 * Handle telemetry toggle change
 */
async function handleTelemetryToggle(): Promise<void> {
  try {
    const enabled = telemetryToggle.checked;
    await invoke('set_telemetry_enabled', { enabled });
    console.log(`Telemetry ${enabled ? 'enabled' : 'disabled'}`);
  } catch (error) {
    console.error('Failed to update telemetry setting:', error);
    // Revert the toggle on error
    telemetryToggle.checked = !telemetryToggle.checked;
  }
}

/**
 * Handle close button click
 */
async function handleCloseClick(): Promise<void> {
  try {
    await appWindow.hide();
  } catch (error) {
    console.error('Failed to hide settings window:', error);
  }
}

/**
 * Handle drag start for window movement - only from header
 */
async function handleDragStart(event: MouseEvent): Promise<void> {
  const target = event.target as HTMLElement;
  const isCloseButton = target.closest('#close-button');
  
  // Only start dragging if we're clicking on the header and not on the close button
  if (!isCloseButton && target.closest('.settings-header')) {
    // Debug: Starting drag from header
    try {
      await appWindow.startDragging();
    } catch (error) {
      console.error('Failed to start dragging:', error);
    }
  }
}

/**
 * Handle keyboard shortcuts
 */
function handleKeyDown(event: KeyboardEvent): void {
  switch (event.key) {
    case 'Escape':
      event.preventDefault();
      handleCloseClick();
      break;
  }
}
