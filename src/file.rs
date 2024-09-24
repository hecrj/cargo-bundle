use crate::Error;

use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::{Component, Path, PathBuf};

/// Creates a new file at the given path, creating any parent directories as
/// needed.
pub fn create(path: &Path) -> Result<BufWriter<File>, Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}

#[cfg(unix)]
fn symlink_dir(src: &Path, dst: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn symlink_dir(src: &Path, dst: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_dir(src, dst)
}

#[cfg(unix)]
fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}

#[cfg(windows)]
fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_file(src, dst)
}

/// Copies a regular file from one path to another, creating any parent
/// directories of the destination path as necessary. Fails if the source path
/// is a directory or doesn't exist.
pub fn copy(from: &Path, to: &Path) -> Result<(), Error> {
    let dest_dir = to
        .parent()
        .expect("Destination should have a parent directory");

    fs::create_dir_all(dest_dir)?;
    fs::copy(from, to)?;
    Ok(())
}

/// Recursively copies a directory file from one path to another, creating any
/// parent directories of the destination path as necessary.  Fails if the
/// source path is not a directory or doesn't exist, or if the destination path
/// already exists.
pub fn copy_dir(from: &Path, to: &Path) -> Result<(), Error> {
    let dest_dir = to
        .parent()
        .expect("Destination should have a parent directory");

    fs::create_dir_all(dest_dir)?;

    for entry in walkdir::WalkDir::new(from) {
        let entry = entry?;
        debug_assert!(entry.path().starts_with(from));

        let rel_path = entry.path().strip_prefix(from).unwrap();
        let dest_path = to.join(rel_path);

        if entry.file_type().is_symlink() {
            let target = fs::read_link(entry.path())?;

            if entry.path().is_dir() {
                symlink_dir(&target, &dest_path)?;
            } else {
                symlink(&target, &dest_path)?;
            }
        } else if entry.file_type().is_dir() {
            fs::create_dir(dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }

    Ok(())
}

/// Given a path (absolute or relative) to a resource file, returns the
/// relative path from the bundle resources directory where that resource
/// should be stored.
pub fn resource_relpath(path: &Path) -> PathBuf {
    let mut dest = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Prefix(_) => {}
            Component::RootDir => dest.push("_root_"),
            Component::CurDir => {}
            Component::ParentDir => dest.push("_up_"),
            Component::Normal(string) => dest.push(string),
        }
    }

    dest
}

#[cfg(test)]
mod tests {
    use super::{copy_dir, create, resource_relpath, symlink};

    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn create_file_with_parent_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(!tmp.path().join("parent").exists());
        {
            let mut file = create(&tmp.path().join("parent/file.txt")).unwrap();
            writeln!(file, "Hello, world!").unwrap();
        }
        assert!(tmp.path().join("parent").is_dir());
        assert!(tmp.path().join("parent/file.txt").is_file());
    }

    #[test]
    #[cfg(not(windows))]
    fn copy_dir_with_symlinks() {
        // Create a directory structure that looks like this:
        //   ${TMP}/orig/
        //       sub/
        //           file.txt
        //       link -> sub/file.txt
        let tmp = tempfile::tempdir().unwrap();
        {
            let mut file = create(&tmp.path().join("orig/sub/file.txt")).unwrap();
            writeln!(file, "Hello, world!").unwrap();
        }
        symlink(
            &PathBuf::from("sub/file.txt"),
            &tmp.path().join("orig/link"),
        )
        .unwrap();
        assert_eq!(
            std::fs::read(tmp.path().join("orig/link"))
                .unwrap()
                .as_slice(),
            b"Hello, world!\n"
        );
        // Copy ${TMP}/orig to ${TMP}/parent/copy, and make sure that the
        // directory structure, file, and symlink got copied correctly.
        copy_dir(&tmp.path().join("orig"), &tmp.path().join("parent/copy")).unwrap();
        assert!(tmp.path().join("parent/copy").is_dir());
        assert!(tmp.path().join("parent/copy/sub").is_dir());
        assert!(tmp.path().join("parent/copy/sub/file.txt").is_file());
        assert_eq!(
            std::fs::read(tmp.path().join("parent/copy/sub/file.txt"))
                .unwrap()
                .as_slice(),
            b"Hello, world!\n"
        );
        assert!(tmp.path().join("parent/copy/link").exists());
        assert_eq!(
            std::fs::read_link(tmp.path().join("parent/copy/link")).unwrap(),
            PathBuf::from("sub/file.txt")
        );
        assert_eq!(
            std::fs::read(tmp.path().join("parent/copy/link"))
                .unwrap()
                .as_slice(),
            b"Hello, world!\n"
        );
    }

    #[test]
    fn resource_relative_paths() {
        assert_eq!(
            resource_relpath(&PathBuf::from("./data/images/button.png")),
            PathBuf::from("data/images/button.png")
        );
        assert_eq!(
            resource_relpath(&PathBuf::from("../../images/wheel.png")),
            PathBuf::from("_up_/_up_/images/wheel.png")
        );
        assert_eq!(
            resource_relpath(&PathBuf::from("/home/ferris/crab.png")),
            PathBuf::from("_root_/home/ferris/crab.png")
        );
    }
}
