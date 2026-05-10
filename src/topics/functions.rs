use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "함수와 반환값",
        category: "기초",
        explanation: "\
Rust 함수는 fn 키워드로 정의하며, 매개변수와 반환 타입을 명시합니다.
중요한 특징: 마지막 표현식이 세미콜론 없이 끝나면 그것이 반환값이 됩니다.
return 키워드도 쓸 수 있지만, 관용적으로 마지막 표현식을 사용합니다.

  문(statement): 값을 반환하지 않는 코드 (let x = 5;)
  식(expression): 값을 평가하는 코드 (5 + 3, if...else, 블록{})

Rust에서 if/else, 블록 {} 자체가 '식'이어서 값을 반환할 수 있습니다.
이것이 세미콜론의 역할을 이해하는 핵심입니다!",
        why_it_matters: "\
세미콜론이 의미를 바꿉니다! 실수하기 쉬운 부분입니다.
- add(a, b) { a + b }  → a+b 반환 (Ok)
- add(a, b) { a + b; } → () (unit) 반환, 타입 불일치 에러!

이 구분은 처음에는 번거롭지만, 표현식 기반 코드가 간결하고 버그가 적습니다.",
        diagram: "\
  세미콜론의 마법 (가장 자주 하는 실수!)
  ──────────────────────────────────

  ✓ 올바른 코드:             ✗ 세미콜론 추가 시:
  fn add(a:i32,b:i32)->i32{  fn add(a:i32,b:i32)->i32{
      a + b                       a + b;
  }                          }
  반환: i32 (a+b)            반환: () ← 타입 불일치 에러!

  ┌──────────────────────────────────────────────┐
  │  식(expression)    →  값을 평가              │
  │    5 + 3           →  8                      │
  │    if x > 0 {..}   →  분기 결과값            │
  │    { let a=1; a+1} →  2 (마지막 식)          │
  │                                              │
  │  문(statement)     →  값을 반환하지 않음      │
  │    let x = 5;      →  () (unit)              │
  │    a + b;           →  () ← 세미콜론!        │
  └──────────────────────────────────────────────┘

  블록도 식이다:
  let result = {
      let x = 3;      ← 문 (값 없음)
      let y = 4;      ← 문 (값 없음)
      x * x + y * y   ← 식 (반환값, 세미콜론 없음!)
  };
  result = 25",
        code: r#"// 반환 타입을 -> 뒤에 명시
fn add(a: i32, b: i32) -> i32 {
    a + b  // 세미콜론 없음 = 이 값을 반환
}

// 반환값 없으면 -> 생략 (사실은 -> () 와 같음)
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 일찍 반환하려면 return 사용
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;  // 조기 반환
    }
    Some(a / b)  // 마지막 표현식 반환
}

fn main() {
    let sum = add(3, 4);
    println!("Sum: {}", sum);  // 7

    greet("Alice");

    // if 자체가 식(expression)이라 값을 반환할 수 있음
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"  // 세미콜론 없음
    } else {
        "C"
    };
    println!("Grade: {}", grade);

    // 블록도 식
    let result = {
        let x = 3;
        let y = 4;
        x * x + y * y  // 세미콜론 없이 반환
    };
    println!("Result: {}", result);  // 25

    // match도 식
    let n = 42;
    let parity = match n % 2 {
        0 => "짝수",
        _ => "홀수",
    };
    println!("{} is {}", n, parity);
}
"#,
        key_points: &[
            "마지막 표현식에 세미콜론 없음 = 반환값 (return 생략 가능)",
            "세미콜론 있으면 ()을 반환 — 반환 타입 불일치 시 컴파일 에러",
            "if/else, 블록 {}, match도 표현식이라 값을 가짐",
            "-> () 생략 가능, 반환값 없는 함수도 사실 ()을 반환",
        ],
        comparisons: &[
            "header|식 (Expression)|문 (Statement)",
            "left|값을 평가한다|값을 반환하지 않는다",
            "left|세미콜론 없음|세미콜론 있음",
            "left|반환값으로 사용 가능|반환값으로 사용 불가",
            "left|a + b → 8|let x = 5; → ()",
        ],
    }
}
