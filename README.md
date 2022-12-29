# systray-rs

I took the liberty to fix merge all the PR's of the parent project (minus 1).
I suggest using Tauri but if you want a non web desktop app, then it's possible you may find this library useful.

systray-rs is a Rust library heavily influenced by [the systray library for the Go Language](https://github.com/getlantern/systray) that makes it easy for applications to have minimal UI in a platform specific way. It wraps the platform specific calls required to show an icon in the system tray, as well as add menu entries.

[Parent issues](https://github.com/qdot/systray-rs/issues)

## Installation

```toml
systray = { git = "https://github.com/elibroftw/systray-rs", branch = "master" }
```

## Supported Platforms

systray-rs currently supports:

- Linux GTK
- Win32
- MacOS

Cocoa core still needed!

## Other Licenses

systray-rs includes some code from [winapi-rs, by retep998](https://github.com/retep998/winapi-rs).
This code is covered under the MIT license. This code will be removed once winapi-rs has a 0.3 crate available.

systray-rs includes some code from [rust-sysbar, by rust-sysbar](https://github.com/rust-sysbar/rust-sysbar).
This code is covered under the MIT license.
