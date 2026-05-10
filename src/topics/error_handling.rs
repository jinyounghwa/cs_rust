use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Error Handling 실전 패턴",
        category: "실전",
        explanation: "\
Rust의 실전 에러 처리 패턴을 정리합니다.

  1. thiserror crate: 커스텀 에러 타입 정의
     - derive 매크로로 보일러플레이트 감소
     - 에러 타입을 명확하게 문서화

  2. anyhow crate: 앱 코드에서 간편한 에러 처리
     - 에러 타입 통합 (anyhow::Error)
     - 컨텍스트 추가 (.context(\"설명\")?)

  3. 커스텀 에러 설계 원칙:
     - 라이브러리 → thiserror (명확한 에러 타입)
     - 애플리케이션 → anyhow (간편 처리)

  4. 패턴 매칭으로 에러 복구:
     - match로 특정 에러만 처리하고 나머지는 전파",
        why_it_matters: "\
NestJS에서 에러 처리는 try-catch + 커스텀 Exception이 일반적입니다.
Rust에서는 타입 시스템을 활용해 어떤 에러가 발생할 수 있는지
함수 시그니처로 명확히 알 수 있습니다.
실무에서 thiserror + anyhow 조합이 가장 많이 쓰입니다.",
        diagram: "\
  에러 처리 아키텍처
  ──────────────────────────────────

  라이브러리 crate:
  ┌─────────────────────────────────┐
  │ thiserror                       │
  │                                 │
  │ #[derive(Error, Debug)]         │
  │ enum AppError {                 │
  │   #[error(\"not found: {0}\")]    │
  │   NotFound(String),             │  ← 명확한 에러 타입
  │                                 │
  │   #[error(\"io: {0}\")]          │
  │   Io(#[from] io::Error),        │  ← 자동 From 변환
  │ }                               │
  └─────────────────────────────────┘

  애플리케이션 crate:
  ┌─────────────────────────────────┐
  │ anyhow                          │
  │                                 │
  │ fn main() -> Result<()> {       │
  │   let file = File::open(path)   │
  │     .context(\"설정 파일 열기\")?;│  ← 컨텍스트 추가
  │   Ok(())                        │
  │ }                               │
  └─────────────────────────────────┘

  에러 전파 체인:
  ──────────────────────────────────
  io::Error              →  AppError::Io     →  anyhow::Error
  (근본 원인)               (라이브러리 에러)     (앱 에러)

  file.open()              parse_config()      main()
  └── Error                 └── AppError        └── anyhow
      └── 원인                └── 컨텍스트         └── 최종 처리",
        code: r#"use std::fmt;
use std::num::ParseIntError;

// 수동 커스텀 에러 (thiserror 없이)
#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(ParseIntError),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ParseError(e) => write!(f, "Parse Error: {}", e),
            AppError::InvalidInput(msg) => write!(f, "Invalid: {}", msg),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// ? 연산자로 자동 변환 (From 덕분)
fn parse_config(input: &str) -> Result<i32, AppError> {
    let n: i32 = input.parse()?;  // ParseIntError → AppError 자동
    if n <= 0 {
        return Err(AppError::InvalidInput(format!("{} must be positive", n)));
    }
    Ok(n)
}

fn find_user(id: i32) -> Result<String, AppError> {
    match id {
        1 => Ok(String::from("Alice")),
        _ => Err(AppError::NotFound(format!("user {}", id))),
    }
}

fn main() {
    // 성공 케이스
    match parse_config("42") {
        Ok(n) => println!("Config value: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // 에러 케이스들
    println!("{:?}", parse_config("abc"));    // ParseError
    println!("{:?}", parse_config("-5"));     // InvalidInput
    println!("{:?}", find_user(99));          // NotFound

    // match로 선택적 복구
    match find_user(1) {
        Ok(name) => println!("Found: {}", name),
        Err(AppError::NotFound(msg)) => {
            println!("사용자 없음: {} → 기본값 사용", msg);
        }
        Err(e) => println!("다른 에러: {}", e),
    }
}
"#,
        key_points: &[
            "라이브러리: thiserror로 명확한 에러 타입 정의",
            "애플리케이션: anyhow로 간편한 에러 처리 + 컨텍스트",
            "From 트레이트 구현 → ? 연산자로 자동 에러 변환",
            "match로 에러 복구, ?로 에러 전파 — 상황에 맞게 선택",
        ],
        comparisons: &[
            "header|thiserror|anyhow",
            "diff|라이브러리용|애플리케이션용",
            "diff|명확한 에러 타입|통합 에러 타입",
            "diff|API 문서화에 좋음|빠른 프로토타이핑",
            "diff|#[derive(Error)]|anyhow::Result<T>",
        ],
    }
}
