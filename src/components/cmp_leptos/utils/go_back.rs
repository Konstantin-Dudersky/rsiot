use web_sys::window;

/// Перейти назад по истории
pub fn go_back() -> Result<(), String> {
    window()
        .ok_or("Window is None")?
        .history()
        .map_err(|_| "History access trouble")?
        .back()
        .map_err(|_| "Back trouble")?;
    Ok(())
}
