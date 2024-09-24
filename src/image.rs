use std::ffi::OsStr;
use std::path::Path;

/// Returns true if the path has a filename indicating that it is a high-density
/// "retina" icon.  Specifically, returns true the the file stem ends with
/// "@2x" (a convention specified by the [Apple developer docs](
/// https://developer.apple.com/library/mac/documentation/GraphicsAnimation/Conceptual/HighResolutionOSX/Optimizing/Optimizing.html)).
pub fn is_retina<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .file_stem()
        .and_then(OsStr::to_str)
        .map(|stem| stem.ends_with("@2x"))
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::is_retina;

    #[test]
    fn retina_icon_paths() {
        assert!(!is_retina("data/icons/512x512.png"));
        assert!(is_retina("data/icons/512x512@2x.png"));
    }
}
