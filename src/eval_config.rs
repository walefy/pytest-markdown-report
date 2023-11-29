use std::collections::HashMap;

pub struct EvalConfig {
    content: String,
}

pub trait EvalConfigTrait {
    fn read_emojis(&self) -> HashMap<String, String>;
    fn read_section(&self, section_prefix: &str) -> String;
    fn new(file_path: String) -> EvalConfig;
}

impl EvalConfigTrait for EvalConfig {
    fn read_emojis(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        for line in self.content.lines() {
            let infos_array = self.extract_info_line(line);
            let prefix: String;
            let key: String;
            let value: String;

            match infos_array {
                Ok(arr) => {
                    [prefix, key, value] = arr;
                }
                Err(_) => continue,
            }

            if prefix != "e" {
                continue;
            }

            result.insert(key, value);
        }

        result
    }

    fn read_section(&self, section_prefix: &str) -> String {
        let mut section_text = String::new();

        for line in self.content.lines() {
            let infos_array = self.extract_info_line(line);
            let prefix: String;
            let value: String;

            match infos_array {
                Ok(arr) => {
                    [prefix, _, value] = arr;
                }
                Err(_) => continue,
            }

            if prefix != section_prefix {
                continue;
            }

            section_text.push_str(format!("{}\n", value).as_str());
        }

        section_text
    }

    fn new(config_string: String) -> EvalConfig {
        let mut eval_config = EvalConfig {
            content: String::new(),
        };

        eval_config.content = config_string;

        eval_config
    }
}

trait PrivateEvalConfigTrait {
    fn extract_info_line(&self, line: &str) -> Result<[String; 3], &str>;
}

impl PrivateEvalConfigTrait for EvalConfig {
    fn extract_info_line(&self, line: &str) -> Result<[String; 3], &str> {
        let splitted_line: Vec<&str> = line.split('=').collect();

        if splitted_line.len() <= 1 {
            return Err("Empty line");
        }

        let prefix_and_key: Vec<&str> = splitted_line[0].split(' ').collect();
        let key = prefix_and_key[1].trim().to_string();
        let prefix = prefix_and_key[0].trim().to_string();
        let value = splitted_line[1].trim().to_string();

        Ok([prefix, key, value])
    }
}
