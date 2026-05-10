use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Ownership (소유권) — Rust의 심장",
        category: "핵심",
        explanation: "\
Rust에는 GC(가비지 컬렉터)가 없습니다. 대신 '소유권' 규칙으로 메모리를 관리합니다.

소유권 3대 규칙:
  1. 모든 값에는 소유자(owner)가 있다
  2. 한 번에 소유자는 하나뿐이다
  3. 소유자가 스코프를 벗어나면 값은 drop()된다

Move (이동): 대입하면 소유권이 이동, 이전 변수는 무효화
Copy: i32, bool, char 등 스택 타입은 복사됨 (소유권 이동 없음)
Clone: Heap 타입을 복사하려면 .clone() 명시 필요

이 규칙이 이중 해제, 댕글링 포인터, 메모리 누수를 컴파일 타임에 막습니다!",
        why_it_matters: "\
GC 없이 메모리 안전성을 보장하는 Rust의 핵심 메커니즘입니다.
C/C++는 수동 관리(버그 많음), Java/Go는 GC(런타임 오버헤드), Rust는 컴파일 타임 검사.
처음에 어색하지만 익숙해지면 '컴파일러가 메모리 버그를 잡아준다'는 안정감이 큽니다.",
        diagram: "\
  Move vs Copy — 핵심 시각화
  ──────────────────────────────────

  ⚡ MOVE (Heap 데이터)
  let s1 = String::from(\"hello\");
  let s2 = s1;  // 소유권 이동!

  BEFORE:                  AFTER:
  ┌──────┐  ┌───────┐     ┌──────┐  ┌───────┐
  │ s1   │  │ Heap  │     │ s1   │  │ Heap  │
  │ ptr──┼─►│hello  │     │ ??   │  │hello  │
  │ len:5│  └───────┘     │ ✗무효│  └───────┘
  │ cap:5│                └──────┘     ▲
  └──────┘                              │
                              ┌──────┐  │
                              │ s2   │  │
                              │ ptr──┼──┘
                              │ len:5│
                              │ cap:5│
                              └──────┘
  s1 사용 불가! s2만 유효!

  ✓ COPY (스택 데이터)
  let x = 5;
  let y = x;  // 복사!

  ┌──────┐     ┌──────┐
  │ x: 5 │     │ y: 5 │
  └──────┘     └──────┘
  둘 다 유효! (i32는 Copy 트레이트)

  함수로 소유권 이동:
  ──────────────────────────────────
  let s = String::from(\"Rust\");
  takes_ownership(s);   // s 이동 → s 무효
  // println!(\"{}\", s); // 에러!

  fn takes_ownership(s: String) {
      println!(\"{}\", s);
  }  // 여기서 s drop! 메모리 해제",
        code: r#"fn main() {
    // ─── Move (이동) ───
    let s1 = String::from("hello");  // s1이 소유
    let s2 = s1;                      // 소유권이 s2로 이동
    // println!("{}", s1);            // 에러! s1은 유효하지 않음
    println!("{}", s2);               // OK

    // ─── Copy (복사) ───
    let x = 5;    // i32는 Copy 트레이트 구현
    let y = x;    // y는 x의 복사본
    println!("{} {}", x, y);  // 둘 다 유효

    // ─── Clone (명시적 깊은 복사) ───
    let s3 = String::from("world");
    let s4 = s3.clone();  // Heap 데이터까지 복사
    println!("{} {}", s3, s4);  // 둘 다 유효

    // ─── 함수로 이동 ───
    let s5 = String::from("Rust");
    takes_ownership(s5);     // s5의 소유권이 함수로 이동
    // println!("{}", s5);   // 에러! 이미 이동됨

    let n = 10;
    makes_copy(n);           // i32는 복사되므로 n 여전히 유효
    println!("{}", n);       // OK

    // ─── 함수에서 소유권 반환 ───
    let s6 = gives_ownership();  // 함수가 소유권 반환
    println!("{}", s6);
}

fn takes_ownership(s: String) {
    println!("Owned: {}", s);
}  // 여기서 s가 drop됨

fn makes_copy(n: i32) {
    println!("Copied: {}", n);
}  // n은 복사본, 원본에 영향 없음

fn gives_ownership() -> String {
    String::from("given")
}  // 반환하면 소유권 이동, drop 안 됨
"#,
        key_points: &[
            "소유권은 하나: 이동 후 원래 변수는 무효 — 이중 해제 불가능",
            "스코프 끝에서 drop() 자동 호출 — GC 없이 메모리 해제",
            "i32/bool/char: Copy (자동 복사) / String/Vec: Move (소유권 이동)",
            ".clone(): 명시적 깊은 복사 — 비용이 있음을 코드에서 명확히 드러냄",
        ],
        comparisons: &[
            "header|Copy (자동)|Move (이동)|Clone (명시)",
            "diff|i32, bool, char, f64|String, Vec, Box|모든 타입 가능",
            "diff|스택 값 복사|소유권만 이동|Heap까지 깊은 복사",
            "diff|원본 유효|원본 무효|원본 유효",
            "diff|비용 없음|비용 없음|비용 있음 (Heap 할당)",
        ],
    }
}
