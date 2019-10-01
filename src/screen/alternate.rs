//! This module contains all the logic for switching between alternate screen and main screen.
//!
//! *Nix style applications often utilize an alternate screen buffer, so that they can modify the entire contents of the buffer, without affecting the application that started them.
//! The alternate buffer is exactly the dimensions of the window, without any scrollback region.
//! For an example of this behavior, consider when vim is launched from bash.
//! Vim uses the entirety of the screen to edit the file, then returning to bash leaves the original buffer unchanged.

#[cfg(windows)]
use crossterm_utils::supports_ansi;
use crossterm_utils::Result;

#[cfg(windows)]
use crate::sys::winapi::ToAlternateScreenCommand;
use crate::sys::{self, IAlternateScreenCommand};

use super::RawScreen;

/// An alternate screen.
///
/// With this type you will be able to switch to the alternate screen and then back to
/// the main screen.
///
/// Be aware that you'll be switched back to the main screen when you drop the
/// `AlternateScreen` value.
///
/// # Examples
///
/// Alternate screen with raw mode enabled:
///
/// ```no_run
/// use crossterm_screen::AlternateScreen;
/// use crossterm_utils::Result;
///
/// fn main() -> Result<()> {
///     let _alternate = AlternateScreen::to_alternate(true)?;
///
///     // Do something on the alternate screen in the raw mode
///
///     Ok(())
/// } // `_alternate` dropped here <- raw mode disabled & back to main screen
/// ```
pub struct AlternateScreen {
    #[cfg(windows)]
    command: Box<(dyn IAlternateScreenCommand + Sync + Send)>,
    #[cfg(unix)]
    command: sys::ToAlternateScreenCommand,
    _raw_screen: Option<RawScreen>,
}

impl AlternateScreen {
    /// Switches to the alternate screen.
    ///
    /// # Arguments
    ///
    /// * `raw_mode` - `true` enables the raw mode as well
    ///
    /// # Notes
    ///
    /// You'll be automatically switched to the main screen if this function
    /// fails.
    pub fn to_alternate(raw_mode: bool) -> Result<AlternateScreen> {
        #[cfg(windows)]
        let command = if supports_ansi() {
            Box::from(sys::ToAlternateScreenCommand::new())
                as Box<(dyn IAlternateScreenCommand + Sync + Send)>
        } else {
            Box::from(ToAlternateScreenCommand::new())
                as Box<(dyn IAlternateScreenCommand + Sync + Send)>
        };

        #[cfg(unix)]
        let command = sys::ToAlternateScreenCommand::new();

        command.enable()?;

        let mut alternate = AlternateScreen {
            command,
            _raw_screen: None,
        };

        if raw_mode {
            // If into_raw_mode fails, alternate will be dropped and
            // we'll switch back to the main screen.
            alternate._raw_screen = Some(RawScreen::into_raw_mode()?);
        }

        Ok(alternate)
    }

    /// Switches to the main screen.
    pub fn to_main(&self) -> Result<()> {
        self.command.disable()
    }
}

impl Drop for AlternateScreen {
    fn drop(&mut self) {
        let _ = self.to_main();
    }
}
