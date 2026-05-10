mod topic;
mod topics;
mod display;
mod visual;
mod runner;

use colored::Colorize;
use std::env;
use rand::Rng;

fn main() {
    let all = topics::all_topics();
    let total = all.len();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_random_topic(&all, total);
    } else {
        match args[1].as_str() {
            "run" => run_topic_with_code(&all, total),
            "list" => display::list_topics(&all),
            n if n.parse::<usize>().is_ok() => {
                let idx = n.parse::<usize>().unwrap();
                show_topic_by_index(&all, idx, total);
            }
            _ => show_random_topic(&all, total),
        }
    }
}

fn show_random_topic(topics: &[topic::CSTopic], total: usize) {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..topics.len());
    display::print_topic(&topics[idx], idx + 1, total);
}

fn show_topic_by_index(topics: &[topic::CSTopic], idx: usize, total: usize) {
    if idx == 0 || idx > topics.len() {
        println!("{}", format!("번호는 1~{} 사이로 입력하세요.", topics.len()).bright_red());
        return;
    }
    display::print_topic(&topics[idx - 1], idx, total);
}

fn run_topic_with_code(topics: &[topic::CSTopic], total: usize) {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..topics.len());
    let topic = &topics[idx];

    display::print_topic(topic, idx + 1, total);
    println!("\n{}", "=".repeat(62).cyan());
    println!("{}", "  🚀 코드 실행 중...".bright_yellow().bold());
    println!("{}", "=".repeat(62).cyan());

    runner::execute_code(topic.code);
}
