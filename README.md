# cs-bite — CLI & Web Rust CS 학습 도구

> Rust 코드 예제 기반 CS 개념을 **터미널 또는 브라우저**에서 한 입 크기로 학습하는 도구.  
> 대기 시간 2~5분, 실행할 때마다 새로운 토픽.

```
╔══════════════════════════════════════════════════════════════╗
  [9/38] 💎 Ownership (소유권) — Rust의 심장  [핵심]
  ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  9/38
╚══════════════════════════════════════════════════════════════╝

  💡 Concept
  ──────────────────────────────────────────────────────
  Rust에는 GC(가비지 컬렉터)가 없습니다. 대신 '소유권' 규칙으로 ...

  📐 Visual Diagram
  ┌──────────────────────────────────────────────────────────┐
  │ Move vs Copy — 핵심 시각화                                │
  │   BEFORE:  s1 → Heap("hello")   AFTER: s1 ✗ 무효        │
  └──────────────────────────────────────────────────────────┘

  ⚖  Comparison Matrix
  ┌────────────────────────────────────┐
  │ Box<T>          │ Rc<T>           │
  ├────────────────────────────────────┤
  │  ✦ 단일 소유권  │ 공유 소유권     │
  └────────────────────────────────────┘

  💻 Code Example  ·  🔑 Key Takeaways (4줄 정리)
```

---

## 설치 (CLI)

```bash
git clone https://github.com/jinyounghwa/cs_rust.git
cd cs_rust
cargo install --path .
```

## CLI 사용법

| 명령 | 설명 |
|------|------|
| `cs-bite` | 랜덤 토픽 출력 |
| `cs-bite list` | 전체 토픽 목록 보기 |
| `cs-bite <번호>` | 지정한 토픽 출력 |
| `cs-bite run` | 랜덤 토픽 + 코드 직접 실행 |

---

## 웹 버전

별도 설치 없이 브라우저에서 바로 실행할 수 있습니다.

```bash
# 파일 직접 열기
open web/index.html          # macOS
xdg-open web/index.html      # Linux
start web/index.html         # Windows

# 로컬 서버 실행 (추천 — CORS 문제 방지)
cd web && python3 -m http.server 8080
# → http://localhost:8080 접속
```

| 기능 | 설명 |
|------|------|
| 사이드바 토픽 탐색 | Lucide SVG 아이콘으로 카테고리별 목록 표시 |
| Shuffle 랜덤 토픽 | 버튼 또는 `R` 키로 랜덤 이동 |
| ◀ ▶ 이동 | 버튼 또는 `←` `→` 키로 이전/다음 토픽 |
| Copy 코드 복사 | 코드 블록 원클릭 복사 |
| Directory 목록 | 전체 38개 토픽 그리드로 한눈에 보기 |
| 반응형 | 모바일에서 사이드바 접힘 (☰ 토글) |

> `web/index.html` + `web/topics.js` 두 파일만으로 완전 동작합니다.

---

## 토픽 목록 (38개)

### 🌱 기초 — 언어의 첫걸음 (6개)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 1 | 변수, 상수, 불변성 | `let` vs `let mut` vs `const`, 섀도잉 |
| 2 | 기본 타입 시스템 | i32, f64, char, 튜플, 배열, `as` 캐스팅 |
| 3 | 함수와 반환값 | 식 vs 문, 세미콜론의 의미, if 표현식 |
| 4 | String vs &str | Heap/String과 바이너리/&str, 슬라이스 |
| 5 | 제어 흐름 | if식, loop값반환, for..in, range |
| 6 | Vec\<T\>와 HashMap\<K,V\> | 동적배열, 해시맵, `get()` 안전접근 |

### 💎 핵심 — Rust다운 코드 (6개)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 7 | Struct & impl | 구조체, 메서드, 연관함수, derive |
| 8 | Enum & Pattern Matching | ADT, match 완전성, if let |
| 9 | Ownership | 소유권 3규칙, Move/Copy/Clone |
| 10 | Borrowing & References | &T vs &mut T, 빌림 규칙, NLL |
| 11 | Option\<T\> | null 대신 타입으로, map/and_then |
| 12 | Result\<T,E\> & 에러 처리 | `?` 연산자, map_err, 에러 전파 |

