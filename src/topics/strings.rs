use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "String vs &str (문자열 두 종류)",
        category: "기초",
        explanation: "\
Rust 초학자가 가장 헷갈리는 것 중 하나입니다. 문자열이 두 종류입니다.

  &str (문자열 슬라이스)
    - 불변, 고정 크기
    - 프로그램 바이너리 또는 String의 일부를 가리킴
    - 메모리: 스택에 (포인터, 길이)만 저장

  String (소유된 문자열)
    - 가변, 동적 크기
    - Heap에 할당
    - 런타임에 생성/수정 가능
    - 메모리: 스택에 (포인터, 길이, 용량) + Heap에 실제 데이터

함수 매개변수는 대부분 &str로 받습니다 (String도 &str로 자동 변환 가능).",
        why_it_matters: "\
NestJS에서 string 하나로 되던 것이 왜 두 종류인가?
→ 메모리 위치와 소유권 때문입니다.
&str: '이 문자열을 그냥 보겠다' (참조, 복사 없음, 빠름)
String: '내가 이 문자열을 소유하겠다' (Heap 할당, 수정 가능)
API 설계 시 &str로 받으면 유연합니다.",
        diagram: "\
  String vs &str 메모리 구조
  ──────────────────────────────────

  let s: &str = \"hello\";
  ┌──────────────┐
  │ s (스택)     │     \"hello\" (바이너리 영역)
  │ ptr ─────────┼────► h e l l o \\0
  │ len: 5       │
  └──────────────┘

  let s: String = String::from(\"hello\");
  ┌──────────────┐     ┌───────────────────┐
  │ s (스택)     │     │ Heap              │
  │ ptr ─────────┼────►│ h e l l o \\0      │
  │ len: 5       │     │                   │
  │ cap: 5       │     │                   │
  └──────────────┘     └───────────────────┘

  let slice: &str = &s[0..2];
  ┌──────────────┐     ┌───────────────────┐
  │ slice(스택)  │     │ Heap              │
  │ ptr ─────────┼────►│ he                │
  │ len: 2       │     │ (원본의 일부)      │
  └──────────────┘     └───────────────────┘

  format! vs + 연산자:
  let c = a + &b;        let f = format!(\"{}{}\",d,e);
  ┌───┐  ┌───┐          ┌───┐  ┌───┐
  │ a │  │ b │          │ d │  │ e │  둘 다 유효!
  └─┬─┘  └───┘          └───┘  └───┘
    │
  소유권 이동!               새 String 생성
  a 사용 불가                 d, e 그대로 유지",
        code: r#"fn main() {
    // &str: 문자열 리터럴 (바이너리에 저장, 불변)
    let s1: &str = "Hello, world!";
    println!("{}", s1);

    // String: Heap 할당, 가변
    let mut s2: String = String::from("Hello");
    s2.push_str(", world!"); // 문자열 추가
    s2.push('!');            // 문자 추가
    println!("{}", s2);

    // String -> &str 변환 (자동/명시)
    let s3: String = String::from("Rust");
    let s4: &str = &s3;       // String을 &str로 빌림
    let s5: &str = &s3[0..2]; // 슬라이스 (첫 2바이트)
    println!("{}, {}", s4, s5);

    // 함수 설계: &str로 받으면 둘 다 수용
    fn say(msg: &str) {
        println!("Say: {}", msg);
    }
    say("리터럴");        // &str 직접
    say(&s2);            // &String -> &str 자동 변환 (Deref)

    // 연결: + 연산자는 String을 소유권 이동 후 뒤에 &str 추가
    let a = String::from("Hello");
    let b = String::from(" World");
    let c = a + &b;  // a의 소유권이 이동됨, a는 더 이상 사용 불가
    println!("{}", c);

    // format!: 소유권 이동 없이 연결
    let d = String::from("Hello");
    let e = String::from(" World");
    let f = format!("{}{}", d, e);  // d, e 모두 유효
    println!("{}", f);
}
"#,
        key_points: &[
            "&str: 불변 참조, Heap 할당 없음 — 빠르고 가볍다",
            "String: Heap 할당, 가변, 소유권을 가짐",
            "함수 매개변수는 &str로 받으면 String/&str 모두 수용 가능",
            "format!(): 소유권 이동 없이 문자열 조합하는 안전한 방법",
        ],
        comparisons: &[
            "header|&str (문자열 슬라이스)|String (소유 문자열)",
            "diff|불변|가변 (push, push_str)",
            "diff|바이너리/참조 영역|Heap 할당",
            "diff|ptr + len (2개 필드)|ptr + len + cap (3개 필드)",
            "left|빠르고 가벼움|생성/해제 비용 있음",
            "diff|함수 파라미터에 추천|데이터 소유/수정 시 필요",
        ],
    }
}
