use crate::Error;

use std::io::{self, Write};
use std::path::PathBuf;

/// Prints a message to stderr, in the same format that `cargo` uses,
/// indicating that we are creating a bundle with the given filename.
pub fn print_bundling(filename: &str) -> Result<(), Error> {
    print_progress("Bundling", filename)
}

/// Prints a message to stderr, in the same format that `cargo` uses,
/// indicating that we have finished the the given bundles.
pub fn print_finished(output_paths: &Vec<PathBuf>) -> Result<(), Error> {
    let pluralised = if output_paths.len() == 1 {
        "bundle"
    } else {
        "bundles"
    };
    let msg = format!("{} {} at:", output_paths.len(), pluralised);
    print_progress("Finished", &msg)?;
    for path in output_paths {
        println!("        {}", path.display());
    }
    Ok(())
}

fn safe_term_attr<T: term::Terminal + ?Sized>(
    output: &mut Box<T>,
    attr: term::Attr,
) -> term::Result<()> {
    match output.supports_attr(attr) {
        true => output.attr(attr),
        false => Ok(()),
    }
}

pub fn print_progress(step: &str, msg: &str) -> Result<(), Error> {
    if let Some(mut output) = term::stderr() {
        safe_term_attr(&mut output, term::Attr::Bold)?;
        output.fg(term::color::GREEN)?;
        write!(output, "    {step}")?;
        output.reset()?;
        writeln!(output, " {msg}")?;
        output.flush()?;
        Ok(())
    } else {
        let mut output = io::stderr();
        write!(output, "    {step}")?;
        writeln!(output, " {msg}")?;
        output.flush()?;
        Ok(())
    }
}

/// Prints a warning message to stderr, in the same format that `cargo` uses.
pub fn print_warning(message: &str) -> Result<(), Error> {
    if let Some(mut output) = term::stderr() {
        safe_term_attr(&mut output, term::Attr::Bold)?;
        output.fg(term::color::YELLOW)?;
        write!(output, "warning:")?;
        output.reset()?;
        writeln!(output, " {message}")?;
        output.flush()?;
        Ok(())
    } else {
        let mut output = io::stderr();
        write!(output, "warning:")?;
        writeln!(output, " {message}")?;
        output.flush()?;
        Ok(())
    }
}

/// Prints an error to stderr, in the same format that `cargo` uses.
pub fn print_error(error: &Error) -> Result<(), Error> {
    if let Some(mut output) = term::stderr() {
        safe_term_attr(&mut output, term::Attr::Bold)?;
        output.fg(term::color::RED)?;
        write!(output, "error:")?;
        output.reset()?;

        safe_term_attr(&mut output, term::Attr::Bold)?;
        writeln!(output, " {error}")?;
        output.reset()?;

        output.flush()?;

        Ok(())
    } else {
        let mut output = io::stderr();
        write!(output, "error:")?;
        writeln!(output, " {error}")?;

        output.flush()?;
        Ok(())
    }
}
