use colored::*;

/// 다이어그램을 비주얼 박스로 감싸서 출력
pub fn render_diagram(diagram: &str) {
    if diagram.is_empty() {
        return;
    }
    println!();
    println!("{}", "  📐 시각화 다이어그램".bright_cyan().bold());
    println!("{}", "  ┌─────────────────────────────────────────────────────────┐".bright_black());
    for line in diagram.lines() {
        println!("  {} {:<57} {}", "│".bright_black(), colorize_diagram_line(line), "│".bright_black());
    }
    println!("{}", "  └─────────────────────────────────────────────────────────┘".bright_black());
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

    // 컬럼 너비 계산
    let left_w = parts.iter().map(|p| p[1].len()).max().unwrap_or(10).max(10);
    let right_w = parts.iter().map(|p| p[2].len()).max().unwrap_or(10).max(10);
    let mid_w = left_w + right_w + 7;

    println!();
    println!("{}", "  ⚖ 비교 표".bright_magenta().bold());

    // 헤더
    let header = &parts[0];
    println!(
        "  {}",
        format!("┌─{:─^width$}─┐", "", width = mid_w).bright_black()
    );
    println!(
        "  {} {:^left_w$} {} {:^right_w$} {}",
        "│".bright_black(),
        header[1].bright_white().bold(),
        "│".bright_black(),
        header[2].bright_white().bold(),
        "│".bright_black(),
        left_w = left_w + 1,
        right_w = right_w + 1,
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
        let left_icon = match row[0] {
            "left" => " ◀".bright_blue().to_string(),
            "right" => " ▶".bright_green().to_string(),
            "equal" => " ■".bright_yellow().to_string(),
            "diff" => " ✦".bright_red().to_string(),
            "win" => " ✔".bright_green().to_string(),
            "lose" => " ✘".bright_red().to_string(),
            _ => "  ".to_string(),
        };
        println!(
            "  {} {}{} {} {}{} {}",
            "│".bright_black(),
            left_icon,
            row[1].cyan(),
            "│".bright_black(),
            " ",
            row[2].white(),
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
        "핵심" => "⚙",
        "추상화" => "🧩",
        "동시성" => "⚡",
        "실전" => "🛠",
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
