use colored::*;
use rand::Rng;
use std::env;
use std::process::Command;

struct CSTopic {
    title: &'static str,
    category: &'static str,
    explanation: &'static str,
    why_it_matters: &'static str,
    code: &'static str,
    key_points: &'static [&'static str],
}

const TOPICS: &[CSTopic] = &[
    // ────────── 기초: 언어의 첫걸음 ──────────
    CSTopic {
        title: "변수, 상수, 불변성",
        category: "기초",
        explanation: "\
Rust에서 모든 변수는 기본적으로 불변(immutable)입니다.
값을 바꾸려면 명시적으로 mut 키워드를 붙여야 합니다.
이것은 실수로 값을 덮어쓰는 버그를 컴파일 타임에 막아줍니다.

  let x = 5;          // 불변 변수
  let mut y = 5;      // 가변 변수
  const MAX: u32 = 100; // 상수 (타입 명시 필수, 절대 변경 불가)

변수 섀도잉(shadowing): 같은 이름으로 새 변수를 선언해서 타입도 바꿀 수 있습니다.",
        why_it_matters: "\
'왜 기본이 불변이지?' → 함수형 프로그래밍 철학. 상태 변화를 추적하기 어렵고
버그의 주원인이 되는 '의도치 않은 변경'을 컴파일러가 잡아줍니다.
NestJS에서 const를 쓰는 이유와 같지만, Rust는 기본값이 const입니다.",
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
}
"#,
        key_points: &[
            "let: 불변 / let mut: 가변 — 기본이 불변이라 실수가 줄어든다",
            "const: 타입 필수, 절대 변경 불가, 전역 스코프 가능",
            "섀도잉: 같은 이름으로 다른 타입의 변수를 선언 가능 (mut과 다름)",
            "불변 설계 → 다중 참조가 안전 (나중에 Borrowing과 연결)",
        ],
    },
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

