mod config;
mod counter;

use std::env;
use std::fs;
use std::process;

fn print_help() {
    println!("用法: zhwc [选项] <文件>");
    println!();
    println!("选项:");
    println!("  -i    忽略配置文件中指定的字符");
    println!("  -z    只计算中文字符");
    println!();
    println!("示例:");
    println!("  zhwc file.txt       # 统计文件的行数、字数和字节数");
    println!("  zhwc -i file.txt    # 统计时忽略配置的字符");
    println!("  zhwc -z file.txt    # 只统计中文字符");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let mut ignore_mode = false;
    let mut chinese_only = false;
    let mut filename = String::new();

    // 解析参数
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-i" => ignore_mode = true,
            "-z" => chinese_only = true,
            _ => filename = arg.clone(),
        }
    }

    if filename.is_empty() {
        print_help();
        return;
    }

    // 读取文件内容
    let content = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("错误: 无法读取文件 '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // 加载配置
    let config = config::Config::load();

    // 计算统计信息
    let stats = if chinese_only {
        counter::count_chinese_only(&content)
    } else if ignore_mode {
        counter::count_with_ignore(&content, &config.ignore_chars)
    } else {
        counter::count_all(&content)
    };

    // 输出结果 (类似 wc 的格式)
    println!(
        "{:8} {:8} {:8} [{}]",
        stats.lines, stats.words, stats.bytes, filename
    );
}
