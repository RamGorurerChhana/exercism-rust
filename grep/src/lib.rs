use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
///

#[derive(Debug)]
pub struct Flags<'a> {
    flags: &'a [&'a str],
}

impl<'a> Flags<'a> {
    pub fn new(flags: &'a [&str]) -> Self {
        Self { flags }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = if flags.flags.contains(&"-i") {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    let mut result = vec![];
    'outer: for file in files {
        let content = std::fs::read_to_string(file)?;
        for (line_no, line) in content.split('\n').enumerate() {
            if line.is_empty() {
                continue;
            }
            let line_converted = if flags.flags.contains(&"-i") {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            let is_match = (flags.flags.contains(&"-x") && line_converted == pattern)
                || (!flags.flags.contains(&"-x") && line_converted.contains(pattern.as_str()));

            if is_match ^ flags.flags.contains(&"-v") {
                if flags.flags.contains(&"-l") {
                    result.push(file.to_string());
                    continue 'outer;
                }

                let mut output = String::new();
                if files.len() > 1 {
                    output.push_str(file);
                    output.push(':');
                }
                if flags.flags.contains(&"-n") {
                    output.push_str((line_no + 1).to_string().as_str());
                    output.push(':');
                }
                output.push_str(line);
                result.push(output);
            }
        }
    }

    Ok(result)
}