타입 캐스팅은 as 키워드를 사용하며, 명시적으로만 가능합니다.",
        why_it_matters: "\
TypeScript와 달리 any가 없습니다. 타입이 맞지 않으면 컴파일되지 않습니다.
i32가 기본인 이유: 대부분의 정수 연산에 충분하고, 오버플로우 체크가 가능합니다.
배열 크기가 타입에 포함되는 이유: 스택 할당 크기를 컴파일 타임에 알아야 하기 때문입니다.",
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
}
"#,
        key_points: &[
            "정수 기본 i32, 부동소수점 기본 f64 — 명시하지 않으면 컴파일러가 추론",
            "배열 [T; N]: 크기가 타입에 포함됨 (컴파일 타임에 스택 크기 확정)",
            "튜플: 다른 타입 혼합 가능, 인덱스(.0, .1)나 구조분해로 접근",
            "as 캐스팅: 암묵적 변환 없음, 반드시 명시적으로",
        ],
    },
    CSTopic {
        title: "함수와 반환값",
        category: "기초",
        explanation: "\
Rust 함수는 fn 키워드로 정의하며, 매개변수와 반환 타입을 명시합니다.
중요한 특징: 마지막 표현식이 세미콜론 없이 끝나면 그것이 반환값이 됩니다.
return 키워드도 쓸 수 있지만, 관용적으로 마지막 표현식을 사용합니다.

  문(statement): 값을 반환하지 않는 코드 (let x = 5;)
  식(expression): 값을 평가하는 코드 (5 + 3, { let x=1; x+2 }, if ... else ...)

Rust에서 if/else, 블록 {} 자체가 '식'이어서 값을 반환할 수 있습니다.",
        why_it_matters: "\
세미콜론이 의미를 바꿉니다! 실수하기 쉬운 부분입니다.
- add(a, b) { a + b }  → a+b 반환
- add(a, b) { a + b; } → () (unit) 반환, 타입 불일치 에러
NestJS의 return 방식과 다르지만, 간결한 함수 표현을 가능하게 합니다.",
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
}
"#,
        key_points: &[
            "마지막 표현식에 세미콜론 없음 = 반환값 (return 생략 가능)",
            "세미콜론 있으면 ()을 반환 — 반환 타입 불일치 시 컴파일 에러",
            "if/else, 블록 {}도 표현식이라 값을 가짐",
            "-> () 생략 가능, 반환값 없는 함수도 사실 ()을 반환",
        ],
    },
    CSTopic {
        title: "String vs &str (문자열 두 종류)",
        category: "기초",
        explanation: "\
Rust 초학자가 가장 헷갈리는 것 중 하나입니다. 문자열이 두 종류입니다.

  &str (문자열 슬라이스)
    - 불변, 고정 크기
    - 프로그램에 하드코딩된 문자열 리터럴의 기본 타입
    - 메모리: 프로그램 바이너리 또는 다른 String의 일부를 가리킴

  String (소유된 문자열)
    - 가변, 동적 크기
    - Heap에 할당
    - 런타임에 생성/수정 가능

함수 매개변수는 대부분 &str로 받습니다 (String도 &str로 변환 가능하기 때문).",
        why_it_matters: "\
NestJS에서 string 하나로 되던 것이 왜 두 종류인가?
→ 메모리 위치와 소유권 때문입니다.
&str: '이 문자열을 그냥 보겠다' (참조, 복사 없음)
String: '내가 이 문자열을 소유하겠다' (Heap 할당, 수정 가능)
API 함수 설계 시 매개변수를 &str로 받으면 String과 &str 모두 받을 수 있어 유연합니다.",
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
    },
    CSTopic {
        title: "제어 흐름 (if, loop, while, for)",
        category: "기초",
        explanation: "\
Rust의 제어 흐름은 C/TypeScript와 비슷하지만 몇 가지 중요한 차이가 있습니다.

  1. if/else는 '식(expression)' — 값을 반환할 수 있음
  2. 조건문에 괄호 불필요 (if x > 5 { ... })
  3. loop: 무한 루프, break로 값을 반환 가능
  4. while: 조건 기반 반복
  5. for ... in: 반복자 기반 (가장 관용적)
  6. range: 0..5 (0~4), 0..=5 (0~5)

Rust에서는 for 루프에서 인덱스보다 반복자를 권장합니다.",
        why_it_matters: "\
loop에서 break value로 반환값을 뽑을 수 있습니다 — 재시도 로직에서 유용합니다.
for in을 선호하는 이유: 배열 범위 초과(off-by-one) 버그가 없고,
반복자 최적화로 C의 for 루프와 같은 성능을 냅니다.",
        code: r#"fn main() {
    // if/else: 식이므로 값을 반환
    let n = 7;
    let label = if n % 2 == 0 { "짝수" } else { "홀수" };
    println!("{} is {}", n, label);

    // loop: 무한 루프, break로 값 반환 가능
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2;  // 10 반환
        }
    };
    println!("loop result: {}", result);  // 10

    // while: 조건 기반
    let mut x = 0;
    while x < 3 {
        print!("{} ", x);
        x += 1;
    }
    println!();

    // for in range: 0, 1, 2, 3, 4
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // for in range inclusive: 0, 1, 2, 3, 4, 5
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    // for in 배열 반복 (인덱스 없이)
    let fruits = ["사과", "바나나", "오렌지"];
    for fruit in &fruits {
        println!("- {}", fruit);
    }

    // 인덱스가 필요하면 enumerate()
    for (i, fruit) in fruits.iter().enumerate() {
        println!("[{}] {}", i, fruit);
    }

    // 라벨로 중첩 루프 탈출
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                break 'outer;
            }
            print!("({},{}) ", x, y);
        }
    }
    println!();
}
"#,
        key_points: &[
            "if/else는 표현식 — 삼항 연산자 대신 사용",
            "loop { break value } — 반환값 있는 무한 루프",
            "for in: 인덱스 없이 안전하게 반복, enumerate()로 인덱스 추가",
            "0..5 (0~4), 0..=5 (0~5) — off-by-one 버그 방지",
        ],
    },
    CSTopic {
        title: "Vec<T>와 HashMap<K,V> (컬렉션)",
        category: "기초",
        explanation: "\
두 가지 가장 자주 쓰는 컬렉션입니다.

  Vec<T>: 동적 배열 (TypeScript의 배열, Java의 ArrayList)
    - Heap에 저장, 크기 동적 변경
    - 연속된 메모리 블록

  HashMap<K, V>: 해시맵 (TypeScript의 Map, Node의 객체)
    - 키-값 저장, O(1) 평균 조회
    - Heap에 저장, 순서 보장 없음

둘 다 소유권 규칙을 따릅니다. 값을 넣으면 소유권이 이동됩니다.",
        why_it_matters: "\
Vec은 단순 배열보다 훨씬 많이 씁니다. 동적 크기 + 반복자와 조합하면 강력합니다.
HashMap은 NestJS의 Map<string, T>와 같은 역할.
주의: HashMap은 std::collections에서 import 필요.
get()은 Option<&V>를 반환합니다 — 키가 없으면 None.",
        code: r#"use std::collections::HashMap;

fn main() {
    // Vec 생성
    let mut v: Vec<i32> = Vec::new();  // 빈 Vec
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![10, 20, 30];  // 매크로로 초기화

    // Vec 접근
    println!("v[0] = {}", v[0]);             // 인덱스 (패닉 가능)
    println!("{:?}", v.get(1));              // Option<&i32>, 안전
    println!("len: {}", v.len());

    // Vec 반복
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // Vec 변환
    v.push(99);
    v.sort();
    v.dedup();          // 중복 제거 (정렬 후)
    println!("{:?}", v);

    // HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 75);

    // 조회: Option<&V> 반환
    let alice = scores.get("Alice");
    match alice {
        Some(score) => println!("Alice: {}", score),
        None => println!("Not found"),
    }

    // 없으면 삽입 (or_insert)
    scores.entry(String::from("Charlie")).or_insert(80);

    // 반복
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 값이 있는지 확인
    println!("Bob 있나? {}", scores.contains_key("Bob"));
}
"#,
        key_points: &[
            "vec![...]: 매크로로 간단 초기화 / Vec::new()로 빈 Vec 생성",
            "v[0]: 패닉 가능 / v.get(0): Option<&T> 반환 (안전)",
            "HashMap::get(): Option<&V> — 없는 키 접근을 안전하게 처리",
            "entry().or_insert(): 없으면 삽입, 있으면 기존 값 유지 (upsert)",
        ],
    },

    // ────────── 핵심: Rust다운 코드 ──────────
    CSTopic {
        title: "Struct & impl (데이터와 메서드)",
        category: "핵심",
        explanation: "\
Struct는 연관된 데이터를 묶는 사용자 정의 타입입니다.
impl 블록에서 해당 타입의 메서드를 정의합니다.

  연관 함수 (associated function): self 없음 → 타입으로 호출 (User::new())
  메서드 (method): &self 또는 &mut self → 인스턴스로 호출 (user.name())

NestJS 클래스와 비슷하지만, 상속이 없습니다.
대신 트레이트(Trait)로 공통 동작을 정의합니다.",
        why_it_matters: "\
클래스가 없는데 어떻게 OOP를 하냐고? Rust의 답: Struct + Trait 조합입니다.
상속 없음 → 합성(Composition)을 권장 → 더 유연하고 버그가 적습니다.
#[derive(Debug, Clone)]으로 자주 쓰는 트레이트를 자동 구현할 수 있습니다.",
        code: r#"#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    // 연관 함수 (생성자 역할, self 없음)
    fn new(name: &str, age: u32) -> Self {
        User {
            name: String::from(name),
            age,            // 변수명과 필드명이 같으면 축약 가능
            active: true,
        }
    }

    // 메서드 (읽기 전용)
    fn greeting(&self) -> String {
        format!("안녕하세요, {}세 {}입니다.", self.age, self.name)
    }

    // 메서드 (값 변경)
    fn deactivate(&mut self) {
        self.active = false;
    }

    // 소유권 소비 (self를 받아서 다른 타입으로 변환)
    fn into_name(self) -> String {
        self.name  // User가 사라지고 String만 남음
    }
}

