# i18n-embed [![crates.io badge](https://img.shields.io/crates/v/i18n-embed.svg)](https://crates.io/crates/i18n-embed) [![license badge](https://img.shields.io/github/license/kellpossible/cargo-i18n)](https://github.com/kellpossible/cargo-i18n/blob/master/i18n-build/LICENSE.txt) [![docs.rs badge](https://docs.rs/i18n-embed/badge.svg)](https://docs.rs/i18n-embed/)

This library contains traits and macros to conveniently embed the output of [cargo-i18n](https://crates.io/crates/cargo_i18n) into your application binary in order to localize it at runtime.

Currently this library depends on [rust-embed](https://crates.io/crates/rust-embed) to perform the actual embedding of the language files. This may change in the future to make the library more convenient to use.

## Example

The following is an example for how to derive the required traits on structs, and localize your binary using this library:

```rust
use i18n_embed::{I18nEmbed, LanguageLoader, DesktopLanguageRequester};
use rust_embed::RustEmbed;

#[derive(RustEmbed, I18nEmbed)]
#[folder = "i18n/mo"] // path to the compiled localization resources
struct Translations;

#[derive(LanguageLoader)]
struct MyLanguageLoader;

fn main() {
    let language_loader = MyLanguageLoader {};

    // Use the language requester for the desktop platform (linux, windows, mac).
    // There is also a requester available for the web-sys WASM platform called
    // WebLanguageRequester, or you can implement your own.
    let language_requester = DesktopLanguageRequester::new();
    Translations::select(&language_requester, &language_loader);
}
```

For more examples, see the [documentation for i18n-embed](https://docs.rs/i18n-embed/).