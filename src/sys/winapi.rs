use crossterm_utils::Result;
use crossterm_winapi::{ConsoleMode, Handle, ScreenBuffer};
use winapi::shared::minwindef::DWORD;
use winapi::um::wincon;

use self::wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT};

/// This command is used for enabling and disabling raw mode for Windows systems.
/// For more info check: https://docs.microsoft.com/en-us/windows/console/high-level-console-modes.
#[derive(Clone, Copy)]
pub struct RawModeCommand {
    mask: DWORD,
}

impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand {
            mask: ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT,
        }
    }
}

impl RawModeCommand {
    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        let console_mode = ConsoleMode::from(Handle::input_handle()?);

        let dw_mode = console_mode.mode()?;

        let new_mode = dw_mode & !self.mask;

        console_mode.set_mode(new_mode)?;

        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&self) -> Result<()> {
        let console_mode = ConsoleMode::from(Handle::input_handle()?);

        let dw_mode = console_mode.mode()?;

        let new_mode = dw_mode | self.mask;

        console_mode.set_mode(new_mode)?;

        return Ok(());
    }
}
