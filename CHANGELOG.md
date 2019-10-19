# Version 0.3.2

- `to_alternate` switch back to main screen if it fails to switch into raw mode ([PR #4](https://github.com/crossterm-rs/crossterm-screen/pull/4))
- Improve the documentation ([PR #5](https://github.com/crossterm-rs/crossterm-screen/pull/5))
  - Public API
  - Include the book content in the documentation
- Remove all references to the crossterm book ([PR #6](https://github.com/crossterm-rs/crossterm-screen/pull/6))
- New commands introduced ([PR #7](https://github.com/crossterm-rs/crossterm-screen/pull/7))
  - `EnterAlternateScreen`
  - `LeaveAlternateScreen`
- Sync Windows and UNIX raw mode behavior ([PR #8](https://github.com/crossterm-rs/crossterm-screen/pull/8))

# Version 0.3.1

- Maintenance release only
- Moved to a [separate repository](https://github.com/crossterm-rs/crossterm-cursor)

# Version 0.3.0

- `RawScreen::into_raw_mode` returns `crossterm::Result` instead of `io::Result`
- `RawScreen::disable_raw_mode` returns `crossterm::Result` instead of `io::Result`
- `AlternateScreen::to_alternate` returns `crossterm::Result` instead of `io::Result`
- `AsyncReader::stop_reading()` to `stop()`
- `RawScreen::disable_raw_mode_on_drop` to `keep_raw_mode_on_drop`