fn main() {
    let mut user = User::new("Alice", 30);
    println!("{}", user.greeting());

    user.deactivate();
    println!("{:?}", user);  // #[derive(Debug)] 덕분에 출력 가능

    // 구조체 업데이트 문법
    let user2 = User {
        name: String::from("Bob"),
        ..user  // 나머지 필드는 user에서 복사 (age, active)
    };
    println!("{:?}", user2);

    // Clone으로 깊은 복사
    let user3 = user2.clone();
    println!("{:?}", user3);
}
"#,
        key_points: &[
            "&self: 불변 참조 메서드 / &mut self: 가변 메서드 / self: 소유권 소비",
            "연관 함수 Type::new(): 관용적 생성자 패턴",
            "#[derive(Debug, Clone, PartialEq)]: 자주 쓰는 트레이트 자동 구현",
            "구조체 업데이트 문법(..user): 일부 필드만 변경하고 나머지 복사",
        ],
    },
    CSTopic {
        title: "Enum & Pattern Matching (열거형과 패턴 매칭)",
        category: "핵심",
        explanation: "\
Rust의 Enum은 단순한 값 목록이 아닙니다. 각 변형(variant)이 데이터를 가질 수 있는
'대수적 데이터 타입(Algebraic Data Type)'입니다.

  enum Shape {
    Circle(f64),             // 반지름
    Rectangle(f64, f64),    // 너비, 높이
    Triangle { base: f64, height: f64 }, // 명명된 필드
  }

match는 모든 경우를 반드시 처리해야 합니다 (exhaustiveness check).
처리 안 하면 컴파일 에러! → 버그를 설계 단계에서 잡습니다.",
        why_it_matters: "\
TypeScript의 union type + 구조분해를 합쳐놓은 것과 비슷합니다.
Option<T>와 Result<T, E> 자체가 Enum입니다 (Rust 표준 라이브러리).
패턴 매칭은 단순 switch-case가 아니라 값, 구조, 타입을 동시에 분해합니다.",
        code: r#"#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "원",
            Shape::Rectangle(_, _) => "직사각형",
            Shape::Triangle { .. } => "삼각형",  // ..로 나머지 무시
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    for shape in &shapes {
        println!("{}: 넓이 = {:.2}", shape.name(), shape.area());
    }

    // if let: 한 가지 경우만 처리
    let s = Shape::Circle(3.0);
    if let Shape::Circle(r) = s {
        println!("원의 반지름: {}", r);
    }

    // 패턴에 guard 조건 추가
    let x = 7;
    match x {
        n if n < 0 => println!("음수"),
        0 => println!("영"),
        1..=9 => println!("한 자리"),  // 범위 패턴
        _ => println!("두 자리 이상"),  // 와일드카드
    }
}
"#,
        key_points: &[
            "Enum variant는 데이터를 가질 수 있음 (Tuple형, Struct형)",
            "match는 모든 경우 처리 강제 (컴파일 타임 exhaustiveness check)",
            "if let: 한 패턴만 처리할 때 match보다 간결",
            "1..=9: 범위 패턴 / _ : 와일드카드 / guard: 조건 추가",
        ],
    },
    CSTopic {
        title: "Ownership (소유권) — Rust의 심장",
        category: "핵심",
        explanation: "\
Rust에는 GC(가비지 컬렉터)가 없습니다. 대신 '소유권' 규칙으로 메모리를 관리합니다.

소유권 3대 규칙:
  1. 모든 값에는 소유자(owner)가 있다
  2. 한 번에 소유자는 하나뿐이다
  3. 소유자가 스코프를 벗어나면 값은 drop()된다 (메모리 해제)

Move (이동): 대입하면 소유권이 이동, 이전 변수는 무효화
Copy: i32, bool, char 등 스택 타입은 복사됨 (소유권 이동 없음)
Clone: Heap 타입을 복사하려면 .clone() 명시 필요",
        why_it_matters: "\
왜 이렇게 복잡하게 만들었나?
→ 댕글링 포인터, 이중 해제, 메모리 누수를 컴파일 타임에 원천 차단.
GC 없이도 안전하고, 런타임 오버헤드도 없습니다.
처음에 어색하지만 익숙해지면 '이 코드는 안전하다'를 컴파일러가 보장해줍니다.",
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
    },
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

