use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "변수, 상수, 불변성",
        category: "기초",
        explanation: "\
Rust에서 모든 변수는 기본적으로 불변(immutable)입니다.
값을 바꾸려면 명시적으로 mut 키워드를 붙여야 합니다.
이것은 실수로 값을 덮어쓰는 버그를 컴파일 타임에 막아줍니다.

  let x = 5;            // 불변 변수
  let mut y = 5;        // 가변 변수
  const MAX: u32 = 100; // 상수 (타입 명시 필수, 절대 변경 불가)

변수 섀도잉(shadowing): 같은 이름으로 새 변수를 선언해서
타입도 바꿀 수 있습니다. 이것은 mut와 다른 개념입니다!",
        why_it_matters: "\
'왜 기본이 불변이지?' → 함수형 프로그래밍 철학. 상태 변화를 추적하기 어렵고
버그의 주원인이 되는 '의도치 않은 변경'을 컴파일러가 잡아줍니다.
NestJS에서 const를 쓰는 이유와 같지만, Rust는 기본값이 const입니다.

Rust에서 '안전함'의 출발점이 바로 이 기본 불변성입니다.",
        diagram: "\
  스택 메모리에서 일어나는 일
  ──────────────────────────────────

  let x = 5;         let mut y = 5;       y = 10;

  ┌──────┐          ┌──────┐           ┌──────┐
  │  x   │          │  y   │           │  y   │
  │  5   │          │  5   │    ──►    │  10  │
  └──────┘          └──────┘           └──────┘
   immutable          mutable           수정 OK!

  x = 6;  ✗ 에러!       y = 10;  ✓ OK

  섀도잉 vs mut:
  ──────────────────────────────────
  let spaces = \"   \";     // &str
  let spaces = spaces.len(); // usize ← 새 변수!
    ↓
  ┌──────────┐     ┌──────────┐
  │ spaces   │ ──► │ spaces   │
  │ \"   \"    │     │ 3        │
  │ (&str)   │     │ (usize)  │
  └──────────┘     └──────────┘
   이전 것은 drop    새 타입으로 교체",
        code: r#"fn main() {
    // 기본: 불변 변수
    let x = 5;
    // x = 6; // 컴파일 에러! cannot assign twice to immutable variable

    // mut으로 가변 선언
    let mut y = 5;
    println!("y = {}", y);
    y = 10;
    println!("y = {}", y);  // y = 10

    // 상수: 타입 명시 필수, 런타임 계산값 불가
    const MAX_USERS: u32 = 1_000_000;  // 언더스코어로 가독성 향상
    println!("Max: {}", MAX_USERS);

    // 섀도잉: 같은 이름으로 새 변수 선언 (타입도 변경 가능!)
    let spaces = "   ";         // &str
    let spaces = spaces.len(); // usize (타입이 바뀌었지만 에러 없음)
    println!("Spaces: {}", spaces);  // 3

    // 섀도잉으로 값 변환 후 재사용
    let x = 5;
    let x = x + 1;     // 6
    let x = x * 2;     // 12
    println!("Shadowed x = {}", x);  // 12
}
"#,
        key_points: &[
            "let: 불변 / let mut: 가변 — 기본이 불변이라 실수가 줄어든다",
            "const: 타입 필수, 절대 변경 불가, 전역 스코프 가능",
            "섀도잉: 같은 이름으로 다른 타입의 변수를 선언 가능 (mut과 다름)",
            "불변 설계 → 다중 참조가 안전 (나중에 Borrowing과 연결)",
        ],
        comparisons: &[
            "header|let (불변)|let mut (가변)",
            "diff|재할당 불가|재할당 가능",
            "diff|컴파일러가 보호|개발자가 의도 표현",
            "equal|스택에 저장|스택에 저장",
            "diff|기본값|명시적 선언 필요",
        ],
    }
}
