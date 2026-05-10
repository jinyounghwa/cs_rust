use colored::Colorize;
use std::io::Write;
use std::process::Command;

pub fn execute_code(code: &str) {
    let mut child = Command::new("rustc")
        .args(["--edition", "2021", "-", "-o", "/tmp/cs_bite_out"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("rustc를 찾을 수 없습니다. Rust가 설치되어 있나요?");

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(code.as_bytes());
    }

    let output = child.wait_with_output().expect("컴파일 실패");

    if !output.status.success() {
        println!("{}", "  컴파일 에러:".bright_red().bold());
        for line in String::from_utf8_lossy(&output.stderr).lines() {
            println!("  {}", line.bright_red());
        }
        return;
    }

    let output = Command::new("/tmp/cs_bite_out")
        .output()
        .expect("실행 실패");

    if output.status.success() {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            println!("  {}", line.bright_white());
        }
    } else {
        println!("{}", "  실행 에러:".bright_red().bold());
        println!("{}", String::from_utf8_lossy(&output.stderr).bright_red());
    }
}