이 규칙이 컴파일 타임에 데이터 경쟁(race condition)을 막습니다.",
        why_it_matters: "\
'소유권을 넘기지 않고 쓰려면?' → 참조(&)를 사용합니다.
함수에 값을 넘길 때 소유권을 이동하면 돌려받을 때까지 못 씁니다.
참조로 넘기면 소유권을 유지한 채로 함수가 사용할 수 있습니다.

가변 참조가 하나만 가능한 이유: 여러 곳에서 동시에 같은 메모리를 수정하면 데이터 경쟁이 발생하기 때문입니다.",
        code: r#"fn main() {
    let s1 = String::from("hello");

    // 불변 참조로 빌림 — 소유권 이전 없음
    let len = calculate_length(&s1);  // &s1: s1의 참조
    println!("{} has {} chars", s1, len);  // s1 여전히 유효

    // 가변 참조로 빌림
    let mut s2 = String::from("hello");
    change(&mut s2);  // &mut: 가변 참조
    println!("{}", s2);  // "hello, world!"

    // 빌림 규칙 시연
    let mut s3 = String::from("test");

    // 불변 참조는 여러 개 동시에 가능
    let r1 = &s3;
    let r2 = &s3;
    println!("{}, {}", r1, r2);  // r1, r2 마지막 사용 이후로 스코프 끝

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
    },
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
  .is_some() / .is_none()
  .unwrap(): Some이면 값, None이면 panic
  .unwrap_or(default): None이면 기본값
  .map(f): Some이면 f 적용, None이면 None
  .and_then(f): Some이면 f(T)로 또 다른 Option 반환 (체이닝)",
        why_it_matters: "\
Tony Hoare가 null을 발명하고 10억 달러짜리 실수라 불렀습니다.
Rust는 타입 시스템으로 이 문제를 해결했습니다.
API 반환값이 Option<T>이면 '없을 수 있다'는 의미가 타입에 명시됩니다.
개발자가 None 케이스를 반드시 처리해야 합니다.",
        code: r#"fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),
        2 => Some(String::from("Bob")),
        _ => None,
    }
}

fn get_email(user_id: u32) -> Option<String> {
    // and_then으로 Option 체이닝
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
    println!("{:?}", v.get(5));      // None (인덱스 초과)
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
    },
    CSTopic {
        title: "Result<T, E> & 에러 처리",
        category: "핵심",
        explanation: "\
Option이 '없을 수 있음'이라면, Result는 '실패할 수 있음'을 타입으로 표현합니다.

  Result<T, E>:
    Ok(T):  성공, 값 포함
    Err(E): 실패, 에러 포함

try/catch가 없습니다. 에러는 반환값으로 전달됩니다.

핵심 연산자 ?: 에러가 있으면 즉시 현재 함수에서 Err를 반환합니다.
  let n = parse_number(s)?;  // 에러면 Err 반환하고 함수 종료

커스텀 에러 타입 패턴:
  - thiserror crate로 에러 정의
  - anyhow crate로 간단한 에러 처리",
        why_it_matters: "\
NestJS에서 throw/catch로 에러를 던지면 어떤 함수가 어떤 에러를 낼 수 있는지 타입으로 알 수 없습니다.
Result<T, E>는 반환 타입에 에러 타입도 명시됩니다.
? 연산자 덕분에 에러 전파가 간결하고, 처리를 강제합니다.",
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
    },

    // ────────── 추상화: 다형성과 재사용 ──────────
    CSTopic {
        title: "Traits (트레이트) — 인터페이스이자 계약",
        category: "추상화",
        explanation: "\
Trait는 '이 타입이 할 수 있는 것'을 정의하는 계약입니다.
NestJS의 Interface와 비슷하지만 더 강력합니다:
  - 기본 구현(default implementation) 제공 가능
  - 여러 타입에 동일한 동작 부여 가능
  - 표준 라이브러리 트레이트를 구현해서 언어 기능 활용 가능
    (Display, Debug, Iterator, From, PartialOrd, ...)

트레이트 바운드: 함수가 받을 타입에 '이 트레이트를 구현해야 한다' 제약을 줍니다.
  fn print_area<T: Drawable>(shape: T) { ... }
  fn print_area(shape: &impl Drawable) { ... }  // 간략 문법",
        why_it_matters: "\
상속 없이 다형성을 달성하는 Rust의 핵심 메커니즘입니다.
Trait를 구현한다 = 그 계약을 이행한다.
dyn Trait: 런타임 다형성 (동적 디스패치)
impl Trait: 컴파일 타임 다형성 (정적 디스패치, 더 빠름)
표준 라이브러리 트레이트를 구현하면 언어 기능(+연산자, 출력, 비교 등)을 쓸 수 있습니다.",
        code: r#"use std::fmt;

// 트레이트 정의: 기본 구현 포함
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    // 기본 구현 (override 가능)
    fn describe(&self) -> String {
        format!("{}: {} 소리를 낸다", self.name(), self.sound())
    }
}

