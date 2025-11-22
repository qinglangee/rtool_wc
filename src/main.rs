mod config;
mod counter;

use std::env;
use std::fs;
use std::process;
use glob::glob;

fn print_help() {
    println!("用法: zhwc [选项] <文件...>");
    println!("输出：行数   字数   字节数   [文件名]");
    println!("配置文件：wc-config.txt");
    println!();
    println!("选项:");
    println!("  -i    忽略配置文件中指定的字符");
    println!("  -z    只计算中文字符");
    println!();
    println!("示例:");
    println!("  zhwc file.txt           # 统计文件的行数、字数和字节数");
    println!("  zhwc -i file.txt        # 统计时忽略配置的字符");
    println!("  zhwc -z file.txt        # 只统计中文字符");
    println!("  zhwc *.txt              # 统计多个文件");
    println!("  zhwc file1.txt file2.txt # 统计多个文件");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut ignore_mode = false;
    let mut chinese_only = false;
    let mut file_patterns: Vec<String> = Vec::new();

    // 解析参数
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-i" => ignore_mode = true,
            "-z" => chinese_only = true,
            _ => file_patterns.push(arg.clone()),
        }
    }

    if file_patterns.is_empty() {
        print_help();
        return;
    }

    // 加载配置
    let config = config::Config::load();

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut file_count = 0;

    // 展开所有文件模式并处理每个文件
    let mut all_files: Vec<String> = Vec::new();

    for pattern in file_patterns {
        // 尝试作为通配符展开
        match glob(&pattern) {
            Ok(paths) => {
                let mut found_any = false;
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            if path.is_file() {
                                all_files.push(path.to_string_lossy().to_string());
                                found_any = true;
                            }
                        }
                        Err(e) => eprintln!("错误: 处理路径时出错: {}", e),
                    }
                }
                // 如果通配符没有匹配到文件，尝试作为普通文件名
                if !found_any {
                    all_files.push(pattern);
                }
            }
            Err(_) => {
                // 不是有效的通配符，作为普通文件名处理
                all_files.push(pattern);
            }
        }
    }

    // 处理每个文件
    for filename in &all_files {
        // 读取文件内容
        let content = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("错误: 无法读取文件 '{}': {}", filename, e);
                continue;
            }
        };

        // 计算统计信息
        let stats = if chinese_only {
            counter::count_chinese_only(&content)
        } else if ignore_mode {
            counter::count_with_ignore(&content, &config.ignore_chars)
        } else {
            counter::count_all(&content)
        };

        // 输出结果
        println!(
            "{:8} {:8} {:8} [{}]",
            stats.lines, stats.words, stats.bytes, filename
        );

        // 累加总数
        total_lines += stats.lines;
        total_words += stats.words;
        total_bytes += stats.bytes;
        file_count += 1;
    }

    // 如果处理了多个文件，输出总计
    if file_count > 1 {
        println!(
            "{:8} {:8} {:8} [总计]",
            total_lines, total_words, total_bytes
        );
    }

    if file_count == 0 {
        eprintln!("错误: 没有找到可处理的文件");
        process::exit(1);
    }
}
