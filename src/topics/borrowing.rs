use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Borrowing & References (빌림과 참조)",
        category: "핵심",
        explanation: "\
소유권을 이전하지 않고 값을 '잠깐 빌려쓰는' 방법입니다.

  &T: 불변 참조 (immutable reference)
    - 동시에 여러 개 가능
    - 값 읽기만 가능

  &mut T: 가변 참조 (mutable reference)
    - 같은 스코프에서 하나만 가능
    - 값 읽기/쓰기 가능

빌림 규칙 (Borrowing Rules):
  1. 불변 참조 여러 개 OR 가변 참조 하나 — 동시에 공존 불가
  2. 참조는 항상 유효해야 함 (댕글링 참조 불가)

이 규칙이 컴파일 타임에 데이터 경쟁(race condition)을 막습니다!",
        why_it_matters: "\
'소유권을 넘기지 않고 쓰려면?' → 참조(&)를 사용합니다.
함수에 값을 넘길 때 소유권을 이동하면 돌려받을 때까지 못 씁니다.
참조로 넘기면 소유권을 유지한 채로 함수가 사용할 수 있습니다.

가변 참조가 하나만 가능한 이유: 데이터 경쟁을 원천 차단하기 때문입니다.",
        diagram: "\
  Borrowing 규칙 시각화
  ──────────────────────────────────

  ✓ 불변 참조 여러 개 (OK):
  let r1 = &s;
  let r2 = &s;
  let r3 = &s;

       ┌─────┐  ┌─────┐  ┌─────┐
  r1──►│     │  │     │  │     │◄──r3
       │  s  │  │     │  │     │
  r2──►│     │  │     │  │     │
       └─────┘  └─────┘  └─────┘
       모두 읽기만 → 안전!

  ✓ 가변 참조 하나만 (OK):
  let r = &mut s;

       ┌─────┐
  r───►│  s  │  수정 가능
       └─────┘

  ✗ 불변 + 가변 동시에 (에러!):
  let r1 = &s;
  let r2 = &mut s;  // 에러!

       ┌─────┐
  r1──►│  s  │ 읽기 중인데
  r2──►│     │ 수정하려 함 → 에러!

  NLL (Non-Lexical Lifetimes):
  ──────────────────────────────────
  r1, r2의 마지막 사용 이후에는
  가변 참조가 허용됨 (스마트!)

  let r1 = &s;
  println!(\"{}\", r1);  // r1 마지막 사용
  // ← 여기서 r1 스코프 종료 (NLL)
  let r2 = &mut s;  // OK!
  r2.push_str(\"!!!\");",
        code: r#"fn main() {
    let s1 = String::from("hello");

    // 불변 참조로 빌림 — 소유권 이전 없음
    let len = calculate_length(&s1);  // &s1: s1의 참조
    println!("{} has {} chars", s1, len);  // s1 여전히 유효

    // 가변 참조로 빌림
    let mut s2 = String::from("hello");
    change(&mut s2);  // &mut: 가변 참조
    println!("{}", s2);  // "hello, world!"

    // 빌림 규칙 시연 (NLL)
    let mut s3 = String::from("test");

    // 불변 참조는 여러 개 동시에 가능
    let r1 = &s3;
    let r2 = &s3;
    println!("{}, {}", r1, r2);  // r1, r2 마지막 사용

    // 이제 가변 참조 가능 (r1, r2 더 이상 안 쓰임)
    let r3 = &mut s3;
    r3.push_str("!!!");
    println!("{}", r3);

    // 슬라이스: 컬렉션의 일부에 대한 참조
    let s4 = String::from("hello world");
    let word = first_word(&s4);
    println!("First word: {}", word);  // "hello"
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // 참조 반납, s1은 drop 안 됨

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
"#,
        key_points: &[
            "& : 불변 참조 (여러 개 동시 가능) / &mut: 가변 참조 (하나만 가능)",
            "참조 기간 동안 원래 소유자가 값을 못 움직임",
            "가변 참조 하나만 제한 → 컴파일 타임에 데이터 경쟁 원천 차단",
            "슬라이스(&[T], &str): 컬렉션 일부에 대한 참조",
        ],
        comparisons: &[
            "header|&T (불변 참조)|&mut T (가변 참조)",
            "diff|여러 개 동시 가능|한 번에 하나만",
            "diff|읽기만 가능|읽기/쓰기 가능",
            "equal|소유권 이전 없음|소유권 이전 없음",
            "diff|원본 수정 불가|원본 수정 가능",
        ],
    }
}
