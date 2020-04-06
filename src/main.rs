use anyhow::Result;
use clap::{crate_authors, crate_version, App, Arg, SubCommand};
use gettext::Catalog;
use i18n_build::run;
use i18n_config::Crate;
use i18n_embed::I18nEmbed;
use rust_embed::RustEmbed;
use std::path::Path;
use tr::{set_translator, tr};

use unic_langid::LanguageIdentifier;

#[derive(RustEmbed, I18nEmbed)]
#[folder = "i18n/mo"]
struct Translations;

struct LanguageLoader;

impl LanguageLoader {
    fn new() -> LanguageLoader {
        LanguageLoader {}
    }
}

impl i18n_embed::LanguageLoader for LanguageLoader {
    fn load_language_file<R: std::io::Read>(&self, reader: R) {
        let catalog = Catalog::parse(reader).expect("could not parse the catalog");
        set_translator!(catalog);
    }

    fn module_path() -> &'static str {
        module_path!()
    }
}

/// Produce the message to be displayed when running `cargo i18n -h`.
fn short_about() -> String {
    // The help message displayed when running `cargo i18n -h`.
    tr!("A Cargo sub-command to extract and build localization resources.")
}

/// Produce the message to be displayed when running `cargo i18n --help`.
fn long_about() -> String {
    tr!(
        // The help message displayed when running `cargo i18n --help`.
        // {0} is the short about message.
        "{0}

This command reads the \"i18n.toml\" config in your crate root, \
and based on the configuration there, proceeds to extract \
localization resources, and build them.

If you are using the gettext localization system, you will \
need to have the following gettext tools installed: \"msgcat\", \
\"msginit\", \"msgmerge\" and \"msgfmt\". You will also need to have \
the \"xtr\" tool installed, which can be installed using \"cargo \
install xtr\".
",
        short_about()
    )
}

fn main() -> Result<()> {
    let loader = LanguageLoader::new();
    println!("Loading translations for cargo-gettext");
    Translations::desktop_select(&loader);

    println!("Loading translations for i18n_build");
    i18n_build::localize::<Translations>();

    let matches = App::new("cargo-i18n")
        .bin_name("cargo")
        .about(
            tr!(
                // The message displayed when running the binary outside of cargo using `cargo-18n`.
                "This binary is designed to be executed as a cargo subcommand using \"cargo i18n\".").as_str()
        )
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("i18n")
            .about(short_about().as_str())
            .long_about(long_about().as_str())
            .version(crate_version!())
            .author(crate_authors!())
            .arg(Arg::with_name("path")
                .help(
                    // The help message for the `--path` command line argument.
                    tr!("Path to the crate you want to localize (if not the current directory). The crate needs to contain \"i18n.toml\" in its root.").as_str()
                    )
                .long("path")
                .takes_value(true)
            )
            .arg(Arg::with_name("config file name")
                .help(
                    tr!(
                        // The help message for the `-c` command line argument.
                        "The name of the i18n config file for this crate").as_str()
                )
                .long("config-file-name")
                .short("c")
                .takes_value(true)
                .default_value("i18n.toml")
            )
        )
        .get_matches();

    if let Some(i18n_matches) = matches.subcommand_matches("i18n") {
        let config_file_name = i18n_matches
            .value_of("config file name")
            .expect("expected a config file name to be present");

        let path: Box<Path> = Box::from(Path::new("."));
        let config_file_path: Box<Path> = Box::from(Path::new(config_file_name));
        let crt = Crate::from(path, None, config_file_path).unwrap();

        run(&crt)?;
    }

    Ok(())
}