struct Dog { name: String }
struct Cat { name: String }

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "멍" }
    // describe는 기본 구현 사용
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "야옹" }
    // 기본 구현 override
    fn describe(&self) -> String {
        format!("고양이 {} (도도함)", self.name())
    }
}

// 표준 라이브러리 Display 트레이트 구현
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

// 트레이트 바운드: impl Trait 문법
fn make_sound(animal: &impl Animal) {
    println!("{}", animal.describe());
}

// 여러 바운드: + 문법
fn show<T: Animal + fmt::Debug>(animal: &T) {
    println!("{:?}", animal);
}

// 동적 디스패치: 런타임에 어떤 타입인지 결정
fn make_all_sounds(animals: &[&dyn Animal]) {
    for animal in animals {
        println!("{}", animal.describe());
    }
}

#[derive(Debug)]
struct Parrot { name: String }
impl Animal for Parrot {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "폴리원터크래커" }
}

fn main() {
    let dog = Dog { name: String::from("멍멍이") };
    let cat = Cat { name: String::from("야옹이") };
    let parrot = Parrot { name: String::from("앵무") };

    make_sound(&dog);
    make_sound(&cat);
    println!("{}", dog);  // Display 트레이트 덕분에 {}로 출력 가능

    let animals: Vec<&dyn Animal> = vec![&dog, &cat, &parrot];
    make_all_sounds(&animals);
}
"#,
        key_points: &[
            "trait: 메서드 시그니처 정의 + 기본 구현 제공 가능",
            "impl Trait: 정적 디스패치 (컴파일 타임 해결, 빠름)",
            "&dyn Trait: 동적 디스패치 (런타임 해결, 유연함)",
            "표준 트레이트(Display, Iterator, From 등) 구현으로 언어 기능 활용",
        ],
    },
    CSTopic {
        title: "Generics (제네릭) — 타입 매개변수",
        category: "추상화",
        explanation: "\
같은 로직을 다양한 타입에 재사용할 때 제네릭을 씁니다.
컴파일 타임에 구체적인 타입으로 '단형화(monomorphization)'되어
런타임 오버헤드가 없습니다. (Java의 제네릭과 다른 점)

  fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

T는 타입 매개변수, PartialOrd는 트레이트 바운드입니다.
'T는 반드시 PartialOrd를 구현해야 한다'는 의미입니다.

where 구문으로 복잡한 바운드를 정리할 수 있습니다.",
        why_it_matters: "\
TypeScript의 제네릭과 개념은 같지만, 런타임 동작이 다릅니다.
TypeScript: 타입 정보가 런타임에 사라짐 (type erasure)
Rust: 컴파일 타임에 구체 타입으로 확정 (단형화) → 런타임 비용 없음

트레이트 바운드 덕분에 '이 타입으로 뭘 할 수 있는지' 제약을 명시합니다.",
        code: r#"// 트레이트 바운드가 있는 제네릭 함수
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 제네릭 구조체
#[derive(Debug)]
struct Wrapper<T> {
    value: T,
}

impl<T: std::fmt::Display> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn show(&self) {
        println!("Wrapped: {}", self.value);
    }
}

// where 구문으로 복잡한 바운드 정리
fn complex_fn<T, U>(t: T, u: U) -> String
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug,
{
    format!("{:?} + {}", u, t.clone())
}

