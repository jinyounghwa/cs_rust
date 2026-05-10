use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Option<T> — null을 타입으로",
        category: "핵심",
        explanation: "\
Rust에는 null/undefined가 없습니다. 대신 Option<T>으로 '값이 없을 수 있음'을 타입에 표현합니다.

  Option<T>는 두 가지 변형을 가진 Enum:
    Some(T): 값이 있음
    None:    값이 없음

null 체크를 빠뜨리면 런타임 에러가 나는 다른 언어들과 달리,
Option<T>를 처리하지 않으면 컴파일이 안 됩니다.

주요 메서드:
  .unwrap(): Some이면 값, None이면 panic
  .unwrap_or(default): None이면 기본값
  .map(f): Some이면 f 적용, None이면 None
  .and_then(f): Some이면 f(T) → Option (체이닝)
  .is_some() / .is_none()",
        why_it_matters: "\
Tony Hoare가 null을 발명하고 '10억 달러짜리 실수'라 불렀습니다.
Rust는 타입 시스템으로 이 문제를 해결했습니다.
API 반환값이 Option<T>이면 '없을 수 있다'는 의미가 타입에 명시됩니다.
개발자가 None 케이스를 반드시 처리해야 합니다.",
        diagram: "\
  Option<T>의 메모리 레이아웃
  ──────────────────────────────────

  Option<i32>:
  ┌────────┬────────────┐
  │ 태그   │ 값          │
  │ 0(None)│ (미사용)    │   ← None: 4바이트 + 태그
  │ 1(Some)│ 42          │   ← Some(42): 4바이트 + 태그
  └────────┴────────────┘

  특별 최적화: Option<&T>는 0비용!
  ┌─────────────────────┐
  │ 포인터 (64비트)      │
  │ 0x0    → None        │   ← 널 포인터 = None
  │ 0x1234 → Some(&T)   │   ← 유효 주소 = Some
  └─────────────────────┘
  추가 메모리 0바이트!

  Option 체이닝:
  ──────────────────────────────────

  find_user(1)          find_user(99)
       │                      │
    Some(\"Alice\")          None
       │                      │
    .and_then(make_email)     │
       │                      │
    Some(\"alice@...\")      None
       │                      │
    .map(|e| e.len())        │
       │                      │
    Some(13)               None

  최종: match로 분기 처리 → None 대비 필수!",
        code: r#"fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),
        2 => Some(String::from("Bob")),
        _ => None,
    }
}

fn get_email(user_id: u32) -> Option<String> {
    find_user(user_id).and_then(|name| {
        Some(format!("{}@example.com", name.to_lowercase()))
    })
}

fn main() {
    // 기본 처리: match
    match find_user(1) {
        Some(name) => println!("Found: {}", name),
        None => println!("Not found"),
    }

    // if let: 한 경우만 처리
    if let Some(name) = find_user(2) {
        println!("Found: {}", name);
    }

    // unwrap_or: 기본값
    let name = find_user(99).unwrap_or_else(|| String::from("Unknown"));
    println!("User: {}", name);

    // map: Some이면 변환
    let len = find_user(1).map(|n| n.len());
    println!("Name length: {:?}", len);  // Some(5)

    // and_then: 체이닝
    if let Some(email) = get_email(1) {
        println!("Email: {}", email);
    }

    // Option을 반환하는 표준 라이브러리
    let v = vec![1, 2, 3];
    println!("{:?}", v.get(5));      // None
    println!("{:?}", v.first());     // Some(1)

    let s = "hello";
    println!("{:?}", s.find('l'));   // Some(2)
    println!("{:?}", s.find('z'));   // None
}
"#,
        key_points: &[
            "null이 없음 → Option<T>로 없음을 타입에 표현",
            ".unwrap(): 위험, None이면 panic / .unwrap_or(): 기본값으로 안전 처리",
            ".map(): Some에만 함수 적용, None은 그대로 / .and_then(): Option 체이닝",
            "if let Some(x) = opt { ... }: 한 가지 경우만 처리할 때 간결",
        ],
        comparisons: &[
            "header|null 방식 (TS/Java)|Option<T> 방식 (Rust)",
            "diff|null/undefined 존재|null/undefined 없음",
            "diff|체크 안 해도 컴파일 됨|처리 안 하면 컴파일 에러",
            "diff|NullPointerException 가능|불가능 (타입이 보장)",
            "win| |컴파일러가 None 처리 강제",
        ],
    }
}
