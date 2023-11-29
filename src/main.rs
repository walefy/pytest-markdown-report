mod args;
mod eval_config;

use args::Args;
use clap::Parser;
use eval_config::{EvalConfig, EvalConfigTrait};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Error, Read};
use std::process::Command;

fn write_md(
    list: Vec<String>,
    map_optional_sections: HashMap<&str, String>,
    output_file_path: String,
) -> Result<(), Error> {
    let mut file = File::create(output_file_path)?;

    file.write(
        map_optional_sections
            .get("header")
            .unwrap_or(&"".to_string())
            .as_bytes(),
    )?;

    file.write(b"\n")?;
    file.write(b"*name* | *passed*\n")?;
    file.write(b"--- | :---:\n")?;
    file.write(list.join("\n").as_bytes())?;
    file.write(b"\n\n")?;

    file.write(
        map_optional_sections
            .get("footer")
            .unwrap_or(&"".to_string())
            .as_bytes(),
    )?;

    Ok(())
}

fn read_file(file_path: String) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut lines_string = String::new();

    file.read_to_string(&mut lines_string)?;

    Ok(lines_string)
}

fn get_or_default(map: &HashMap<String, String>, key: &str, default: String) -> String {
    map.get(&key.to_string()).unwrap_or(&default).to_string()
}

fn main() {
    let args: Args = Args::parse();
    let config_content = read_file(args.config_file).unwrap_or(String::new());
    let eval_config = EvalConfig::new(config_content);

    let config_in_file = eval_config.read_variables("c");

    let output: String = if args.no_auto {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut line = String::new();
        let _ = stdin.read_to_string(&mut line);

        line
    } else {
        let curr_dir = get_or_default(&config_in_file, "target-folder", args.target_folder);
        let command_output = Command::new("pytest")
            .current_dir(curr_dir)
            .args(["-v", "-W", "ignore::DeprecationWarning"])
            .output()
            .expect("failed to execute process");

        String::from_utf8(command_output.stdout).unwrap_or("".to_string())
    };

    let output_splitted = output.split('\n');
    let status_map = ["PASSED", "FAILED", "SKIPPED"];
    let mut emoji_map = HashMap::from([("passed", "✅"), ("failed", "❌"), ("skipped", "⚠️")]);
    let mut tests: Vec<String> = Vec::new();
    let mut map_optional_sections: HashMap<&str, String> = HashMap::new();

    let config_emojis_map = eval_config.read_variables("e");

    map_optional_sections.insert("header", eval_config.read_section("h"));
    map_optional_sections.insert("footer", eval_config.read_section("f"));

    for (key, value) in &config_emojis_map {
        emoji_map.insert(key.as_str(), value);
    }

    let selected_lines: Vec<&str> = output_splitted
        .filter(|line| status_map.iter().any(|status| line.contains(status)))
        .filter(|line| !line.starts_with("FAILED"))
        .collect();

    for line in selected_lines.into_iter() {
        let splitted_line: Vec<&str> = line.split(' ').collect();
        let test_name: &str = splitted_line[0].split("::").collect::<Vec<&str>>()[1];
        let test_passed: String = splitted_line[1].to_lowercase();
        let emoji = emoji_map.get(test_passed.as_str()).unwrap_or(&"❓");

        tests.push(format!("{} | {}", test_name, emoji));
    }

    let output = get_or_default(&config_in_file, "output", args.output);
    let result = write_md(tests, map_optional_sections, output);

    match result {
        Ok(_) => {}
        Err(_) => println!("Error in generate markdown"),
    };
}
