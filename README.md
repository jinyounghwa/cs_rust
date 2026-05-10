# cs-bite 🍬 — CLI 대기 시간 Rust CS 공부 도구

> Claude Code / AI CLI 실행 후 대기하는 2~5분 동안,
> Rust 코드 예제 기반 CS 개념을 터미널에 출력하는 학습 도구

```
╔══════════════════════════════════════════════════════════════╗
  [9/22] ⚙ Ownership (소유권) — Rust의 심장  [핵심]
  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  9/22
╚══════════════════════════════════════════════════════════════╝

  💡 개념 설명
  ────────────────────────────────────────────────────
  Rust에는 GC(가비지 컬렉터)가 없습니다. 대신 '소유권' 규칙으로 ...

  📐 시각화 다이어그램
  ┌─────────────────────────────────────────────────────────┐
  │ Move vs Copy — 핵심 시각화                                │
  │   ⚡ MOVE (Heap 데이터)                                   │
  │   BEFORE:                  AFTER:                         │
  │   ┌──────┐  ┌───────┐     ┌──────┐  ┌───────┐           │
  │   │ s1   │  │ Heap  │     │ s1   │  │ Heap  │           │
  │   │ ptr──┼─►│hello  │     │ ??   │  │hello  │           │
  │   └──────┘  └───────┘     │ ✗무효│  └───────┘           │
  │                           └──────┘                       │
  └─────────────────────────────────────────────────────────┘

  ⚖ 비교 표
  ┌────────────────────────────────────────────────┐
  │       Copy (자동)       │      Move (이동)       │
  ├────────────────────────────────────────────────┤
  │  i32, bool, char, f64   │  String, Vec, Box     │
  │  원본 유효               │  원본 무효             │
  │  비용 없음               │  비용 없음             │
  └────────────────────────────────────────────────┘

  🔧 코드 예제  ·  📌 핵심 포인트 4줄 정리
```

---

## 설치

```bash
git clone https://github.com/jinyounghwa/cs_rust.git
cd cs_rust
cargo build --release
./target/release/cs-bite
```

또는 cargo install:

```bash
cargo install --path .
cs-bite
```

## 사용법

| 명령 | 설명 |
|------|------|
| `cs-bite` | 랜덤 토픽 출력 |
| `cs-bite list` | 전체 토픽 목록 보기 |
| `cs-bite <번호>` | 지정한 토픽 출력 |
| `cs-bite run` | 랜덤 토픽 + 코드 직접 실행 |

## 웹 버전 🌐

별도 설치 없이 브라우저에서 바로 실행할 수 있습니다.

```bash
# 방법 1: 파일 직접 열기
open web/index.html          # macOS
xdg-open web/index.html      # Linux
start web/index.html         # Windows

# 방법 2: 로컬 서버 실행 (추천 — CORS 문제 방지)
cd web && python3 -m http.server 8080
# → http://localhost:8080 접속
```

| 기능 | 설명 |
|------|------|
| 사이드바 토픽 탐색 | 카테고리별 전체 토픽 목록 |
| 🎲 랜덤 토픽 | 버튼 또는 `R` 키로 랜덤 이동 |
| ◀ ▶ 이동 | 버튼 또는 `←` `→` 키로 이전/다음 토픽 |
| 📋 코드 복사 | 코드 블록 복사 버튼 |
| 📚 목록 보기 | 전체 토픽 한눈에 보기 |
| 반응형 | 모바일에서 사이드바 접힘 (☰ 토글) |

> `web/index.html` + `web/topics.js` 두 파일만 있으면 어디서든 실행 가능합니다.

---

## 22개 토픽 목록

### 🌱 기초 — 언어의 첫걸음
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 1 | 변수, 상수, 불변성 | `let` vs `let mut` vs `const`, 섀도잉 |
| 2 | 기본 타입 시스템 | i32, f64, char, 튜플, 배열, `as` 캐스팅 |
| 3 | 함수와 반환값 | 식 vs 문, 세미콜론의 의미, if 표현식 |
| 4 | String vs &str | Heap/String과 바이너리/&str, 슬라이스 |
| 5 | 제어 흐름 | if식, loop값반환, for..in, range |
| 6 | Vec\<T\>와 HashMap\<K,V\> | 동적배열, 해시맵, `get()` 안전접근 |

