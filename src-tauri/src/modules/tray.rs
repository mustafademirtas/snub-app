use crate::modules::types::{TRAY_ID, MENU_TOGGLE_MIC, MENU_SHOW_WINDOW, MENU_QUIT, TOOLTIP_MUTED, TOOLTIP_ACTIVE};
use std::error::Error;
use tauri::{
    menu::{Menu, MenuItem},
    Runtime,
};

/// Creates a tray icon based on the microphone state
pub fn create_icon(is_muted: bool) -> Result<tauri::image::Image<'static>, Box<dyn Error>> {
    let icon_bytes: Box<[u8]> = if is_muted {
        include_bytes!("../../icons/tray-icon-muted.png").to_vec().into_boxed_slice()
    } else {
        include_bytes!("../../icons/tray-icon.png").to_vec().into_boxed_slice()
    };
    
    let image = image::load_from_memory(&icon_bytes)?;
    let rgba = image.to_rgba8();
    let (width, height) = (image.width(), image.height());
    
    Ok(tauri::image::Image::new_owned(rgba.into_raw(), width, height))
}

/// Updates the tray icon and tooltip based on microphone state
pub fn update_icon<R: Runtime>(app: &tauri::AppHandle<R>, is_muted: bool) -> Result<(), Box<dyn Error>> {
    let tray = app.tray_by_id(TRAY_ID).ok_or("Tray not found")?;
    
    let icon = create_icon(is_muted)?;
    tray.set_icon(Some(icon))?;
    
    let tooltip = if is_muted { TOOLTIP_MUTED } else { TOOLTIP_ACTIVE };
    tray.set_tooltip(Some(tooltip))?;
    
    Ok(())
}

/// Creates the tray context menu
pub fn create_menu<R: Runtime>(app: &tauri::AppHandle<R>, is_muted: bool) -> Result<Menu<R>, Box<dyn Error>> {
    let menu_text = if is_muted { "Unmute Mic" } else { "Mute Mic" };
    
    let toggle_item = MenuItem::with_id(app, MENU_TOGGLE_MIC, menu_text, true, None::<&str>)?;
    let show_item = MenuItem::with_id(app, MENU_SHOW_WINDOW, "Show Window", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, MENU_QUIT, "Quit", true, None::<&str>)?;
    
    let menu = Menu::new(app)?;
    menu.append(&toggle_item)?;
    menu.append(&show_item)?;
    menu.append(&quit_item)?;
    
    Ok(menu)
}

/// Updates the tray menu text based on microphone state
pub fn update_menu<R: Runtime>(app: &tauri::AppHandle<R>, is_muted: bool) -> Result<(), Box<dyn Error>> {
    let tray = app.tray_by_id(TRAY_ID).ok_or("Tray not found")?;
    let new_menu = create_menu(app, is_muted)?;
    tray.set_menu(Some(new_menu))?;
    Ok(())
}

/// Updates both tray icon and menu
pub fn update_tray_state<R: Runtime>(app: &tauri::AppHandle<R>, is_muted: bool) -> Result<(), Box<dyn Error>> {
    update_icon(app, is_muted)?;
    update_menu(app, is_muted)?;
    Ok(())
}