// 제네릭 Enum (Option과 Result가 이렇게 구현됨)
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));  // 100

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));  // y

    let w1 = Wrapper::new(42);
    let w2 = Wrapper::new("hello");
    w1.show();
    w2.show();

    println!("{}", complex_fn("Rust", vec![1, 2, 3]));
}
"#,
        key_points: &[
            "단형화: 컴파일 타임에 구체 타입으로 확정 → 런타임 오버헤드 없음",
            "트레이트 바운드 T: Trait → '이 타입은 이 능력이 있어야 한다'",
            "where 구문: 복잡한 바운드를 함수 시그니처 밖으로 분리해서 가독성 향상",
            "Vec<T>, Option<T>, Result<T,E> 모두 제네릭으로 구현됨",
        ],
    },
    CSTopic {
        title: "Closures & Iterators — 함수형 패턴",
        category: "추상화",
        explanation: "\
클로저: 주변 환경(변수)을 캡처하는 익명 함수
  |x| x + 1              // 인자 하나, 타입 추론
  |x: i32| -> i32 { x } // 타입 명시
  move |x| x + n         // n의 소유권을 캡처

Fn 트레이트 3종:
  Fn:     불변 참조로 캡처 (&self)
  FnMut:  가변 참조로 캡처 (&mut self)
  FnOnce: 소유권으로 캡처 (self) — 한 번만 호출 가능

Iterator: 지연 계산(lazy)으로 데이터를 처리하는 체인
  map, filter, fold, take, skip, flatten, zip, enumerate, ...",
        why_it_matters: "\
NestJS의 Array 메서드(map, filter, reduce)와 비슷하지만 성능이 다릅니다.
Rust Iterator는 lazy: 실제로 소비(collect/for_each)되기 전까지 계산하지 않습니다.
체인이 길어도 중간 컬렉션을 생성하지 않아서 메모리 효율이 좋습니다.
Iterator를 직접 구현하면 for 루프를 쓸 수 있습니다.",
        code: r#"fn main() {
    // 클로저 기본
    let add = |x: i32, y: i32| x + y;
    println!("{}", add(3, 4));

    // 환경 캡처: n을 &로 빌림
    let n = 5;
    let add_n = |x| x + n;  // Fn: n을 불변 참조로 캡처
    println!("{}", add_n(3));  // 8, n도 여전히 유효

    // FnMut: 가변 참조로 캡처
    let mut count = 0;
    let mut inc = || { count += 1; count };
    println!("{}", inc());  // 1
    println!("{}", inc());  // 2

    // move: 소유권 이전 (스레드에서 필수)
    let text = String::from("hello");
    let show = move || println!("{}", text);  // text 소유권 이동
    show();
    // println!("{}", text);  // 에러: text 소유권 이미 이전됨

    // Iterator 체인
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = data.iter()
        .filter(|&&x| x % 2 == 0)   // 짝수만
        .map(|&x| x * x)            // 제곱
        .take(3)                     // 최대 3개
        .collect();
    println!("{:?}", result);  // [4, 16, 36]

    // fold: 누산 (reduce)
    let sum: i32 = data.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // flat_map: 중첩 컬렉션 펼치기
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{:?}", chars);

    // 직접 반복자 구현
    struct Counter { count: u32 }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            self.count += 1;
            if self.count <= 5 { Some(self.count) } else { None }
        }
    }
    let total: u32 = Counter { count: 0 }.sum();
    println!("Counter sum: {}", total);
}
"#,
        key_points: &[
            "Fn/FnMut/FnOnce: 캡처 방식에 따른 분류 (컴파일러가 자동 선택)",
            "move |...| { ... }: 소유권 이전 — 스레드/비동기 코드에서 필수",
            "Iterator는 lazy: collect()/for_each()까지 실제 계산 안 함",
            "체인 (filter→map→take): 중간 Vec 생성 없이 효율적으로 처리",
        ],
    },
    CSTopic {
        title: "Lifetime ('a) — 참조의 유효 기간",
        category: "추상화",
        explanation: "\
라이프타임은 참조가 '언제까지 유효한지'를 컴파일러에게 알려주는 주석입니다.
대부분의 경우 컴파일러가 자동으로 추론(lifetime elision)합니다.

명시적으로 써야 하는 경우:
  1. 함수가 참조를 받고 참조를 반환할 때
     → 반환된 참조의 유효 기간을 명시해야 함
  2. 구조체가 참조를 필드로 가질 때

'a: '라이프타임 a'라고 읽습니다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
'x와 y 중 적어도 하나만큼은 반환값이 살아있다'는 의미입니다.",
        why_it_matters: "\
댕글링 참조를 컴파일 타임에 막습니다.
다른 언어에서 null pointer exception이 나는 상황을 컴파일러가 잡습니다.
처음에는 가장 어렵게 느껴지지만, 실제로 명시해야 하는 경우는 많지 않습니다.
'static: 프로그램 전체 수명 — 문자열 리터럴이 이 라이프타임",
        code: r#"// 두 참조 중 긴 것을 반환 — 반환값 라이프타임 명시 필요
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a: x와 y의 겹치는 수명 중 짧은 것
    if x.len() > y.len() { x } else { y }
}

