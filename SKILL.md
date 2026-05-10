# SKILL.md — cs-bite 구현 스펙

## 프로젝트 구조

```
cs-bite/
├── Cargo.toml
├── CLAUDE.md
├── SKILL.md
├── src/
│   ├── main.rs          # CLI 진입점, 서브커맨드 파싱
│   ├── lesson.rs        # Lesson 구조체 + 출력 로직
│   ├── renderer.rs      # 터미널 컬러 렌더링
│   └── topics/
│       mod.rs           # 전체 토픽 레지스트리
│       01_ownership.rs  # 소유권
│       02_borrowing.rs  # 빌림
│       03_lifetimes.rs  # 라이프타임
│       04_async.rs      # async/await + Tokio
│       05_channels.rs   # mpsc 채널
│       06_eventloop.rs  # 이벤트 루프 비교 (Node vs Tokio)
│       07_hashmap.rs    # HashMap 내부 구조
│       08_btree.rs      # BTree — DB 인덱스 원리
│       09_postgres.rs   # PostgreSQL 연결 패턴 (sqlx)
│       10_sqlite.rs     # SQLite WAL 모드 (rusqlite)
│       11_pgvector.rs   # pgvector + HNSW 개념
│       12_redis.rs      # Redis 패턴 (redis-rs)
│       13_llm_stream.rs # LLM 스트리밍 SSE 패턴
│       14_axum.rs       # axum 경량 HTTP 서버
│       15_ipc.rs        # Unix socket IPC (로컬 AI 데몬)
└── examples/            # cargo run --example <name> 용
```

## Cargo.toml 의존성

```toml
[package]
name = "cs-bite"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "cs-bite"
path = "src/main.rs"

[dependencies]
colored = "2"          # 터미널 컬러 출력
rand = "0.8"           # 랜덤 토픽 선택
clap = { version = "4", features = ["derive"] }  # CLI 파싱
syntect = "5"          # Rust 코드 신택스 하이라이팅 (선택)
```

## Lesson 구조체

```rust
pub struct Lesson {
    pub id: u8,
    pub topic: &'static str,       // "Ownership"
    pub category: Category,        // Phase1 / Phase2 / Phase3
    pub concept: &'static str,     // 1~2줄 개념 설명
    pub code: &'static str,        // 실행 가능한 Rust 코드 전체
    pub key_points: [&'static str; 3], // 핵심 포인트 3줄
    pub estimated_minutes: u8,     // 읽기 예상 시간
}

pub enum Category {
    SystemsRust,   // Phase1: 소유권, 메모리, async
    DataStorage,   // Phase2: DB, 캐시 패턴
    LocalAI,       // Phase3: LLM 연동, IPC, 에이전트
}
```

## CLI 인터페이스

```
cs-bite              # 랜덤 Lesson 출력
cs-bite next         # 다음 순서 Lesson (id 기반 순환)
cs-bite run          # 현재 Lesson 코드를 tmp 파일로 저장 후 cargo run
cs-bite list         # 전체 토픽 목록 출력
cs-bite show <id>    # 특정 id Lesson 출력
```

## 터미널 출력 형식

```
╭─────────────────────────────────────────╮
│  📘 #03 · Lifetimes                     │
│  Phase 1 · Systems Rust  │  ~3분        │
╰─────────────────────────────────────────╯

[ 개념 ]
라이프타임은 참조가 유효한 범위를 컴파일러에게 명시하는 것.
GC 없이 메모리 안전성을 보장하는 Rust의 핵심 메커니즘.

[ 코드 ]
─────────────────────────────────────────
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("hello");
    let result;
    {
        let s2 = String::from("world!");
        result = longest(s1.as_str(), s2.as_str());
        println!("longest: {}", result);
    }
    // result는 여기서 사용 불가 — s2가 이미 drop됨
}
─────────────────────────────────────────

[ 핵심 포인트 ]
  ① 'a는 두 인자 중 더 짧은 라이프타임을 따른다
  ② 반환값의 라이프타임은 항상 입력에서 나와야 한다
  ③ 로컬 AI 버퍼 참조 설계 시 이 원칙이 직접 적용됨

╭─ cs-bite run  →  코드 직접 실행 ─────────╮
```

## cs-bite run 동작

```rust
// main.rs에서 run 서브커맨드 처리
fn run_lesson(lesson: &Lesson) -> Result<(), Box<dyn Error>> {
    let tmp = std::env::temp_dir().join("cs_bite_lesson.rs");
    std::fs::write(&tmp, lesson.code)?;
    // rustc 단독 컴파일 후 실행 (cargo 없이 빠르게)
    let out = tmp.with_extension("");
    std::process::Command::new("rustc")
        .args([tmp.to_str().unwrap(), "-o", out.to_str().unwrap()])
        .status()?;
    std::process::Command::new(out).status()?;
    Ok(())
}
```

## 토픽 파일 작성 규칙

각 `topics/NN_name.rs` 파일은 `Lesson` 상수 하나만 export:

```rust
// topics/01_ownership.rs
use crate::lesson::{Category, Lesson};

pub const LESSON: Lesson = Lesson {
    id: 1,
    topic: "Ownership — 값의 주인",
    category: Category::SystemsRust,
    concept: "Rust에서 모든 값은 단 하나의 소유자를 가진다.\n소유자가 scope를 벗어나면 값은 자동으로 drop된다.",
    code: r#"
fn take_ownership(s: String) {
    println!("got: {}", s);
} // s는 여기서 drop

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("{}", s1); // 컴파일 에러 — s1은 이미 move됨

    let x = 5;
    let y = x; // i32는 Copy trait — clone 불필요
    println!("x={}, y={}", x, y);
}
"#,
    key_points: [
        "String은 heap 할당 — move 시 소유권 이전, 원본 무효",
        "i32/bool 같은 Copy 타입은 자동 복사 — move 없음",
        "이 원리가 로컬 AI 버퍼 zero-copy 설계의 기반이 됨",
    ],
    estimated_minutes: 2,
};
```

## 구현 순서 (Claude Code 지시용)

```
1. cargo new cs-bite --bin
2. Cargo.toml 의존성 추가
3. src/lesson.rs — Lesson 구조체 정의
4. src/renderer.rs — colored crate 기반 출력 함수
5. src/topics/mod.rs — LESSONS 배열 ([&Lesson; N])
6. topics/01~05.rs — Phase 1 토픽 5개 먼저
7. src/main.rs — clap 기반 CLI (show/next/run/list)
8. topics/06~15.rs — Phase 2~3 토픽 추가
9. cargo install --path . → cs-bite 전역 등록
```

## 설치 후 사용 시나리오

```bash
# Claude Code 실행
$ claude "NestJS 서비스 레이어 리팩토링해줘"

# 다른 탭에서
$ cs-bite

# 2~5분 후 Claude Code 완료 — 결과 확인
```

## 확장 아이디어 (나중에)

- `cs-bite quiz` — 방금 본 코드에서 빈칸 채우기
- `cs-bite today` — 오늘 날짜 기반 고정 토픽 (반복 학습)
- 토픽 파일을 `~/.cs-bite/topics/` 에서 로드 — 커스텀 추가 가능
