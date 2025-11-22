use std::collections::HashSet;
// use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

/// 判断字符是否为 CJK 字符
fn is_cjk(ch: char) -> bool {
    let code = ch as u32;
    // CJK 统一表意文字范围
    (0x4E00..=0x9FFF).contains(&code) ||     // CJK Unified Ideographs
    (0x3400..=0x4DBF).contains(&code) ||     // CJK Unified Ideographs Extension A
    (0x20000..=0x2A6DF).contains(&code) ||   // CJK Unified Ideographs Extension B
    (0x2A700..=0x2B73F).contains(&code) ||   // CJK Unified Ideographs Extension C
    (0x2B740..=0x2B81F).contains(&code) ||   // CJK Unified Ideographs Extension D
    (0x2B820..=0x2CEAF).contains(&code) ||   // CJK Unified Ideographs Extension E
    (0xF900..=0xFAFF).contains(&code) ||     // CJK Compatibility Ideographs
    (0x2F800..=0x2FA1F).contains(&code)      // CJK Compatibility Ideographs Supplement
}

/// 判断字符是否为中文字符（更宽泛的定义）
fn is_chinese(ch: char) -> bool {
    is_cjk(ch)
}

/// 计算所有统计信息
pub fn count_all(content: &str) -> Stats {
    let lines = content.lines().count();
    let bytes = content.len();
    let words = count_words(content, &HashSet::new());

    Stats { lines, words, bytes }
}

/// 使用忽略字符集计算
pub fn count_with_ignore(content: &str, ignore_chars: &HashSet<char>) -> Stats {
    let lines = content.lines().count();
    let bytes = content.len();
    let words = count_words(content, ignore_chars);

    Stats { lines, words, bytes }
}

/// 只计算中文字符
pub fn count_chinese_only(content: &str) -> Stats {
    let lines = content.lines().count();
    let bytes = content.len();

    let words = content.chars().filter(|&ch| is_chinese(ch)).count();

    Stats { lines, words, bytes }
}

/// 计算单词数
/// - 英文单词以空白字符分隔
/// - CJK 字符每个字符算一个单词
fn count_words(content: &str, ignore_chars: &HashSet<char>) -> usize {
    let mut word_count = 0;
    let mut in_english_word = false;

    for ch in content.chars() {
        // 跳过忽略字符
        if ignore_chars.contains(&ch) {
            in_english_word = false;
            continue;
        }

        if is_cjk(ch) {
            // CJK 字符，每个字符算一个单词
            word_count += 1;
            in_english_word = false;
        } else if ch.is_whitespace() {
            // 空白字符，结束当前英文单词
            in_english_word = false;
        } else if ch.is_alphanumeric() || ch == '\'' || ch == '-' {
            // 英文字符、数字、撇号或连字符
            if !in_english_word {
                word_count += 1;
                in_english_word = true;
            }
        } else {
            // 其他标点符号等，结束当前英文单词
            in_english_word = false;
        }
    }

    word_count
}
