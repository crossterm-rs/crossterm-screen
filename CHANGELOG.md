# Version 0.3.1

- Maintenance release only
- Moved to a [separate repository](https://github.com/crossterm-rs/crossterm-cursor)

# Version 0.3.0

- `RawScreen::into_raw_mode` returns `crossterm::Result` instead of `io::Result`
- `RawScreen::disable_raw_mode` returns `crossterm::Result` instead of `io::Result`
- `AlternateScreen::to_alternate` returns `crossterm::Result` instead of `io::Result`
- `AsyncReader::stop_reading()` to `stop()`
- `RawScreen::disable_raw_mode_on_drop` to `keep_raw_mode_on_drop`