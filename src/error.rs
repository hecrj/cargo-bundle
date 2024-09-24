use std::path::PathBuf;
use std::process;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported bundle format: {0}")]
    UnsupportedBundle(String),

    #[error("`debug` profile is reserved")]
    DebugProfileIsReserved,

    #[error("glob error: {0}")]
    Glob(#[from] ::glob::GlobError),

    #[error("glob pattern error: {0}")]
    GlobPattern(#[from] ::glob::PatternError),

    #[error("io error: {0}")]
    Io(#[from] ::std::io::Error),

    #[error("image error: {0}")]
    Image(#[from] ::image::ImageError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("metadate error: {0}")]
    Metadata(#[from] cargo_metadata::Error),

    #[error("target error: {0}")]
    Target(#[from] ::target_build_utils::Error),

    #[error("terminal error: {0}")]
    Term(#[from] ::term::Error),

    #[error("toml error: {0}")]
    Toml(#[from] ::toml::de::Error),

    #[error("walkdir error: {0}")]
    Walkdir(#[from] ::walkdir::Error),

    #[error("`cargo build` failed with exist status: {0}")]
    BuildFailed(process::ExitStatus),

    #[error("unable to find root package")]
    RootPackageNotFound,

    #[error("no `bin` target is found in package '{0}'")]
    NoBinTargetFound(String),

    #[error("OS not supported: {0}")]
    OSNotSupported(String),

    #[error("framework path should have .framework extension: {0}")]
    MacosFrameworkNotValid(String),

    #[error("could not locate framework: {0}")]
    MacosFrameworkNotFound(String),

    #[error("no usable icon files found")]
    UsableIconFilesNotFound,

    #[error("unexpected directory: {0}")]
    UnexpectedDirectory(PathBuf),
}