### ⚙ 핵심 — Rust다운 코드
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 7 | Struct & impl | 구조체, 메서드, 연관함수, derive |
| 8 | Enum & Pattern Matching | ADT, match 완전성, if let |
| 9 | Ownership | 소유권 3규칙, Move/Copy/Clone |
| 10 | Borrowing & References | &T vs &mut T, 빌림 규칙, NLL |
| 11 | Option\<T\> | null 대신 타입으로, map/and_then |
| 12 | Result\<T,E\> & 에러 처리 | `?` 연산자, map_err, 에러 전파 |

### 🧩 추상화 — 다형성과 재사용
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 13 | Traits | 트레이트 정의, impl/dyn, 기본구현 |
| 14 | Generics | 단형화, 트레이트 바운드, where |
| 15 | Closures & Iterators | Fn/FnMut/FnOnce, lazy 체인 |
| 16 | Lifetime ('a) | 참조 유효기간, elision, 'static |

### ⚡ 동시성 — 병렬과 안전
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 17 | Smart Pointers | Box, Rc, Arc, RefCell |
| 18 | Threads & Mutex | spawn, Arc\<Mutex\<T\>\>, Send/Sync |
| 19 | Channels | mpsc, 소유권 이전, Producer-Consumer |

### 🛠 실전 — 실무 패턴
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 20 | Module System | mod, use, pub, 가시성, 파일 매핑 |
| 21 | Testing | #[test], assert!, cargo test |
| 22 | Error Handling 실전 | thiserror, anyhow, 커스텀 에러 설계 |

## 비주얼 시스템

각 토픽은 6개의 섹션으로 구성됩니다:

| 섹션 | 설명 |
|------|------|
| 💡 **개념 설명** | 핵심 개념을 간결하게 설명 |
| 🎯 **왜 중요한가?** | NestJS/TS 경험과 연결하여 왜 배워야 하는지 |
| ⚖ **비교 표** | 관련 개념 간 비교 테이블 (Rust vs TS, 개념 간 차이) |
| 📐 **시각화 다이어그램** | 메모리 구조, 흐름도, 상태 변화를 ASCII 아트로 |
| 🔧 **코드 예제** | 실행 가능한 Rust 코드 (신택스 컬러링) |
| 📌 **핵심 포인트** | 4줄 요약 + 번호 |

## 프로젝트 구조

```
src/
├── main.rs              ← CLI 진입점 (인자 파싱, 실행 분기)
├── topic.rs             ← CSTopic 구조체 정의
├── display.rs           ← 터미널 출력 (print_topic, list_topics, 신택스 컬러링)
├── visual.rs            ← 비주얼 렌더링 (다이어그램, 비교표, 프로그레스바)
├── runner.rs            ← 코드 실행 (rustc 컴파일 & 실행)
└── topics/
    ├── mod.rs           ← all_topics()로 22개 토픽 취합
    ├── variables.rs     ← 각 토픽별 독립 파일
    ├── types.rs
    ├── functions.rs
    ├── strings.rs
    ├── control_flow.rs
    ├── collections.rs
    ├── structs.rs
    ├── enums.rs
    ├── ownership.rs
    ├── borrowing.rs
    ├── option.rs
    ├── result.rs
    ├── traits.rs
    ├── generics.rs
    ├── closures.rs
    ├── lifetimes.rs
    ├── smartpointers.rs
    ├── threads.rs
    ├── channels.rs
    ├── modules.rs
    ├── testing.rs
    └── error_handling.rs
```

새 토픽 추가: 파일 하나 만들고 `topics/mod.rs`에 두 줄 추가하면 끝.

## 특징

- ✨ 2~5분 안에 읽고 이해할 수 있는 분량
- 📐 아스키 다이어그램으로 메모리 구조·흐름 시각화
- ⚖ 개념 간 비교 테이블 (Rust vs TypeScript, 개념 간 차이)
- 🎯 실행 가능한 Rust 코드 예제 (rustc로 컴파일 & 실행)
- 📌 핵심 포인트 4줄 정리
- 🌈 컬러 터미널 출력 (카테고리별 색상, 프로그레스 바)
- 📚 NestJS 백엔드 경험자를 위한 비교 설명

## 개발

```bash
cargo build              # 디버그 빌드
cargo build --release    # 릴리스 빌드
cargo run                # 랜덤 토픽 실행
cargo run -- list        # 토픽 목록
cargo run -- 9           # 9번 토픽 (Ownership)
cargo run -- run         # 랜덤 토픽 + 코드 실행
```

## 라이센스

MIT
