use colored::*;
use unicode_width::UnicodeWidthStr;

/// 다이어그램을 비주얼 박스로 감싸서 출력
pub fn render_diagram(diagram: &str) {
    if diagram.is_empty() {
        return;
    }
    println!();
    println!("{}", "  📐 Visual Diagram".bright_cyan().bold());
    println!("{}", "  ┌──────────────────────────────────────────────────────────┐".bright_black());
    for line in diagram.lines() {
        let display_width = line.width();
        let padding = if display_width < 58 { " ".repeat(58 - display_width) } else { "".to_string() };
        println!("  {} {}{} {}", "│".bright_black(), colorize_diagram_line(line), padding, "│".bright_black());
    }
    println!("{}", "  └──────────────────────────────────────────────────────────┘".bright_black());
}

/// 다이어그램 라인에 컬러 적용
fn colorize_diagram_line(line: &str) -> String {
    // 화살표 색상
    let result = line
        .replace("◄──", &"◄──".bright_red().to_string())
        .replace("──►", &"──►".bright_blue().to_string())
        .replace("<─", &"<─".bright_red().to_string())
        .replace("─►", &"─►".bright_blue().to_string())
        .replace("←", &"←".bright_red().to_string())
        .replace("→", &"→".bright_blue().to_string())
        .replace("⇒", &"⇒".bright_blue().to_string())
        .replace("⇐", &"⇐".bright_red().to_string());

    // 강조 마커: [텍스트] 형태를 하이라이트
    let mut colored = String::new();
    let mut in_bracket = false;
    let mut buffer = String::new();

    for ch in result.chars() {
        match ch {
            '[' if !in_bracket => {
                in_bracket = true;
                buffer.clear();
            }
            ']' if in_bracket => {
                in_bracket = false;
                colored.push_str(&buffer.bright_yellow().bold().to_string());
                buffer.clear();
            }
            _ if in_bracket => {
                buffer.push(ch);
            }
            _ => {
                colored.push(ch);
            }
        }
    }
    if !buffer.is_empty() {
        colored.push_str(&buffer);
    }

    colored
}

/// 비교 표 렌더링: "header|left|right" 형식의 슬라이스를 테이블로 출력
pub fn render_comparisons(comparisons: &[&str]) {
    if comparisons.is_empty() {
        return;
    }

    let parts: Vec<Vec<&str>> = comparisons.iter().map(|s| s.split('|').collect()).collect();
    if parts.is_empty() || parts[0].len() < 3 {
        return;
    }

    // 컬럼 너비 계산 (아이콘 공간 고려)
    let left_w = parts.iter().map(|p| p[1].width()).max().unwrap_or(10).max(12) + 4;
    let right_w = parts.iter().map(|p| p[2].width()).max().unwrap_or(10).max(12) + 1;
    let mid_w = left_w + right_w + 1;

    println!();
    println!("{}", "  ⚖  Comparison Matrix".bright_magenta().bold());

    // 헤더
    let header = &parts[0];
    println!(
        "  {}",
        format!("┌─{:─^width$}─┐", "", width = mid_w).bright_black()
    );
    
    let h1 = header[1];
    let h2 = header[2];
    let h1_pad = " ".repeat(left_w.saturating_sub(h1.width()));
    let h2_pad = " ".repeat(right_w.saturating_sub(h2.width()));

    println!(
        "  {} {}{} {} {}{} {}",
        "│".bright_black(),
        h1.bright_white().bold(),
        h1_pad,
        "│".bright_black(),
        h2.bright_white().bold(),
        h2_pad,
        "│".bright_black(),
    );

    println!(
        "  {}",
        format!("├─{:─^width$}─┤", "", width = mid_w).bright_black()
    );

    // 데이터 행
    for row in parts.iter().skip(1) {
        if row.len() < 3 {
            continue;
        }
        let (icon, icon_color) = match row[0] {
            "left" => (" ◀ ", "blue"),
            "right" => (" ▶ ", "green"),
            "equal" => (" ◈ ", "yellow"),
            "diff" => (" ✦ ", "red"),
            "win" => (" ✓ ", "green"),
            "lose" => (" ✕ ", "red"),
            _ => ("   ", "white"),
        };
        
        let icon_colored = match icon_color {
            "blue" => icon.bright_blue(),
            "green" => icon.bright_green(),
            "yellow" => icon.bright_yellow(),
            "red" => icon.bright_red(),
            _ => icon.white(),
        }.to_string();

        let l_text = row[1];
        let r_text = row[2];
        let l_pad = " ".repeat(left_w.saturating_sub(l_text.width() + 3));
        let r_pad = " ".repeat(right_w.saturating_sub(r_text.width()));

        println!(
            "  {} {}{}{} {} {}{} {}",
            "│".bright_black(),
            icon_colored,
            l_text.cyan(),
            l_pad,
            "│".bright_black(),
            r_text.white(),
            r_pad,
            "│".bright_black(),
        );
    }

    println!(
        "  {}",
        format!("└─{:─^width$}─┘", "", width = mid_w).bright_black()
    );
}

/// 토픽 카테고리 이모지 반환
pub fn category_emoji(category: &str) -> &'static str {
    match category {
        "기초" => "🌱",
        "핵심" => "💎",
        "추상화" => "🧩",
        "동시성" => "⚡",
        "실전" => "🏗",
        _ => "📝",
    }
}

/// 프로그레스 바 출력
pub fn progress_bar(current: usize, total: usize, width: usize) -> String {
    let filled = if total > 0 { (current * width) / total } else { 0 };
    let empty = width - filled;
    format!(
        "{}{}",
        "█".repeat(filled).bright_cyan().to_string(),
        "░".repeat(empty).bright_black().to_string(),
    )
}
