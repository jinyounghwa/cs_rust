use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Result<T, E> & 에러 처리",
        category: "핵심",
        explanation: "\
Option이 '없을 수 있음'이라면, Result는 '실패할 수 있음'을 타입으로 표현합니다.

  Result<T, E>:
    Ok(T):  성공, 값 포함
    Err(E): 실패, 에러 포함

try/catch가 없습니다! 에러는 반환값으로 전달됩니다.

핵심 연산자 ?: 에러가 있으면 즉시 현재 함수에서 Err를 반환합니다.
  let n = parse_number(s)?;  // 에러면 Err 반환하고 함수 종료

? 연산자 덕분에 에러 전파가 간결하고, 처리를 강제합니다.",
        why_it_matters: "\
NestJS에서 throw/catch로 에러를 던지면 어떤 함수가 어떤 에러를 낼 수 있는지 타입으로 알 수 없습니다.
Result<T, E>는 반환 타입에 에러 타입도 명시됩니다.
? 연산자로 에러 전파가 간결하고, 개발자가 에러 처리를 강제받습니다.",
        diagram: "\
  Result<T,E> 흐름과 ? 연산자
  ──────────────────────────────────

  Result<T, E> 메모리:
  ┌────────┬───────────────────────┐
  │ 태그   │ Ok(T) 또는 Err(E)     │
  │ 0(Ok) │ 값 데이터              │
  │ 1(Err)│ 에러 데이터            │
  └────────┴───────────────────────┘

  ? 연산자의 마법:
  ──────────────────────────────────

  // 이 코드가:
  let n = parse_positive(s)?;

  // 이렇게 동작:
  match parse_positive(s) {
      Ok(val) => val,          // 성공 → 값 사용
      Err(e)  => return Err(e), // 실패 → 즉시 함수에서 Err 반환
  }

  에러 전파 체인:
  ──────────────────────────────────

  main() → double_positive() → parse_positive()
              │                      │
              │   \"21\"               │
              │ ◄── Ok(42)    ◄── Ok(21)
              │
              │   \"abc\"              │
              │ ◄── Err(\"invalid\") ◄── Err(...)
              │
              │   \"-5\"               │
              │ ◄── Err(\"negative\")◄── Err(...)
              │
           match로 최종 처리!",
        code: r#"use std::num::ParseIntError;

// 에러 타입을 명시적으로 정의
fn parse_positive(s: &str) -> Result<u32, String> {
    let n: i32 = s.parse().map_err(|e: ParseIntError| e.to_string())?;
    if n < 0 {
        return Err(format!("{} is negative", n));
    }
    Ok(n as u32)
}

// ? 연산자로 에러 전파
fn double_positive(s: &str) -> Result<u32, String> {
    let n = parse_positive(s)?;  // 에러면 즉시 Err 반환
    Ok(n * 2)
}

fn main() {
    // match로 처리
    match double_positive("21") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // 에러 케이스들
    println!("{:?}", double_positive("abc"));   // Err("invalid digit...")
    println!("{:?}", double_positive("-5"));    // Err("-5 is negative")

    // unwrap_or_else: 에러시 기본값
    let n = double_positive("bad").unwrap_or_else(|e| {
        println!("에러 발생: {}", e);
        0
    });
    println!("n = {}", n);

    // map: Ok일 때 변환
    let result = double_positive("5")
        .map(|n| format!("답: {}", n));
    println!("{:?}", result);

    // 표준 라이브러리 사용 예
    let parsed: Result<i32, _> = "42".parse();
    println!("{:?}", parsed);  // Ok(42)

    let bad: Result<i32, _> = "abc".parse();
    println!("{:?}", bad);     // Err(ParseIntError)
}
"#,
        key_points: &[
            "? 연산자: 에러면 즉시 현재 함수에서 Err 반환 (가장 자주 쓰는 패턴)",
            "map_err(): Err의 타입을 변환 / map(): Ok의 값을 변환",
            "에러 타입 E는 명시적 — 어떤 에러가 나올지 타입으로 문서화됨",
            "unwrap()은 프로토타입 코드에서만, 실제로는 match/?/unwrap_or 사용",
        ],
        comparisons: &[
            "header|Rust Result|Java/TS try-catch",
            "diff|반환값으로 에러 전달|예외 던지기",
            "diff|타입으로 에러 명시|런타임에만 알 수 있음",
            "win|? 연산자로 간결 전파|try-catch 중첩",
            "win|에러 처리 강제|처리 누락 가능",
            "diff|map_err로 에러 변환|catch에서 변환",
        ],
    }
}