// 구조체가 참조를 가질 때
struct Important<'a> {
    content: &'a str,  // content의 수명이 Important보다 길어야 함
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
        let s2 = String::from("short"); // 이 블록에서만 유효
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);  // OK: result가 s2와 같은 스코프
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
    },

    // ────────── 동시성과 메모리 ──────────
    CSTopic {
        title: "Smart Pointers (Box, Rc, Arc)",
        category: "동시성",
        explanation: "\
스마트 포인터: 추가 메타데이터나 기능이 있는 포인터

  Box<T>: Heap 할당, 단일 소유권
    - 재귀 타입, 큰 데이터를 Heap에 이동할 때
    - 트레이트 객체(Box<dyn Trait>)

  Rc<T>: Reference Counting, 단일 스레드 공유 소유권
    - 여러 곳에서 같은 데이터를 읽을 때
    - 멀티스레드 불가!

  Arc<T>: Atomic Reference Counting, 멀티스레드 안전
    - 스레드 간 공유할 때
    - Rc보다 오버헤드 약간 있음

  RefCell<T>: 런타임에 빌림 규칙 검사 (내부 가변성)
  Mutex<T>: 멀티스레드 안전한 내부 가변성",
        why_it_matters: "\
'여러 곳에서 같은 데이터를 소유하고 싶다' → Rc<T> 또는 Arc<T>
Box<dyn Trait>: 런타임에 다양한 타입을 다룰 때 (TypeScript의 인터페이스 배열과 유사)
Rc<RefCell<T>>: 단일 스레드에서 공유 + 가변 (그래프 구조 등)
Arc<Mutex<T>>: 멀티스레드에서 공유 + 가변 (상태 공유)",
        code: r#"use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    // Box<T>: Heap 할당
    let b = Box::new(5);
    println!("Box: {}", *b);  // * 역참조

    // 재귀 타입은 Box 필요 (크기를 컴파일 타임에 알 수 없음)
    // enum List { Cons(i32, Box<List>), Nil }

    // Box<dyn Trait>: 동적 디스패치
    trait Speak { fn speak(&self); }
    struct Dog;
    struct Cat;
    impl Speak for Dog { fn speak(&self) { println!("멍!"); } }
    impl Speak for Cat { fn speak(&self) { println!("야옹!"); } }

    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &animals { a.speak(); }

    // Rc<T>: 공유 소유권 (단일 스레드)
    let a = Rc::new(5);
    let b = Rc::clone(&a);  // 참조 카운트 증가
    let c = Rc::clone(&a);
    println!("Count: {}", Rc::strong_count(&a));  // 3
    drop(b);
    println!("After drop: {}", Rc::strong_count(&a));  // 2

    // Rc<RefCell<T>>: 공유 + 가변 (단일 스레드)
    let shared = Rc::new(RefCell::new(0));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    *clone1.borrow_mut() += 10;  // 런타임 빌림 규칙 검사
    *clone2.borrow_mut() += 20;
    println!("Shared: {}", shared.borrow());  // 30

    // Arc<T>: 멀티스레드 공유 소유권
    let counter = Arc::new(0);
    let counter_clone = Arc::clone(&counter);
    println!("Arc: {}", counter_clone);
}
"#,
        key_points: &[
            "Box<T>: Heap 할당, 재귀 타입, Box<dyn Trait> 동적 디스패치",
            "Rc<T>: 단일 스레드 공유 소유권 / Arc<T>: 멀티스레드 (원자적 카운팅)",
            "RefCell<T>: 컴파일 타임 대신 런타임에 빌림 검사 (내부 가변성)",
            "Rc<RefCell<T>>: 단스레드 공유+가변 / Arc<Mutex<T>>: 멀티스레드 공유+가변",
        ],
    },
    CSTopic {
        title: "Threads & Mutex (멀티스레딩)",
        category: "동시성",
        explanation: "\
Rust 멀티스레딩은 '두려움 없는 동시성(fearless concurrency)'을 지향합니다.
Send, Sync 트레이트로 스레드 안전성을 컴파일 타임에 보장합니다.

  Send: 이 타입을 다른 스레드로 전달할 수 있음
  Sync: 이 타입의 참조를 여러 스레드에서 공유할 수 있음

thread::spawn: 새 스레드 생성 (move 클로저로 소유권 이전)
Mutex<T>: 한 번에 하나의 스레드만 접근 가능한 값
Arc<Mutex<T>>: 멀티스레드 공유 가변 상태의 표준 패턴",
        why_it_matters: "\
Java/Node.js에서 공유 상태 버그(race condition)가 런타임에 발생합니다.
Rust는 Send/Sync 체크로 컴파일 타임에 잡습니다.
Arc<Mutex<T>>가 verbose해 보여도, 멀티스레드 안전성을 타입이 보장합니다.
NestJS에서 이벤트 루프가 단일 스레드인 것과 달리, Rust는 진정한 병렬 실행이 가능합니다.",
        code: r#"use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // 기본 스레드 생성
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("스레드: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("메인: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();  // 스레드 완료 대기

    // move 클로저: 변수 소유권을 스레드로 이전
    let data = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("데이터: {:?}", data);
    });
    handle2.join().unwrap();

    // Arc<Mutex<T>>: 여러 스레드에서 공유 카운터
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // Mutex 잠금 획득
            *num += 1;
        });  // num이 drop되면서 Mutex 해제
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("최종 카운터: {}", *counter.lock().unwrap());  // 5
}
"#,
        key_points: &[
            "thread::spawn + move 클로저: 소유권을 스레드로 이전",
            "Arc<Mutex<T>>: 멀티스레드 공유 가변 상태의 표준 패턴",
            "Mutex::lock(): 잠금 획득, MutexGuard drop 시 자동 해제",
            "Send/Sync: 컴파일 타임에 스레드 안전성 보장 — race condition 불가",
        ],
    },
    CSTopic {
        title: "Channels — 메시지 패싱으로 통신",
        category: "동시성",
        explanation: "\
'메모리를 공유해서 통신하지 말고, 통신해서 메모리를 공유하라' — Go 철학을 Rust도 채용
채널은 스레드 간 메시지를 안전하게 전달합니다.

  mpsc: Multiple Producer, Single Consumer
  (tx, rx) = mpsc::channel()
  tx.send(val): val의 소유권을 채널로 이전
  rx.recv(): 블로킹 수신 (값이 올 때까지 대기)
  rx.try_recv(): 비블로킹 수신 (없으면 Err)

tx는 clone() 가능 → 여러 스레드에서 전송 가능
rx는 clone() 불가 → 수신자는 하나",
        why_it_matters: "\
Arc<Mutex<T>>보다 간단한 경우가 많습니다.
소유권을 이전하므로 채널을 통과한 데이터에는 데이터 경쟁이 없습니다.
Producer-Consumer 패턴, 작업 큐, 결과 수집에 유용합니다.
Go의 goroutine + channel 패턴과 비슷한 아이디어입니다.",
        code: r#"use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 기본 채널 통신
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();  // val의 소유권 이전
        // println!("{}", val);  // 에러! val은 이미 전송됨
    });

    let received = rx.recv().unwrap();  // 블로킹 대기
    println!("Got: {}", received);

    // 여러 값 전송
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "thread"];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    for received in rx2 {  // rx를 반복자처럼 사용
        println!("Received: {}", received);
    }

    // 여러 Producer (tx 복제)
    let (tx3, rx3) = mpsc::channel();
    let tx3_clone = tx3.clone();

    thread::spawn(move || {
        tx3.send(String::from("Producer 1")).unwrap();
    });
    thread::spawn(move || {
        tx3_clone.send(String::from("Producer 2")).unwrap();
    });

    for _ in 0..2 {
        println!("{}", rx3.recv().unwrap());
    }
}
"#,
        key_points: &[
            "mpsc: 다중 생산자(tx.clone()), 단일 소비자(rx)",
            "send(val): 소유권 이전 → 전송 후 원본 접근 불가 = 데이터 경쟁 없음",
            "rx를 for in으로 사용: 모든 tx가 drop될 때까지 반복",
            "Mutex 공유보다 채널이 더 안전하고 구조가 명확한 경우가 많음",
        ],
    },
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_random_topic();
    } else {
        match args[1].as_str() {
            "run" => run_topic_with_code(),
            "list" => list_topics(),
            n if n.parse::<usize>().is_ok() => {
                let idx = n.parse::<usize>().unwrap();
                show_topic_by_index(idx);
            }
            _ => show_random_topic(),
        }
    }
}

