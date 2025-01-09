pub use colored::*;
use regex::Regex;
pub use regex::RegexBuilder;
use std::fs;
pub use std::io::{self, Write};
pub use walkdir::WalkDir;

pub struct OptionCommand {
    pub max_number_of_lines: usize,
    pub ignore_case: bool,
    pub only_print_count_match: bool,
    pub regex_search: bool,
}
impl OptionCommand {
    pub fn new() -> Self {
        Self {
            max_number_of_lines: usize::MAX,
            ignore_case: false,
            only_print_count_match: false,
            regex_search: false,
        }
    }
}

pub const PROMPT: &str = "marianradu@Marian"; //ex de prompt

pub fn process_file(
    re: &Regex,
    entry: walkdir::DirEntry,
    count_found: &mut usize,
    op: &OptionCommand,
) -> Result<(), io::Error> {
    let path = entry.path();
    let content = fs::read_to_string(path)?;
    let pattern = re.to_string();
    let mut count = 0;
    println!("File: {}", entry.path().display());
    for (index, line) in content.lines().enumerate() {

        let is_match = if op.regex_search {
            re.is_match(line)
        } else if op.ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(&pattern)
        };

        if is_match {
            *count_found += 1;
            count += if op.regex_search {
                re.find_iter(line).count()
            } else if op.ignore_case {
                line.to_lowercase().matches(&pattern.to_lowercase()).count()
            } else {
                line.matches(&pattern).count()
            };

            if !op.only_print_count_match {
                let colored_text = re.replace_all(line, |caps: &regex::Captures| {
                    caps[0].red().bold().to_string() // colorez potrivirea (rosu si bold)
                });
                println!("[{}] \"{}\"", index+1, colored_text);
            }
        }

        if *count_found >= op.max_number_of_lines {
            break;
        }
    }

    if op.only_print_count_match && count > 0 {
        println!("Count: {}", count);
    }

    Ok(())
}

pub fn help() {
    print!(
        "usage:  {0} [{1}] [\"{2}\" | \"{2}\"]
        {3} => ignore case (case insensitive)
        {4} => only print the number of matches per file
        {5} => enable regex search
        {6} => nr is the maximum number of lines where the given string
        \tis found. The command ends when the number of matched lines reaches
        \tthe limit or when the entire directory is searched.",
        "rgrep".yellow().bold(),
        "-option".bold().blue(),
        "string".bold().blue(),
        "-i".bold().blue(),
        "-c".bold().blue(),
        "-r".bold().blue(),
        "-nr".bold().blue()
    );
    println!(
        "\n\t{0} [\"{1}\" | {1}]\n\t{2}\n\t{3}\n\t{4}",
        "setpath".yellow().bold(),
        "path".bold().blue(),
        "cls".yellow().bold(),
        "exit".yellow().bold(),
        "help".yellow().bold()
    );
}

pub fn print_info_rgrep() {
    println!(
        "usage:  {0} [{1}] [\"{2}\" | \"{2}\"]
        {3} => ignore case (case insensitive)
        {4} => only print the number of matches per file
        {5} => enable regex search
        {6} => nr is the maximum number of lines where the given string
        \tis found. The command ends when the number of matched lines reaches
        \tthe limit or when the entire directory is searched.",
        "rgrep".yellow().bold(),
        "-option".bold().blue(),
        "string".bold().blue(),
        "-i".bold().blue(),
        "-c".bold().blue(),
        "-r".bold().blue(),
        "-nr".bold().blue()
    );
}
