use colored::*;
use crate::topic::CSTopic;
use crate::visual;

pub fn print_topic(topic: &CSTopic, idx: usize, total: usize) {
    let category_color = match topic.category {
        "기초" => topic.category.bright_green(),
        "핵심" => topic.category.bright_blue(),
        "추상화" => topic.category.bright_magenta(),
        "동시성" => topic.category.bright_yellow(),
        "실전" => topic.category.bright_cyan(),
        _ => topic.category.white(),
    };

    let emoji = visual::category_emoji(topic.category);

    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_black());
    println!(
        "  {} {} {}  {}",
        format!("[{}/{}]", idx, total).bright_black(),
        emoji,
        topic.title.bright_white().bold(),
        format!("[{}]", category_color).to_string(),
    );
    // 프로그레스 바
    println!(
        "  {}  {}",
        visual::progress_bar(idx, total, 40),
        format!("{}/{}", idx, total).bright_black(),
    );
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_black());

    // 개념 설명
    println!();
    println!("{}", "  💡 Concept".bright_cyan().bold());
    println!("{}", "  ──────────────────────────────────────────────────────".bright_black());
    for line in topic.explanation.lines() {
        println!("  {}", line);
    }

    // 왜 중요한가?
    println!();
    println!("{}", "  🎯 Why It Matters".bright_magenta().bold());
    println!("{}", "  ──────────────────────────────────────────────────────".bright_black());
    for line in topic.why_it_matters.lines() {
        println!("  {}", line);
    }

    // 비교 표 (있으면)
    visual::render_comparisons(topic.comparisons);

    // 다이어그램 (있으면)
    visual::render_diagram(topic.diagram);

    // 코드 예제
    println!();
    println!("{}", "  💻 Code Example".bright_green().bold());
    println!("{}", "  ┌──────────────────────────────────────────────────────────┐".bright_black());
    for line in topic.code.lines() {
        let display_width = line.len(); // Simple approximation for code
        let padding = if display_width < 58 { " ".repeat(58 - display_width) } else { "".to_string() };
        println!("  {} {}{} {}", "│".bright_black(), colorize_code_line(line), padding, "│".bright_black());
    }
    println!("{}", "  └──────────────────────────────────────────────────────────┘".bright_black());

    // 핵심 포인트
    println!();
    println!("{}", "  🔑 Key Takeaways".bright_yellow().bold());
    println!("{}", "  ──────────────────────────────────────────────────────".bright_black());
    for (i, point) in topic.key_points.iter().enumerate() {
        println!("  {} {} {}", "•".yellow(), point, format!("({})", i + 1).bright_black());
    }
    println!();
}

/// 코드 라인에 간단한 신택스 컬러링 적용
fn colorize_code_line(line: &str) -> String {
    let trimmed = line.trim_start();

    // 주석
    if trimmed.starts_with("//") {
        return line.bright_black().to_string();
    }

    let result = line
        // 키워드
        .replace("fn ", &"fn ".bright_purple().to_string())
        .replace("let ", &"let ".bright_purple().to_string())
        .replace("mut ", &"mut ".bright_purple().to_string())
        .replace("const ", &"const ".bright_purple().to_string())
        .replace("pub ", &"pub ".bright_purple().to_string())
        .replace("use ", &"use ".bright_purple().to_string())
        .replace("mod ", &"mod ".bright_purple().to_string())
        .replace("struct ", &"struct ".bright_purple().to_string())
        .replace("enum ", &"enum ".bright_purple().to_string())
        .replace("impl ", &"impl ".bright_purple().to_string())
        .replace("trait ", &"trait ".bright_purple().to_string())
        .replace("match ", &"match ".bright_purple().to_string())
        .replace("if ", &"if ".bright_purple().to_string())
        .replace("else ", &"else ".bright_purple().to_string())
        .replace("for ", &"for ".bright_purple().to_string())
        .replace("while ", &"while ".bright_purple().to_string())
        .replace("loop ", &"loop ".bright_purple().to_string())
        .replace("return ", &"return ".bright_purple().to_string())
        .replace("move ", &"move ".bright_purple().to_string())
        .replace("where ", &"where ".bright_purple().to_string())
        .replace("self", &"self".bright_purple().to_string())
        .replace("Some(", &"Some(".bright_blue().to_string())
        .replace("None", &"None".bright_blue().to_string())
        .replace("Ok(", &"Ok(".bright_blue().to_string())
        .replace("Err(", &"Err(".bright_blue().to_string())
        // 매크로
        .replace("println!", &"println!".bright_yellow().to_string())
        .replace("format!", &"format!".bright_yellow().to_string())
        .replace("vec![", &"vec![".bright_yellow().to_string())
        // 문자열
        .replace("\"", &"\"".bright_red().to_string())
        // 타입
        .replace("String", &"String".bright_cyan().to_string())
        .replace("Option", &"Option".bright_cyan().to_string())
        .replace("Result", &"Result".bright_cyan().to_string())
        .replace("Vec<", &"Vec<".bright_cyan().to_string())
        .replace("HashMap", &"HashMap".bright_cyan().to_string())
        .replace("bool", &"bool".bright_cyan().to_string())
        .replace("usize", &"usize".bright_cyan().to_string())
        .replace("i32", &"i32".bright_cyan().to_string())
        .replace("i64", &"i64".bright_cyan().to_string())
        .replace("u32", &"u32".bright_cyan().to_string())
        .replace("u8", &"u8".bright_cyan().to_string())
        .replace("f64", &"f64".bright_cyan().to_string())
        .replace("f32", &"f32".bright_cyan().to_string());

    result
}

pub fn list_topics(topics: &[CSTopic]) {
    println!();
    println!("{}", "  ╔══════════════════════════════════════════════════════╗".bright_black());
    println!("{}", "  ║  📚 cs-bite — Rust CS 학습 토픽 목록                ║".bright_black());
    println!("{}", "  ╚══════════════════════════════════════════════════════╝".bright_black());
    println!();

    let categories = [
        ("기초", "🌱", "언어의 첫걸음"),
        ("핵심", "💎", "Rust다운 코드"),
        ("추상화", "🧩", "다형성과 재사용"),
        ("동시성", "⚡", "병렬과 안전"),
        ("실전", "🏗", "실무 패턴"),
    ];

    for (cat, emoji, desc) in &categories {
        let topics_in_cat: Vec<(usize, &CSTopic)> = topics
            .iter()
            .enumerate()
            .filter(|(_, t)| t.category == *cat)
            .map(|(i, t)| (i, t))
            .collect();

        if !topics_in_cat.is_empty() {
            println!("  {} {} {} — {}", emoji, cat.bright_yellow().bold(), desc.bright_black(), format!("{}개", topics_in_cat.len()).bright_white());
            println!("  {}", "─".repeat(54).bright_black());
            for (i, topic) in &topics_in_cat {
                let has_diagram = if !topic.diagram.is_empty() { " 📐" } else { "" };
                let has_comparison = if !topic.comparisons.is_empty() { " ⚖" } else { "" };
                println!(
                    "    {:>2}. {}{}{}",
                    i + 1,
                    topic.title.cyan(),
                    has_diagram.bright_black(),
                    has_comparison.bright_black(),
                );
            }
            println!();
        }
    }

    println!("{}", "  ──────────────────────────────────────────────────────".bright_black());
    println!(
        "  {} {}  {} | {} | {}",
        "Total:".bright_white(),
        format!("{} topics", topics.len()).bright_cyan().bold(),
        "cs-bite".bright_white(),
        "cs-bite <num>".bright_black(),
        "cs-bite run".bright_black(),
    );
    println!();
}