fn show_random_topic() {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..TOPICS.len());
    print_topic(&TOPICS[idx], idx + 1);
}

fn show_topic_by_index(idx: usize) {
    if idx == 0 || idx > TOPICS.len() {
        println!("{}", format!("번호는 1~{} 사이로 입력하세요.", TOPICS.len()).bright_red());
        return;
    }
    print_topic(&TOPICS[idx - 1], idx);
}

fn run_topic_with_code() {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..TOPICS.len());
    let topic = &TOPICS[idx];

    print_topic(topic, idx + 1);
    println!("\n{}", "=".repeat(62).cyan());
    println!("{}", "  코드 실행 중...".bright_yellow().bold());
    println!("{}", "=".repeat(62).cyan());

    execute_code(topic.code);
}

fn list_topics() {
    println!("{}\n", "  cs-bite — CS 학습 토픽 목록".bright_cyan().bold());

    let categories = ["기초", "핵심", "추상화", "동시성"];
    for cat in &categories {
        let topics_in_cat: Vec<(usize, &CSTopic)> = TOPICS
            .iter()
            .enumerate()
            .filter(|(_, t)| t.category == *cat)
            .map(|(i, t)| (i, t))
            .collect();

        if !topics_in_cat.is_empty() {
            println!("  {} {}", "▶".yellow(), cat.bright_yellow().bold());
            for (i, topic) in topics_in_cat {
                println!("    {:>2}. {}", i + 1, topic.title.cyan());
            }
            println!();
        }
    }
    println!("{}", format!("총 {} 개의 토픽 | cs-bite <번호> 로 지정해서 볼 수 있습니다.", TOPICS.len()).bright_black());
}

fn print_topic(topic: &CSTopic, idx: usize) {
    let category_color = match topic.category {
        "기초" => topic.category.bright_green(),
        "핵심" => topic.category.bright_blue(),
        "추상화" => topic.category.bright_magenta(),
        "동시성" => topic.category.bright_yellow(),
        _ => topic.category.white(),
    };

    println!();
    println!("{}", "═".repeat(62).bright_black());
    println!(
        "  {} {}  [{}]",
        format!("[{}/{}]", idx, TOPICS.len()).bright_black(),
        topic.title.bright_white().bold(),
        category_color
    );
    println!("{}", "═".repeat(62).bright_black());

    println!();
    println!("{}", "  개념 설명".bright_cyan().bold());
    println!("{}", "  ─────────────────────────────────────────".bright_black());
    for line in topic.explanation.lines() {
        println!("  {}", line);
    }

    println!();
    println!("{}", "  왜 중요한가?".bright_magenta().bold());
    println!("{}", "  ─────────────────────────────────────────".bright_black());
    for line in topic.why_it_matters.lines() {
        println!("  {}", line);
    }

    println!();
    println!("{}", "  코드 예제".bright_green().bold());
    println!("{}", "┌─────────────────────────────────────────────────────────".bright_black());
    for line in topic.code.lines() {
        println!("{} {}", "│".bright_black(), line.green());
    }
    println!("{}", "└─────────────────────────────────────────────────────────".bright_black());

    println!();
    println!("{}", "  핵심 포인트".bright_yellow().bold());
    println!("{}", "  ─────────────────────────────────────────".bright_black());
    for point in topic.key_points {
        println!("  {} {}", "●".yellow(), point);
    }
    println!();
}

fn execute_code(code: &str) {
    let mut child = Command::new("rustc")
        .args(["--edition", "2021", "-", "-o", "/tmp/cs_bite_out"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("rustc를 찾을 수 없습니다. Rust가 설치되어 있나요?");

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(code.as_bytes());
    }

    let output = child.wait_with_output().expect("컴파일 실패");

    if !output.status.success() {
        println!("{}", "  컴파일 에러:".bright_red().bold());
        for line in String::from_utf8_lossy(&output.stderr).lines() {
            println!("  {}", line.bright_red());
        }
        return;
    }

    let output = Command::new("/tmp/cs_bite_out")
        .output()
        .expect("실행 실패");

    if output.status.success() {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            println!("  {}", line.bright_white());
        }
    } else {
        println!("{}", "  실행 에러:".bright_red().bold());
        println!("{}", String::from_utf8_lossy(&output.stderr).bright_red());
    }
}
