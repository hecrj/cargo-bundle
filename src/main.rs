mod bundle;
mod category;
mod error;
mod file;
mod image;
mod terminal;

use bundle::Bundle;
use category::Category;
use error::Error;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::ffi::OsString;
use std::process;

fn build_project_if_unbuilt(settings: &bundle::Settings) -> Result<(), Error> {
    if std::env::var("CARGO_BUNDLE_SKIP_BUILD").is_ok() {
        return Ok(());
    }

    let mut cargo =
        process::Command::new(env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")));
    cargo.arg("build");

    if let Some(triple) = settings.target_triple() {
        cargo.arg(format!("--target={triple}"));
    }

    if let Some(features) = settings.features() {
        cargo.arg(format!("--features={features}"));
    }

    match settings.build_artifact() {
        bundle::BuildArtifact::Main => {}
        bundle::BuildArtifact::Bin(name) => {
            cargo.arg(format!("--bin={name}"));
        }
        bundle::BuildArtifact::Example(name) => {
            cargo.arg(format!("--example={name}"));
        }
    }

    match settings.build_profile() {
        "dev" => {}
        "release" => {
            cargo.arg("--release");
        }
        custom => {
            cargo.arg("--profile");
            cargo.arg(custom);
        }
    }

    if settings.all_features() {
        cargo.arg("--all-features");
    }

    if settings.no_default_features() {
        cargo.arg("--no-default-features");
    }

    let status = cargo.status()?;

    if !status.success() {
        return Err(Error::BuildFailed(status));
    }

    Ok(())
}

fn run() -> Result<(), Error> {
    let all_formats: Vec<&str> = Bundle::ALL.iter().map(Bundle::short_name).collect();

    let m = App::new("cargo-bundle")
        .version(format!("v{}", env!("CARGO_PKG_RUST_VERSION")).as_str())
        .bin_name("cargo")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("bundle")
                .author("George Burton <burtonageo@gmail.com>")
                .about("Bundle Rust executables into OS bundles")
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::UnifiedHelpMessage)
                .arg(
                    Arg::with_name("bin")
                        .long("bin")
                        .value_name("NAME")
                        .help("Bundle the specified binary"),
                )
                .arg(
                    Arg::with_name("example")
                        .long("example")
                        .value_name("NAME")
                        .conflicts_with("bin")
                        .help("Bundle the specified example"),
                )
                .arg(
                    Arg::with_name("format")
                        .long("format")
                        .value_name("FORMAT")
                        .possible_values(&all_formats)
                        .help("Which bundle format to produce"),
                )
                .arg(
                    Arg::with_name("release")
                        .long("release")
                        .help("Build a bundle from a target built in release mode"),
                )
                .arg(
                    Arg::with_name("profile")
                        .long("profile")
                        .value_name("NAME")
                        .conflicts_with("release")
                        .help("Build a bundle from a target build using the given profile"),
                )
                .arg(
                    Arg::with_name("target")
                        .long("target")
                        .value_name("TRIPLE")
                        .help("Build a bundle for the target triple"),
                )
                .arg(
                    Arg::with_name("features")
                        .long("features")
                        .value_name("FEATURES")
                        .help("Set crate features for the bundle. Eg: `--features \"f1 f2\"`"),
                )
                .arg(
                    Arg::with_name("all-features")
                        .long("all-features")
                        .help("Build a bundle with all crate features."),
                )
                .arg(
                    Arg::with_name("no-default-features")
                        .long("no-default-features")
                        .help("Build a bundle without the default crate features."),
                ),
        )
        .get_matches();

    if let Some(m) = m.subcommand_matches("bundle") {
        let output_paths = env::current_dir()
            .map_err(From::from)
            .and_then(|d| bundle::Settings::new(d, m))
            .and_then(|s| {
                build_project_if_unbuilt(&s)?;
                Ok(s)
            })
            .and_then(bundle::run)?;

        terminal::print_finished(&output_paths)?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        terminal::print_error(&error).unwrap();
        std::process::exit(1);
    }
}
