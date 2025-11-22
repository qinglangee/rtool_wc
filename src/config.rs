use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct Config {
    pub ignore_chars: HashSet<char>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = "wc-config.txt";

        if !Path::new(config_path).exists() {
            return Config {
                ignore_chars: HashSet::new(),
            };
        }

        let mut ignore_chars = HashSet::new();

        if let Ok(content) = fs::read_to_string(config_path) {
            for line in content.lines() {
                let line = line.trim();
                // 跳过空行和注释行
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                // 添加每个字符到忽略集合
                for ch in line.chars() {
                    ignore_chars.insert(ch);
                }
            }
        }

        Config { ignore_chars }
    }
}
