use clap::{self, ArgAction, Parser};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::process::{Command, Output};
use std::str::Split;

#[derive(Parser, Debug)]
#[command(author = "Walefy", version = "0.0.1", about = "pytest markdown report", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "report.md")]
    output: String,

    #[arg(long, action = ArgAction::SetTrue)]
    /// Turn off auto run command pytest.
    ///
    /// Usage example:
    /// ```
    /// pytest -v > pytest_markdown_report --no-auto
    /// ```
    no_auto: bool,

    #[arg(short, long, default_value = "./")]
    /// directory where the tests are
    target_folder: String,
}

fn write_md(list: Vec<String>, output_file_path: String) -> Result<()> {
    let mut file = File::create(output_file_path)?;

    file.write(b"*name* | *passed*\n")?;
    file.write(b"--- | :---:\n")?;
    file.write(list.join("\n").as_bytes())?;
    file.write(b"\n")?;

    Ok(())
}

fn main() {
    let args: Args = Args::parse();

    let output: Output = Command::new("pytest")
        .current_dir(args.target_folder)
        .args(["-v", "-W", "ignore::DeprecationWarning"])
        .output()
        .expect("failed to execute process");

    let output_str: String = String::from_utf8(output.stdout).unwrap();
    let output_splitted: Split<'_, char> = output_str.split('\n');
    let status_map: Vec<&str> = vec!["PASSED", "FAILED", "SKIPPED"];
    let mut tests: Vec<String> = Vec::new();

    let emoji_map: HashMap<&str, &str> = [("passed", "✅"), ("failed", "❌"), ("skipped", "⚠️")]
        .iter()
        .cloned()
        .collect();

    let selected_lines: Vec<&str> = output_splitted
        .filter(|line: &&str| status_map.iter().any(|status| line.contains(status)))
        .filter(|line: &&str| !line.starts_with("FAILED"))
        .collect();

    for line in selected_lines.into_iter() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        let test_name: &str = splitted_line[0].split("::").collect::<Vec<&str>>()[1];
        let test_passed: String = splitted_line[1].to_lowercase();
        let emoji: &&str = emoji_map.get(&test_passed.as_str()).unwrap_or(&"❓");

        tests.push(format!("{} | {}", test_name, emoji));
    }

    let result = write_md(tests, args.output);

    match result {
        Ok(_) => {}
        Err(_) => println!("Error in generate markdown"),
    };
}
