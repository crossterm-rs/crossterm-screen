![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] [![Join us on Discord][s5]][l5]

# Crossterm Screen

**The `crossterm_screen` crate is deprecated and no longer maintained. The GitHub repository will
be archived soon. All the code is being moved to the `crossterm`
[crate](https://github.com/crossterm-rs/crossterm). You can learn more in the
[Merge sub-crates to the crossterm crate](https://github.com/crossterm-rs/crossterm/issues/265)
issue.**

This crate allows you to work with alternate and raw screen cross-platform. 
It supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested
see [Tested Terminals](https://github.com/crossterm-rs/crossterm/blob/master/README.md#tested-terminals) for more info)

`crossterm_screen` is a sub-crate of the [crossterm](https://crates.io/crates/crossterm) crate. You can use it
directly, but it's **highly recommended** to use the [crossterm](https://crates.io/crates/crossterm) crate with
the `screen` feature enabled.

## Getting Started

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
# All crossterm features are enabled by default.
crossterm = "0.11"
```

</details>
<p></p>

```rust
use crossterm::RawScreen;

fn main() -> Result<()> {
    // Enable raw mode
    let _raw = RawScreen::into_raw_mode()?;

    // Do something in the raw mode
} // `_raw` dropped here <- raw mode is disabled
```

## Future

> The `crossterm_screen` crate code will be moved to the `crossterm` crate (it's re-exported there now).
> Date is not set yet, but it doesn't make a lot of sense to start a new project with it. Please, use
> the `crossterm` crate with the `screen` feature enabled.

Issues in this repository are disabled for the same reason. Please, report all issues in the
[crossterm-rs/crossterm](https://github.com/crossterm-rs/crossterm/issues) repository.
  
## Features

- Cross-platform
- Multi-threaded (send, sync)
- Detailed documentation
- Few dependencies
- Alternate screen
- Raw screen   

## Other Resources

- [API documentation](https://docs.rs/crossterm_screen/) (with other examples)
- [Examples repository](https://github.com/crossterm-rs/examples)
    
## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

[s1]: https://img.shields.io/crates/v/crossterm_screen.svg
[l1]: https://crates.io/crates/crossterm_screen

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_screen/badge.svg
[l3]: https://docs.rs/crossterm_screen/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/crossterm-rs/crossterm.svg?branch=master