### 🧩 추상화 — 다형성과 재사용 (4개)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 13 | Traits | 트레이트 정의, impl/dyn, 기본구현 |
| 14 | Generics | 단형화, 트레이트 바운드, where |
| 15 | Closures & Iterators | Fn/FnMut/FnOnce, lazy 체인 |
| 16 | Lifetime ('a) | 참조 유효기간, elision, 'static |

### ⚡ 동시성 — 병렬과 안전 (3개)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 17 | Smart Pointers | Box, Rc, Arc, RefCell |
| 18 | Threads & Mutex | spawn, Arc\<Mutex\<T\>\>, Send/Sync |
| 19 | Channels | mpsc, 소유권 이전, Producer-Consumer |

### 🏗 실전 — 실무 패턴 (3개)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 20 | Module System | mod, use, pub, 가시성, 파일 매핑 |
| 21 | Testing | #[test], assert!, cargo test |
| 22 | Error Handling 실전 | thiserror, anyhow, 커스텀 에러 설계 |

### 🌳 자료구조 (4개, 웹 전용)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 23 | 배열과 연결 리스트 | 연속 메모리 vs 포인터 체인 |
| 24 | 스택과 큐 | LIFO/FIFO, 재귀 vs 반복 |
| 25 | 트리와 힙 | BST, Min/Max Heap, 우선순위 큐 |
| 26 | 해시 테이블 | 해시 함수, 충돌 해결, Load Factor |

### 💻 운영체제 (8개, 웹 전용)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 27 | 프로세스와 스레드 | PCB, Context Switch, User/Kernel Thread |
| 28 | 메모리 관리 | Stack/Heap, 가상 메모리, 페이지 테이블 |
| 29 | CPU 스케줄링 | FIFO, SJF, Round Robin, Priority |
| 30 | 동기화와 교착상태 | Mutex, Semaphore, 데드락 조건 |
| 31 | 파일 시스템 | inode, 디렉토리 구조, FAT vs ext4 |
| 32 | 가상화와 컨테이너 | Hypervisor, Docker, 네임스페이스 |
| 33 | 시스템 콜과 인터럽트 | syscall, IRQ, 커널 진입 |
| 34 | 캐시와 메모리 계층 | L1/L2/L3, 지역성, 캐시 미스 |

### 🌐 네트워크 (2개, 웹 전용)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 35 | TCP/IP와 HTTP | 3-Way Handshake, HTTP 메서드, 상태코드 |
| 36 | REST API 설계 | REST 원칙, CRUD API 시뮬레이션 |

### 🗄 데이터베이스 (4개, 웹 전용)
| # | 토픽 | 핵심 내용 |
|---|------|----------|
| 37 | 관계형 DB와 SQL | 테이블/JOIN/인덱스, ACID, 정규화 |
| 38 | 트랜잭션과 동시성 | 계좌 이체 시뮬레이션, ACID, 격리 수준 |

---

## 비주얼 시스템

각 토픽은 6개의 섹션으로 구성됩니다:

| 섹션 | CLI 아이콘 | Web 아이콘 | 설명 |
|------|-----------|-----------|------|
| **Concept** | 💡 | `lightbulb` | 핵심 개념 간결 설명 |
| **Why It Matters** | 🎯 | `target` | 실무 연결 이유 |
| **Comparison Matrix** | ⚖ | `scale` | 개념 간 비교 테이블 |
| **Visual Diagram** | 📐 | `pen-tool` | ASCII 메모리/흐름 다이어그램 |
| **Code Example** | 💻 | `code-2` | 실행 가능한 Rust 코드 |
| **Key Takeaways** | 🔑 | `key` | 4줄 요약 |

---

## 프로젝트 구조

```
cs_rust/
├── src/
│   ├── main.rs              ← CLI 진입점 (인자 파싱, 실행 분기)
│   ├── topic.rs             ← CSTopic 구조체 정의
│   ├── display.rs           ← 터미널 출력 (print_topic, list_topics, 신택스 컬러링)
│   ├── visual.rs            ← 비주얼 렌더링 (다이어그램, 비교표, 프로그레스바)
│   ├── runner.rs            ← 코드 실행 (rustc 컴파일 & 실행)
│   └── topics/              ← 22개 CLI 토픽 (카테고리별 독립 파일)
│       ├── mod.rs
│       ├── variables.rs, types.rs, functions.rs, strings.rs
│       ├── control_flow.rs, collections.rs, structs.rs, enums.rs
│       ├── ownership.rs, borrowing.rs, option.rs, result.rs
│       ├── traits.rs, generics.rs, closures.rs, lifetimes.rs
│       ├── smartpointers.rs, threads.rs, channels.rs
│       ├── modules.rs, testing.rs, error_handling.rs
│       └── ... (자료구조/OS/네트워크/DB는 topics.js에 포함)
└── web/
    ├── index.html           ← 웹 UI (Lucide SVG 아이콘, 다크 테마)
    └── topics.js            ← 38개 전체 토픽 데이터 (JS)
```

> CLI 토픽 추가: 파일 하나 만들고 `topics/mod.rs`에 두 줄 추가하면 끝.

---

## 특징

- ⏱ **2~5분** 안에 읽고 이해할 수 있는 분량
- 🖥 **CLI + 웹** 두 가지 인터페이스 지원
- 🎨 **Lucide SVG 아이콘** — 웹 버전 프리미엄 디자인
- 📐 **ASCII 다이어그램** — 메모리 구조·흐름 시각화
- ⚖ **비교 테이블** — Rust vs TypeScript, 개념 간 차이
- 🦀 **실행 가능한 Rust 코드** — `rustc`로 컴파일 & 실행
- 🌈 **컬러 터미널** — 카테고리별 색상, 프로그레스 바
- 📊 **38개 토픽** — Rust 기초 22개 + CS 심화 16개

---

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
