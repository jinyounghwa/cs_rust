use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "기본 타입 시스템",
        category: "기초",
        explanation: "\
Rust는 정적 타입 언어입니다. 대부분의 경우 컴파일러가 타입을 추론하지만,
명시적으로 적어주면 코드의 의도가 더 명확해집니다.

  정수: i8, i16, i32(기본), i64, i128, u8, u16, u32, u64
  부동소수점: f32, f64(기본)
  불리언: bool  (true/false)
  문자: char    (유니코드, 4바이트)
  튜플: (i32, f64, char)
  배열: [i32; 5]  (고정 크기, 스택에 저장)

타입 캐스팅은 as 키워드를 사용하며, 명시적으로만 가능합니다.
Rust에는 암묵적 타입 변환이 없습니다!",
        why_it_matters: "\
TypeScript와 달리 any가 없습니다. 타입이 맞지 않으면 컴파일되지 않습니다.
i32가 기본인 이유: 대부분의 정수 연산에 충분하고, 오버플로우 체크가 가능합니다.
배열 크기가 타입에 포함되는 이유: 스택 할당 크기를 컴파일 타임에 알아야 하기 때문입니다.",
        diagram: "\
  Rust 기본 타입의 메모리 크기
  ──────────────────────────────────

  타입       크기    범위
  ───────  ────  ────────────────
  u8       1바이트   0 ~ 255
  i32      4바이트   -2³¹ ~ 2³¹-1     ← 기본
  u64      8바이트   0 ~ 2⁶⁴-1
  f32      4바이트   단정도 부동소수점
  f64      8바이트   배정도 부동소수점  ← 기본
  bool     1바이트   true / false
  char     4바이트   유니코드 스칼라

  스택에 올라가는 모습:
  ──────────────────────────────────

  let x: i32 = 42;     let ch: char = '한';

  ┌───────┐            ┌───────┐
  │ x: 42 │ 4바이트    │ch: 한 │ 4바이트
  └───────┘            └───────┘

  let arr: [i32; 3] = [1, 2, 3];

  ┌───┬───┬───┐
  │ 1 │ 2 │ 3 │  12바이트 (연속)
  └───┴───┴───┘
   [0] [1] [2]",
        code: r#"fn main() {
    // 정수 (기본 i32)
    let x: i32 = 42;
    let big: i64 = 9_000_000_000;
    let byte: u8 = 255;  // 0~255

    // 부동소수점 (기본 f64)
    let pi: f64 = 3.14159;
    let small: f32 = 2.0_f32;

    // 불리언
    let is_active: bool = true;
    println!("Active: {}", is_active);

    // 문자 (유니코드! 한글도 됨)
    let ch: char = '한';
    println!("Char: {}", ch);

    // 튜플: 서로 다른 타입을 묶음
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (a, b, c) = tup;  // 구조분해
    println!("{}, {}, {}", a, b, c);
    println!("First: {}", tup.0);  // 인덱스로 접근

    // 배열: 같은 타입, 고정 크기
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Length: {}", arr.len());
    println!("Third: {}", arr[2]);

    // 타입 캐스팅 (명시적)
    let n: i32 = 1000;
    let m = n as i16;  // 명시적 변환 필요
    println!("Cast: {}", m);

    // 오버플로우 체크 (디버그 모드에서 패닉)
    // let overflow: u8 = 256; // 에러!
    let checked: u8 = 255;
    println!("u8 max: {}", checked);
}
"#,
        key_points: &[
            "정수 기본 i32, 부동소수점 기본 f64 — 명시하지 않으면 컴파일러가 추론",
            "배열 [T; N]: 크기가 타입에 포함됨 (컴파일 타임에 스택 크기 확정)",
            "튜플: 다른 타입 혼합 가능, 인덱스(.0, .1)나 구조분해로 접근",
            "as 캐스팅: 암묵적 변환 없음, 반드시 명시적으로",
        ],
        comparisons: &[
            "header|Rust|TypeScript",
            "diff|i32, u32, i64...|number 하나",
            "diff|char (4바이트 유니코드)|string (1글자도 string)",
            "diff|배열 [T; N] 크기 고정|any[] 가변 배열",
            "diff|any 타입 없음|any 가능",
            "win|오버플로우 컴파일 체크|런타임才 체크",
        ],
    }
}
