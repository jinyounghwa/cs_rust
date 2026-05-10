use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Lifetime ('a) — 참조의 유효 기간",
        category: "추상화",
        explanation: "\
라이프타임은 참조가 '언제까지 유효한지'를 컴파일러에게 알려주는 주석입니다.
대부분의 경우 컴파일러가 자동으로 추론(lifetime elision)합니다.

명시적으로 써야 하는 경우:
  1. 함수가 참조를 받고 참조를 반환할 때
  2. 구조체가 참조를 필드로 가질 때

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
'x와 y 중 짧은 수명만큼 반환값이 살아있다'는 의미입니다.",
        why_it_matters: "\
댕글링 참조를 컴파일 타임에 막습니다.
다른 언어에서 null pointer exception이 나는 상황을 컴파일러가 잡습니다.
처음에는 가장 어렵게 느껴지지만, 명시해야 하는 경우는 많지 않습니다.",
        diagram: "\
  라이프타임 타임라인
  ──────────────────────────────────

  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

  시간 ─────────────────────────────►

  s1: |══════════════════════════════|
      ^ 생성                        ^ drop

  s2:      |════════════|
           ^ 생성      ^ drop

  'a:      |════════════|
           ^ s1,s2 교집합 (짧은 쪽)

  result:  |════════════|
           반환값은 'a 안에서만 유효

  ──────────────────────────────────

  라이프타임 오류 예:
  ──────────────────────────────────

  let result;
  {
      let s2 = String::from(\"short\");
      result = longest(s1.as_str(), s2.as_str());
  }  // s2 drop!
  println!(\"{}\", result);  // ✗ 에러!
                            // result의 라이프타임이
                            // 이미 끝난 s2에 의존

  Elision 규칙 (자동 추론):
  ──────────────────────────────────
  규칙 1: 각 참조 파라미터마다 고유 라이프타임
  규칙 2: 입력이 하나면 출력에 동일 라이프타임
  규칙 3: &self면 출력에 self의 라이프타임

  fn first_word(s: &str) -> &str
  → 자동 추론: fn first_word<'a>(s: &'a str) -> &'a str",
        code: r#"// 두 참조 중 긴 것을 반환 — 반환값 라이프타임 명시 필요
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 구조체가 참조를 가질 때
struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn announce(&self) -> &str {
        self.content
    }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);  // OK: 같은 스코프
    }
    // println!("{}", result);  // 에러! s2가 이미 drop됨

    // 'static: 프로그램 전체 수명
    let lit: &'static str = "I have a static lifetime";

    // 구조체 라이프타임
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let important = Important { content: first_sentence };
    println!("{}", important.announce());
}
"#,
        key_points: &[
            "라이프타임은 컴파일러를 위한 주석 — 런타임 비용 없음",
            "대부분 자동 추론, 여러 참조가 얽히면 명시 필요",
            "'a: 두 참조 중 겹치는 수명(짧은 쪽)으로 제한",
            "'static: 프로그램 전체 수명 (문자열 리터럴이 대표적)",
        ],
        comparisons: &[
            "header|명시 필요|자동 추론 (Elision)",
            "diff|여러 참조 입력+참조 반환|입력 참조 하나",
            "diff|구조체에 참조 필드|함수 파라미터만",
            "diff|impl<'a> Type<'a>|&self 메서드",
        ],
    }
}
