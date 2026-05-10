// ═══════════════ CS-BITE TOPIC DATA ═══════════════
// 모든 코드 예제에 학습자 친화적 상세 주석을 포함합니다.

const TOPICS = [

// ──────────────────────────────────────────────────────
// 1. 변수, 상수, 불변성
// ──────────────────────────────────────────────────────
{
  title: "변수, 상수, 불변성",
  category: "기초",
  explanation:
`Rust에서 모든 변수는 기본적으로 불변(immutable)입니다.
값을 바꾸려면 명시적으로 mut 키워드를 붙여야 합니다.
이것은 실수로 값을 덮어쓰는 버그를 컴파일 타임에 막아줍니다.

  let x = 5;            // 불변 변수
  let mut y = 5;        // 가변 변수
  const MAX: u32 = 100; // 상수 (타입 명시 필수, 절대 변경 불가)

변수 섀도잉(shadowing): 같은 이름으로 새 변수를 선언해서
타입도 바꿀 수 있습니다. 이것은 mut와 다른 개념입니다!`,
  whyItMatters:
`'왜 기본이 불변이지?' → 함수형 프로그래밍 철학. 상태 변화를 추적하기 어렵고
버그의 주원인이 되는 '의도치 않은 변경'을 컴파일러가 잡아줍니다.
NestJS에서 const를 쓰는 이유와 같지만, Rust는 기본값이 const입니다.

Rust에서 '안전함'의 출발점이 바로 이 기본 불변성입니다.`,
  diagram:
`  스택 메모리에서 일어나는 일
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
  let spaces = "   ";     // &str
  let spaces = spaces.len(); // usize ← 새 변수!
    ↓
  ┌──────────┐     ┌──────────┐
  │ spaces   │ ──► │ spaces   │
  │ "   "    │     │ 3        │
  │ (&str)   │     │ (usize)  │
  └──────────┘     └──────────┘
   이전 것은 drop    새 타입으로 교체`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Rust의 모든 변수는 기본적으로 불변이다!
//    값을 바꾸려면 "mut" 키워드를 명시해야 한다.
// ══════════════════════════════════════════════════════

fn main() {
    // ─── 1. let: 불변 변수 (기본값) ───
    let x = 5;
    // x = 6;  // ✗ 컴파일 에러! 불변 변수는 재할당 불가
    //         // 에러: "cannot assign twice to immutable variable"
    println!("x = {}", x);  // 출력: x = 5

    // ─── 2. let mut: 가변 변수 ───
    // mut을 명시해야만 값을 변경할 수 있다
    let mut y = 5;
    println!("y = {}", y);   // 출력: y = 5
    y = 10;                   // ✓ mut이 있으므로 재할당 OK
    println!("y = {}", y);   // 출력: y = 10

    // ─── 3. const: 상수 (절대 변경 불가) ───
    // 주의! const는 반드시 타입을 명시해야 한다
    // let과 달리 컴파일 타임에 값이 결정됨 (런타임 함수 호출 불가)
    const MAX_USERS: u32 = 1_000_000;  // _로 자릿수 구분 (가독성 ↑)
    println!("Max: {}", MAX_USERS);    // 출력: Max: 1000000

    // ─── 4. 섀도잉 (Shadowing) ───
    // 같은 이름으로 "새 변수"를 선언 → 이전 변수는 가려짐
    // ★ mut과 다른 점: 타입을 완전히 다르게 바꿀 수 있다!
    let spaces = "   ";         // 타입: &str (문자열 슬라이스)
    let spaces = spaces.len();  // 타입: usize (숫자) — 에러 없음!
    // 만약 mut이었다면 같은 타입(&str)만 재할당 가능했을 것
    println!("Spaces: {}", spaces);  // 출력: Spaces: 3

    // 섀도잉으로 값 변환 파이프라인 만들기
    let x = 5;         // 첫 번째 x = 5
    let x = x + 1;     // 두 번째 x = 6 (이전 x값을 사용해 계산)
    let x = x * 2;     // 세 번째 x = 12
    println!("Shadowed x = {}", x);  // 출력: Shadowed x = 12
}`,

  keyPoints: [
    "let: 불변 / let mut: 가변 — 기본이 불변이라 실수가 줄어든다",
    "const: 타입 필수, 절대 변경 불가, 전역 스코프 가능",
    "섀도잉: 같은 이름으로 다른 타입의 변수를 선언 가능 (mut과 다름)",
    "불변 설계 → 다중 참조가 안전 (나중에 Borrowing과 연결)"
  ],
  comparisons: [
    ["header","let (불변)","let mut (가변)"],
    ["diff","재할당 불가","재할당 가능"],
    ["diff","컴파일러가 보호","개발자가 의도 표현"],
    ["equal","스택에 저장","스택에 저장"],
    ["diff","기본값","명시적 선언 필요"]
  ]
},

// ──────────────────────────────────────────────────────
// 2. 기본 타입 시스템
// ──────────────────────────────────────────────────────
{
  title: "기본 타입 시스템",
  category: "기초",
  explanation:
`Rust는 정적 타입 언어입니다. 대부분의 경우 컴파일러가 타입을 추론하지만,
명시적으로 적어주면 코드의 의도가 더 명확해집니다.

  정수: i8, i16, i32(기본), i64, i128, u8, u16, u32, u64
  부동소수점: f32, f64(기본)
  불리언: bool  (true/false)
  문자: char    (유니코드, 4바이트)
  튜플: (i32, f64, char)
  배열: [i32; 5]  (고정 크기, 스택에 저장)

타입 캐스팅은 as 키워드를 사용하며, 명시적으로만 가능합니다.
Rust에는 암묵적 타입 변환이 없습니다!`,
  whyItMatters:
`TypeScript와 달리 any가 없습니다. 타입이 맞지 않으면 컴파일되지 않습니다.
i32가 기본인 이유: 대부분의 정수 연산에 충분하고, 오버플로우 체크가 가능합니다.
배열 크기가 타입에 포함되는 이유: 스택 할당 크기를 컴파일 타임에 알아야 하기 때문입니다.`,
  diagram:
`  Rust 기본 타입의 메모리 크기
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
   [0] [1] [2]`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Rust는 정적 타입, 암묵적 변환 없음, any 없음
// ══════════════════════════════════════════════════════

fn main() {
    // ─── 정수: 기본 i32 (부호 있는 32비트) ───
    let x: i32 = 42;             // 명시적 타입
    let inferred = 100;          // 추론됨 → i32
    let big: i64 = 9_000_000_000; // i64: 큰 수 (9조)
    let byte: u8 = 255;          // u8: 0~255만 가능 (바이트 단위)

    // ─── 부동소수점: 기본 f64 ───
    let pi: f64 = 3.14159;       // f64: 배정도 (기본, 더 정밀)
    let small: f32 = 2.0_f32;    // f32: 단정도 (메모리 절약 시 사용)

    // ─── 불리언 ───
    let is_active: bool = true;
    println!("Active: {}", is_active);

    // ─── 문자: 유니코드! 한글도 한 문자로 처리 ───
    let ch: char = '한';          // char = 4바이트 유니코드 스칼라
    println!("Char: {}", ch);    // TS에서는 string인데 Rust는 char!

    // ─── 튜플: 여러 타입을 하나로 묶기 ───
    let tup: (i32, f64, bool) = (500, 6.4, true);
    // 구조분해 (JS의 const [a,b] = arr과 비슷)
    let (a, b, c) = tup;
    println!("{}, {}, {}", a, b, c);  // 500, 6.4, true
    // 인덱스로도 접근 가능
    println!("First: {}", tup.0);     // 500

    // ─── 배열: 고정 크기, 같은 타입만 ───
    // [T; N] → T는 타입, N은 개수 (컴파일 타임에 크기 결정)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Length: {}", arr.len()); // 5
    println!("Third: {}", arr[2]);    // 3 (0부터 시작)
    // arr[5] → 런타임 패닉! (TS처럼 undefined가 아님)

    // ─── 타입 캐스팅: 반드시 명시적 (as) ───
    let n: i32 = 1000;
    let m = n as i16;  // i32 → i16으로 강제 변환 (데이터 손실 주의!)
    println!("Cast: {}", m);          // 1000 (i16 범위 내라 OK)

    // ─── 오버플로우: 디버그 모드에서 패닉 발생 ───
    // let overflow: u8 = 256; // ✗ 컴파일 에러! u8 최대는 255
    let checked: u8 = 255;            // OK: u8의 최대값
    println!("u8 max: {}", checked);
}`,

  keyPoints: [
    "정수 기본 i32, 부동소수점 기본 f64 — 명시하지 않으면 컴파일러가 추론",
    "배열 [T; N]: 크기가 타입에 포함됨 (컴파일 타임에 스택 크기 확정)",
    "튜플: 다른 타입 혼합 가능, 인덱스(.0, .1)나 구조분해로 접근",
    "as 캐스팅: 암묵적 변환 없음, 반드시 명시적으로"
  ],
  comparisons: [
    ["header","Rust","TypeScript"],
    ["diff","i32, u32, i64...","number 하나"],
    ["diff","char (4바이트 유니코드)","string (1글자도 string)"],
    ["diff","배열 [T; N] 크기 고정","any[] 가변 배열"],
    ["diff","any 타입 없음","any 가능"],
    ["win","오버플로우 컴파일 체크","런타임才 체크"]
  ]
},

// ──────────────────────────────────────────────────────
// 3. 함수와 반환값
// ──────────────────────────────────────────────────────
{
  title: "함수와 반환값",
  category: "기초",
  explanation:
`Rust 함수는 fn 키워드로 정의하며, 매개변수와 반환 타입을 명시합니다.
중요한 특징: 마지막 표현식이 세미콜론 없이 끝나면 그것이 반환값이 됩니다.
return 키워드도 쓸 수 있지만, 관용적으로 마지막 표현식을 사용합니다.

  문(statement): 값을 반환하지 않는 코드 (let x = 5;)
  식(expression): 값을 평가하는 코드 (5 + 3, if...else, 블록{})

Rust에서 if/else, 블록 {} 자체가 '식'이어서 값을 반환할 수 있습니다.
이것이 세미콜론의 역할을 이해하는 핵심입니다!`,
  whyItMatters:
`세미콜론이 의미를 바꿉니다! 실수하기 쉬운 부분입니다.
- add(a, b) { a + b }  → a+b 반환 (Ok)
- add(a, b) { a + b; } → () (unit) 반환, 타입 불일치 에러!

이 구분은 처음에는 번거롭지만, 표현식 기반 코드가 간결하고 버그가 적습니다.`,
  diagram:
`  세미콜론의 마법 (가장 자주 하는 실수!)
  ──────────────────────────────────

  ✓ 올바른 코드:             ✗ 세미콜론 추가 시:
  fn add(a:i32,b:i32)->i32{  fn add(a:i32,b:i32)->i32{
      a + b                       a + b;
  }                          }
  반환: i32 (a+b)            반환: () ← 타입 불일치 에러!

  블록도 식이다:
  let result = {
      let x = 3;      ← 문 (값 없음)
      let y = 4;      ← 문 (값 없음)
      x * x + y * y   ← 식 (반환값, 세미콜론 없음!)
  };
  result = 25`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 마지막 줄에 세미콜론 없으면 반환값!
//    세미콜론 있으면 () 반환 → 타입 불일치 에러
// ══════════════════════════════════════════════════════

// 반환 타입은 -> 뒤에 명시
fn add(a: i32, b: i32) -> i32 {
    a + b  // ★ 세미콜론 없음 = 이 값을 반환한다는 뜻
           // a + b; 라고 쓰면 () 반환 → 컴파일 에러!
}

// 반환값 없으면 -> 생략 (사실은 -> () 와 같음)
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 조기 반환할 때는 return 키워드 사용
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;  // 조기 반환: return 필요
    }
    Some(a / b)  // 마지막 표현식 → return 없이 반환
}

fn main() {
    // 기본 함수 호출
    let sum = add(3, 4);
    println!("Sum: {}", sum);  // 출력: Sum: 7

    greet("Alice");  // 반환값 없는 함수

    // ─── if/else도 "식"이라 값을 반환할 수 있다! ───
    // TS의 삼항 연산자(a ? b : c) 대신 if-else 블록을 직접 사용
    let score = 85;
    let grade = if score >= 90 {
        "A"          // 세미콜론 없음! (값을 반환)
    } else if score >= 80 {
        "B"          // 세미콜론 없음!
    } else {
        "C"
    };
    println!("Grade: {}", grade);  // 출력: Grade: B

    // ─── 블록 {}도 식이다! ───
    let result = {
        let x = 3;       // 문 (세미콜론 있음 → 값 반환 안 함)
        let y = 4;       // 문
        x * x + y * y    // ★ 식 (세미콜론 없음 → 이 값이 반환됨)
    };
    println!("Result: {}", result);  // 출력: Result: 25

    // ─── match도 식이다! ───
    let n = 42;
    let parity = match n % 2 {
        0 => "짝수",     // 각 갈래도 세미콜론 없이 값 반환
        _ => "홀수",     // _ = 와일드카드 (나머지 모두)
    };
    println!("{} is {}", n, parity);  // 출력: 42 is 짝수
}`,

  keyPoints: [
    "마지막 표현식에 세미콜론 없음 = 반환값 (return 생략 가능)",
    "세미콜론 있으면 ()을 반환 — 반환 타입 불일치 시 컴파일 에러",
    "if/else, 블록 {}, match도 표현식이라 값을 가짐",
    "-> () 생략 가능, 반환값 없는 함수도 사실 ()을 반환"
  ],
  comparisons: [
    ["header","식 (Expression)","문 (Statement)"],
    ["left","값을 평가한다","값을 반환하지 않는다"],
    ["left","세미콜론 없음","세미콜론 있음"],
    ["left","반환값으로 사용 가능","반환값으로 사용 불가"],
    ["left","a + b → 8","let x = 5; → ()"]
  ]
},

// ──────────────────────────────────────────────────────
// 4. String vs &str
// ──────────────────────────────────────────────────────
{
  title: "String vs &str (문자열 두 종류)",
  category: "기초",
  explanation:
`Rust 초학자가 가장 헷갈리는 것 중 하나입니다. 문자열이 두 종류입니다.

  &str (문자열 슬라이스)
    - 불변, 고정 크기
    - 프로그램 바이너리 또는 String의 일부를 가리킴
    - 메모리: 스택에 (포인터, 길이)만 저장

  String (소유된 문자열)
    - 가변, 동적 크기
    - Heap에 할당
    - 런타임에 생성/수정 가능
    - 메모리: 스택에 (포인터, 길이, 용량) + Heap에 실제 데이터

함수 매개변수는 대부분 &str로 받습니다 (String도 &str로 자동 변환 가능).`,
  whyItMatters:
`NestJS에서 string 하나로 되던 것이 왜 두 종류인가?
→ 메모리 위치와 소유권 때문입니다.
&str: '이 문자열을 그냥 보겠다' (참조, 복사 없음, 빠름)
String: '내가 이 문자열을 소유하겠다' (Heap 할당, 수정 가능)
API 설계 시 &str로 받으면 유연합니다.`,
  diagram:
`  String vs &str 메모리 구조
  ──────────────────────────────────

  let s: &str = "hello";
  ┌──────────────┐
  │ s (스택)     │     "hello" (바이너리 영역)
  │ ptr ─────────┼────► h e l l o \\0
  │ len: 5       │
  └──────────────┘

  let s: String = String::from("hello");
  ┌──────────────┐     ┌───────────────────┐
  │ s (스택)     │     │ Heap              │
  │ ptr ─────────┼────►│ h e l l o \\0      │
  │ len: 5       │     │                   │
  │ cap: 5       │     │                   │
  └──────────────┘     └───────────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: &str = 참조(빌림), String = 소유(Heap)
//    함수 파라미터는 &str로 받으면 둘 다 수용 가능!
// ══════════════════════════════════════════════════════

fn main() {
    // ─── &str: 문자열 리터럴 ───
    // 프로그램 바이너리에 저장됨 → Heap 할당 없음 → 빠르다!
    // 불변이며 크기를 컴파일 타임에 알 수 있음
    let s1: &str = "Hello, world!";
    println!("{}", s1);

    // ─── String: Heap 할당 문자열 ───
    // 런타임에 크기가 변할 수 있음 → push, push_str 가능
    let mut s2: String = String::from("Hello");
    s2.push_str(", world!");  // 문자열 추가 (Heap 재할당 발생 가능)
    s2.push('!');             // 단일 문자 추가
    println!("{}", s2);       // 출력: Hello, world!!

    // ─── String → &str 변환 (Deref 자동 변환) ───
    let s3: String = String::from("Rust");
    let s4: &str = &s3;        // &String → &str 자동 변환
    let s5: &str = &s3[0..2];  // 슬라이스: 첫 2바이트만 참조
    println!("{}, {}", s4, s5);  // Rust, Ru

    // ─── ★ 함수 설계: &str로 받으면 String도 &str도 수용 ───
    fn say(msg: &str) {          // &str 파라미터
        println!("Say: {}", msg);
    }
    say("리터럴");    // ✓ &str 직접 전달
    say(&s2);         // ✓ &String → &str 자동 변환 (Deref)

    // ─── + 연산자: 소유권 이동 주의! ───
    let a = String::from("Hello");
    let b = String::from(" World");
    let c = a + &b;   // ★ a의 소유권이 이동됨! a는 더 이상 사용 불가
    println!("{}", c); // 출력: Hello World
    // println!("{}", a); // 에러! a는 이미 이동됨

    // ─── format!: 소유권 이동 없이 안전하게 연결 ───
    let d = String::from("Hello");
    let e = String::from(" World");
    let f = format!("{}{}", d, e);  // d, e 모두 여전히 유효!
    println!("{}", f);  // 출력: Hello World
    println!("{}", d);  // OK! d도 여전히 사용 가능
}`,

  keyPoints: [
    "&str: 불변 참조, Heap 할당 없음 — 빠르고 가볍다",
    "String: Heap 할당, 가변, 소유권을 가짐",
    "함수 매개변수는 &str로 받으면 String/&str 모두 수용 가능",
    "format!(): 소유권 이동 없이 문자열 조합하는 안전한 방법"
  ],
  comparisons: [
    ["header","&str (문자열 슬라이스)","String (소유 문자열)"],
    ["diff","불변","가변 (push, push_str)"],
    ["diff","바이너리/참조 영역","Heap 할당"],
    ["diff","ptr + len (2개 필드)","ptr + len + cap (3개 필드)"],
    ["left","빠르고 가벼움","생성/해제 비용 있음"],
    ["diff","함수 파라미터에 추천","데이터 소유/수정 시 필요"]
  ]
},

// ──────────────────────────────────────────────────────
// 5. 제어 흐름
// ──────────────────────────────────────────────────────
{
  title: "제어 흐름 (if, loop, while, for)",
  category: "기초",
  explanation:
`Rust의 제어 흐름은 C/TypeScript와 비슷하지만 몇 가지 중요한 차이가 있습니다.

  1. if/else는 '식(expression)' — 값을 반환할 수 있음
  2. 조건문에 괄호 불필요 (if x > 5 { ... })
  3. loop: 무한 루프, break로 값을 반환 가능
  4. while: 조건 기반 반복
  5. for ... in: 반복자 기반 (가장 관용적)
  6. range: 0..5 (0~4), 0..=5 (0~5)

Rust에서는 for 루프에서 인덱스보다 반복자를 권장합니다.
반복자 최적화로 C의 for 루프와 같은 성능을 냅니다.`,
  whyItMatters:
`loop에서 break value로 반환값을 뽑을 수 있습니다 — 재시도 로직에서 유용합니다.
for in을 선호하는 이유: 배열 범위 초과(off-by-one) 버그가 없고,
컴파일러가 최적화하여 수동 인덱스 루프와 동일한 성능을 보장합니다.`,
  diagram:
`  반복문의 진화: 안전해지는 과정
  ──────────────────────────────────

  ① while (위험)         ② for + 인덱스 (나음)    ③ for in (최고)
  let mut i = 0;         for i in 0..3 {         for item in &items {
  while i < 3 {              println!(              println!(
      println!(i);               items[i]);            item);
      i += 1;               }                       }
  }
  위험: i 업데이트        나음: 범위 고정          최고: 인덱스 불필요
  누락시 무한루프         하지만 인덱스 접근       안전하고 관용적`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: if/else는 식(값 반환), for in이 관용적
// ══════════════════════════════════════════════════════

fn main() {
    // ─── if/else: 식이므로 값을 변수에 바인딩 가능 ───
    // 괄호 필요 없음! (if (n%2==0) 가 아니라 if n%2==0)
    let n = 7;
    let label = if n % 2 == 0 { "짝수" } else { "홀수" };
    println!("{} is {}", n, label);  // 출력: 7 is 홀수

    // ─── loop: 무한 루프 + break로 값 반환 ───
    // 재시도 로직, 상태 머신 등에서 유용
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2;  // ★ break 뒤에 값 → loop 전체가 그 값을 반환
        }
    };
    println!("loop result: {}", result);  // 출력: 10

    // ─── while: 조건이 참인 동안 반복 ───
    let mut x = 0;
    while x < 3 {
        print!("{} ", x);  // 출력: 0 1 2
        x += 1;
    }
    println!();

    // ─── for in range: 가장 관용적인 반복 ───
    // 0..5 → 0,1,2,3,4 (5 미포함)
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();  // 출력: 0 1 2 3 4

    // ─── for in 배열: 인덱스 없이 안전하게 ───
    let fruits = ["사과", "바나나", "오렌지"];
    for fruit in &fruits {     // &로 빌림 (소유권 이동 안 함)
        println!("- {}", fruit);
    }

    // 인덱스가 필요하면 enumerate()
    for (i, fruit) in fruits.iter().enumerate() {
        println!("[{}] {}", i, fruit);  // [0] 사과, [1] 바나나 ...
    }

    // ─── 라벨('label): 중첩 루프 한 번에 탈출 ───
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                break 'outer;  // ★ 'outer 라벨이 붙은 루프까지 탈출!
            }
            print!("({},{}) ", x, y);
        }
    }
    println!();  // 출력: (0,0) (0,1) (0,2) (1,0)

    // ─── 역방향 반복: .rev() ───
    // 0..=5 → 0~5 포함
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();  // 출력: 5 4 3 2 1
}`,

  keyPoints: [
    "if/else는 표현식 — 삼항 연산자 대신 사용",
    "loop { break value } — 반환값 있는 무한 루프",
    "for in: 인덱스 없이 안전하게 반복, enumerate()로 인덱스 추가",
    "0..5 (0~4), 0..=5 (0~5) — off-by-one 버그 방지"
  ],
  comparisons: [
    ["header","Rust","TypeScript"],
    ["diff","if 식 (값 반환)","if 문 (값 없음)"],
    ["left","loop + break value","while(true) + break"],
    ["left","for x in iter","for...of"],
    ["left","0..5 범위 타입","i < 5 수동 체크"],
    ["left","'label: break 'label","label: break label"]
  ]
},

// ──────────────────────────────────────────────────────
// 6. 컬렉션
// ──────────────────────────────────────────────────────
{
  title: "Vec<T>와 HashMap<K,V> (컬렉션)",
  category: "기초",
  explanation:
`두 가지 가장 자주 쓰는 컬렉션입니다.

  Vec<T>: 동적 배열
    - Heap에 저장, 크기 동적 변경
    - v[0] (패닉 가능) vs v.get(0) (안전, Option 반환)

  HashMap<K, V>: 해시맵
    - 키-값 저장, O(1) 평균 조회
    - get()은 Option<&V>를 반환 — 키가 없으면 None`,
  whyItMatters:
`Vec은 단순 배열보다 훨씬 많이 씁니다. 동적 크기 + 반복자와 조합하면 강력합니다.
HashMap은 NestJS의 Map<string, T>와 같은 역할.
주의: HashMap은 std::collections에서 import 필요.`,
  diagram:
`  Vec<T>의 메모리 구조
  ──────────────────────────────────

  let v = vec![10, 20, 30, 40, 50];

  스택:                   Heap (연속 메모리):
  ┌──────────┐           ┌───┬───┬───┬───┬───┐
  │ ptr ─────┼──────────►│10 │20 │30 │40 │50 │
  │ len: 5   │           └───┴───┴───┴───┴───┘
  │ cap: 8   │            [0] [1] [2] [3] [4]
  └──────────┘            ↑ cap-len 만큼 여유`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Vec은 동적 배열, HashMap은 키-값 저장
//    안전한 접근은 .get() 사용 (Option 반환)
// ══════════════════════════════════════════════════════

use std::collections::HashMap;

fn main() {
    // ─── Vec 생성 방법 2가지 ───
    let mut v: Vec<i32> = Vec::new();  // 방법 1: 빈 Vec 생성
    v.push(1);  // push로 요소 추가
    v.push(2);
    v.push(3);

    let v2 = vec![10, 20, 30];  // 방법 2: 매크로로 초기화 (간편!)

    // ─── Vec 접근: 두 가지 방법 ───
    println!("v[0] = {}", v[0]);     // 직접 인덱스 — 범위 벗어나면 패닉!
    println!("{:?}", v.get(1));       // .get() — Option<&T> 반환 → 안전
    // v.get(99) → None (패닉 없이 안전하게 처리)
    println!("len: {}", v.len());

    // ─── Vec 반복 ───
    for x in &v {        // &로 빌림 (소유권 이동 안 함)
        print!("{} ", x);
    }
    println!();

    // ─── Vec 변환 메서드 ───
    v.push(99);          // 요소 추가
    v.sort();            // 정렬
    v.dedup();           // 중복 제거 (정렬 후에 사용해야 함)
    println!("{:?}", v);

    // ─── HashMap 생성 ───
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 90);   // 키-값 삽입
    scores.insert(String::from("Bob"), 75);

    // ─── HashMap 조회: Option<&V> 반환 ───
    let alice = scores.get("Alice");  // Some(&90)
    match alice {
        Some(score) => println!("Alice: {}", score),  // 출력됨
        None => println!("Not found"),
    }
    // scores.get("없는키") → None (에러 아님!)

    // ─── entry().or_insert(): upsert 패턴 ───
    // 키가 없으면 삽입, 있으면 아무 것도 안 함
    scores.entry(String::from("Charlie")).or_insert(80);
    scores.entry(String::from("Alice")).or_insert(0);  // 이미 있으므로 무시

    // ─── HashMap 반복 ───
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 키 존재 여부 확인
    println!("Bob 있나? {}", scores.contains_key("Bob"));  // true
}`,

  keyPoints: [
    "vec![...]: 매크로로 간단 초기화 / Vec::new()로 빈 Vec 생성",
    "v[0]: 패닉 가능 / v.get(0): Option<&T> 반환 (안전)",
    "HashMap::get(): Option<&V> — 없는 키 접근을 안전하게 처리",
    "entry().or_insert(): 없으면 삽입, 있으면 기존 값 유지 (upsert)"
  ],
  comparisons: [
    ["header","Vec<T>","HashMap<K,V>"],
    ["left","순서 보장","순서 보장 안 됨"],
    ["left","인덱스 접근 O(1)","키 접근 O(1) 평균"],
    ["left","vec![] 매크로","HashMap::new()"],
    ["left","push, pop, sort","insert, get, entry"],
    ["left","연속 메모리","해시 버킷 구조"]
  ]
},

// ──────────────────────────────────────────────────────
// 7. Struct & impl
// ──────────────────────────────────────────────────────
{
  title: "Struct & impl (데이터와 메서드)",
  category: "핵심",
  explanation:
`Struct는 연관된 데이터를 묶는 사용자 정의 타입입니다.
impl 블록에서 해당 타입의 메서드를 정의합니다.

  연관 함수: self 없음 → Type::new() 처럼 호출
  메서드: &self 또는 &mut self → instance.method() 처럼 호출

NestJS 클래스와 비슷하지만, 상속이 없습니다.
대신 트레이트(Trait)로 공통 동작을 정의합니다.`,
  whyItMatters:
`클래스가 없는데 어떻게 OOP를 하냐고? Rust의 답: Struct + Trait 조합입니다.
상속 없음 → 합성(Composition)을 권장 → 더 유연하고 버그가 적습니다.
#[derive(Debug, Clone)]으로 자주 쓰는 트레이트를 자동 구현할 수 있습니다.`,
  diagram:
`  self의 3가지 형태
  ──────────────────────────────────

  &self           &mut self          self
  (불변 참조)      (가변 참조)        (소유권)
  읽기만 가능      읽기/쓰기 가능     소비됨 (값 이동)

  fn name(&self)   fn deactivate    fn into_name
  -> &str          (&mut self)      (self) -> String
                   self.active=false`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Struct = 데이터, impl = 메서드
//    &self = 읽기, &mut self = 쓰기, self = 소비
// ══════════════════════════════════════════════════════

// #[derive(...)]로 자주 쓰는 트레이트를 자동 구현
#[derive(Debug, Clone)]  // Debug: {:?} 출력, Clone: .clone() 가능
struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    // ─── 연관 함수 (생성자, self 없음) ───
    // Type::new() 형태로 호출 — JS의 static 메서드와 비슷
    fn new(name: &str, age: u32) -> Self {  // Self = 현재 타입(User)
        User {
            name: String::from(name),
            age,            // ★ 축약: 변수명과 필드명이 같으면 생략 가능
            active: true,
        }
    }

    // ─── &self 메서드: 읽기 전용 ───
    // 호출 후에도 user를 계속 사용 가능
    fn greeting(&self) -> String {
        format!("안녕하세요, {}세 {}입니다.", self.age, self.name)
    }

    // ─── &mut self 메서드: 값 변경 ───
    // self를 통해 내부 데이터 수정
    fn deactivate(&mut self) {
        self.active = false;
    }

    // ─── self 메서드: 소유권 소비 ───
    // 호출하면 user는 더 이상 사용 불가! (값이 이동됨)
    fn into_name(self) -> String {
        self.name  // User가 해체되고 name(String)만 반환됨
    }
}

fn main() {
    // 연관 함수로 생성
    let mut user = User::new("Alice", 30);
    println!("{}", user.greeting());  // 출력: 안녕하세요, 30세 Alice입니다.

    // &mut self 메서드로 값 변경
    user.deactivate();
    println!("{:?}", user);  // #[derive(Debug)] 덕분에 출력 가능
    // 출력: User { name: "Alice", age: 30, active: false }

    // ─── 구조체 업데이트 문법: 일부 필드만 변경한 새 인스턴스 ───
    let user2 = User {
        name: String::from("Bob"),  // name만 변경
        ..user                       // ★ 나머지 필드(age, active)는 user에서 복사
    };
    println!("{:?}", user2);

    // ─── Clone으로 깊은 복사 ───
    let user3 = user2.clone();  // 완전히 독립된 복사본 생성
    println!("{:?}", user3);

    // ─── self 소비 예제 ───
    let user4 = User::new("Charlie", 25);
    let name = user4.into_name();  // user4가 소비됨!
    println!("이름만 추출: {}", name);
    // println!("{:?}", user4);  // 에러! user4는 이미 소비됨
}`,

  keyPoints: [
    "&self: 불변 참조 메서드 / &mut self: 가변 메서드 / self: 소유권 소비",
    "연관 함수 Type::new(): 관용적 생성자 패턴",
    "#[derive(Debug, Clone, PartialEq)]: 자주 쓰는 트레이트 자동 구현",
    "구조체 업데이트 문법(..user): 일부 필드만 변경하고 나머지 복사"
  ],
  comparisons: [
    ["header","Rust Struct","TS/JS Class"],
    ["diff","상속 없음","extends 가능"],
    ["diff","impl 블록에 메서드","클래스 안에 메서드"],
    ["diff","derive로 자동 구현","수동 구현"],
    ["win","컴파일 타임 필드 체크","런타임才 체크"],
    ["left","Self 생성자 패턴","constructor"]
  ]
},

// ──────────────────────────────────────────────────────
// 8. Enum & Pattern Matching
// ──────────────────────────────────────────────────────
{
  title: "Enum & Pattern Matching (열거형과 패턴 매칭)",
  category: "핵심",
  explanation:
`Rust의 Enum은 각 변형(variant)이 데이터를 가질 수 있는
'대수적 데이터 타입(Algebraic Data Type)'입니다.

match는 모든 경우를 반드시 처리해야 합니다 (exhaustiveness check).
처리 안 하면 컴파일 에러! → 버그를 설계 단계에서 잡습니다.
Option<T>와 Result<T, E> 자체가 Enum으로 구현되어 있습니다.`,
  whyItMatters:
`TypeScript의 union type + 구조분해를 합쳐놓은 것과 비슷합니다.
패턴 매칭은 단순 switch-case가 아니라 값, 구조, 타입을 동시에 분해합니다.
컴파일러가 빠진 케이스를 잡아주므로 버그가 줄어듭니다.`,
  diagram:
`  Enum의 메모리 레이아웃 (태그 + 페이로드)
  ──────────────────────────────────

  enum Shape {
      Circle(f64),         // f64 1개
      Rectangle(f64, f64), // f64 2개
      Triangle{b:f64,h:f64}, // f64 2개
  }

  메모리 (가장 큰 variant 기준):
  ┌────────┬────────────────────────┐
  │ 태그   │ 페이로드 (최대 16B)     │
  │ 1 byte │ f64(8B)  │ f64(8B)    │
  └────────┴────────────────────────┘
   Circle=0, Rect=1, Tri=2`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Enum variant는 데이터를 가질 수 있다
//    match는 모든 경우 처리 강제 (빠뜨리면 컴파일 에러!)
// ══════════════════════════════════════════════════════

#[derive(Debug)]
enum Shape {
    Circle(f64),                            // Tuple형: 값만 저장
    Rectangle(f64, f64),                    // Tuple형: 여러 값
    Triangle { base: f64, height: f64 },    // Struct형: 이름 있는 필드
}

impl Shape {
    // ─── match로 모든 variant 처리 ───
    fn area(&self) -> f64 {
        match self {
            // 패턴 분해: Circle(r)에서 r을 바인딩
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            // Struct형은 { 필드명 }으로 바인딩
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "원",        // _ = 값 무시
            Shape::Rectangle(_, _) => "직사각형",
            Shape::Triangle { .. } => "삼각형", // .. = 나머지 무시
        }
    }
}

fn main() {
    // 각 variant 생성
    let shapes: Vec<Shape> = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    for shape in &shapes {
        println!("{}: 넓이 = {:.2}", shape.name(), shape.area());
    }
    // 출력: 원: 넓이 = 78.54
    //       직사각형: 넓이 = 24.00
    //       삼각형: 넓이 = 6.00

    // ─── if let: 한 가지 패턴만 처리할 때 ───
    // match의 _ => () 를 생략할 수 있어 간결
    let s = Shape::Circle(3.0);
    if let Shape::Circle(r) = s {
        println!("원의 반지름: {}", r);  // 출력: 원의 반지름: 3
    }
    // Circle이 아닌 경우는 그냥 무시됨

    // ─── 패턴 가드(guard): 추가 조건 ───
    let x = 7;
    match x {
        n if n < 0 => println!("음수"),    // guard: n if 조건
        0 => println!("영"),               // 리터럴 매칭
        1..=9 => println!("한 자리"),       // ★ 범위 패턴 (1~9)
        _ => println!("두 자리 이상"),      // _ = 와일드카드 (나머지 전부)
    }
}`,

  keyPoints: [
    "Enum variant는 데이터를 가질 수 있음 (Tuple형, Struct형)",
    "match는 모든 경우 처리 강제 (컴파일 타임 exhaustiveness check)",
    "if let: 한 패턴만 처리할 때 match보다 간결",
    "1..=9: 범위 패턴 / _ : 와일드카드 / guard: if 조건 추가"
  ],
  comparisons: [
    ["header","Rust Enum","TypeScript Union"],
    ["diff","태그+페이로드 메모리","런타임에만 구분"],
    ["win","match 완전성 강제","switch break 누락 가능"],
    ["left","variant가 데이터 보유","discriminant만"],
    ["left","if let 간결 패턴","typeof 체크 필요"],
    ["win","컴파일 타임 분해","런타임 타입 가드"]
  ]
},

// ──────────────────────────────────────────────────────
// 9. Ownership
// ──────────────────────────────────────────────────────
{
  title: "Ownership (소유권) — Rust의 심장",
  category: "핵심",
  explanation:
`Rust에는 GC(가비지 컬렉터)가 없습니다. 대신 '소유권' 규칙으로 메모리를 관리합니다.

소유권 3대 규칙:
  1. 모든 값에는 소유자(owner)가 있다
  2. 한 번에 소유자는 하나뿐이다
  3. 소유자가 스코프를 벗어나면 값은 drop()된다

Move (이동): 대입하면 소유권이 이동, 이전 변수는 무효화
Copy: i32, bool, char 등 스택 타입은 복사됨 (소유권 이동 없음)
Clone: Heap 타입을 복사하려면 .clone() 명시 필요`,
  whyItMatters:
`GC 없이 메모리 안전성을 보장하는 Rust의 핵심 메커니즘입니다.
C/C++는 수동 관리(버그 많음), Java/Go는 GC(런타임 오버헤드), Rust는 컴파일 타임 검사.
처음에 어색하지만 익숙해지면 '컴파일러가 메모리 버그를 잡아준다'는 안정감이 큽니다.`,
  diagram:
`  Move vs Copy — 핵심 시각화
  ──────────────────────────────────

  ⚡ MOVE (Heap 데이터)
  let s1 = String::from("hello");
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
                              └──────┘
  s1 사용 불가! s2만 유효!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 소유권은 하나! 이동 후 원래 변수는 무효
//    스택 타입(i32 등)은 Copy → 둘 다 유효
//    Heap 타입(String 등)은 Move → 원본 무효
// ══════════════════════════════════════════════════════

fn main() {
    // ─── Move (이동): Heap 데이터 ───
    let s1 = String::from("hello");  // s1이 "hello"를 소유
    let s2 = s1;  // ★ 소유권이 s2로 이동! (shallow copy 아님!)
    // println!("{}", s1);  // ✗ 컴파일 에러! s1은 더 이상 유효하지 않음
    println!("{}", s2);              // ✓ s2만 유효

    // ─── Copy (복사): 스택 데이터 ───
    // i32, bool, char, f64 등은 Copy 트레이트 구현 → 대입해도 원본 유효
    let x = 5;
    let y = x;    // 복사! (move가 아님)
    println!("{} {}", x, y);  // ✓ 둘 다 유효! 출력: 5 5

    // ─── Clone (명시적 깊은 복사) ───
    let s3 = String::from("world");
    let s4 = s3.clone();  // ★ .clone()으로 Heap 데이터까지 복사
    println!("{} {}", s3, s4);  // ✓ 둘 다 유효 (비용은 있지만 명확)

    // ─── 함수로 소유권 이동 ───
    let s5 = String::from("Rust");
    takes_ownership(s5);    // s5의 소유권이 함수로 이동
    // println!("{}", s5);  // ✗ 에러! s5는 이미 이동됨

    let n = 10;
    makes_copy(n);          // i32는 Copy이므로 n은 여전히 유효
    println!("{}", n);      // ✓ OK! 출력: 10

    // ─── 함수에서 소유권 반환 ───
    let s6 = gives_ownership();  // 함수가 만든 String의 소유권을 받음
    println!("{}", s6);          // ✓ OK! 출력: given
}

// String을 받으면 소유권이 이곳으로 옴
fn takes_ownership(s: String) {
    println!("Owned: {}", s);
}  // ★ s가 스코프를 벗어나 drop() 호출 → 메모리 해제!

// i32는 Copy → 원본에 영향 없음
fn makes_copy(n: i32) {
    println!("Copied: {}", n);
}  // n은 복사본이므로 원본에 영향 없음

// String을 반환하면 호출자에게 소유권이 이동
fn gives_ownership() -> String {
    String::from("given")  // 이 값은 drop되지 않고 호출자에게 전달됨
}`,

  keyPoints: [
    "소유권은 하나: 이동 후 원래 변수는 무효 — 이중 해제 불가능",
    "스코프 끝에서 drop() 자동 호출 — GC 없이 메모리 해제",
    "i32/bool/char: Copy (자동 복사) / String/Vec: Move (소유권 이동)",
    ".clone(): 명시적 깊은 복사 — 비용이 있음을 코드에서 명확히 드러냄"
  ],
  comparisons: [
    ["header","Copy (자동)","Move (이동)","Clone (명시)"],
    ["diff","i32, bool, char, f64","String, Vec, Box","모든 타입 가능"],
    ["diff","스택 값 복사","소유권만 이동","Heap까지 깊은 복사"],
    ["diff","원본 유효","원본 무효","원본 유효"],
    ["diff","비용 없음","비용 없음","비용 있음 (Heap 할당)"]
  ]
},

// ──────────────────────────────────────────────────────
// 10. Borrowing & References
// ──────────────────────────────────────────────────────
{
  title: "Borrowing & References (빌림과 참조)",
  category: "핵심",
  explanation:
`소유권을 이전하지 않고 값을 '잠깐 빌려쓰는' 방법입니다.

  &T: 불변 참조 — 동시에 여러 개 가능, 읽기만
  &mut T: 가변 참조 — 하나만 가능, 읽기/쓰기

빌림 규칙:
  1. 불변 참조 여러 개 OR 가변 참조 하나 — 동시에 공존 불가
  2. 참조는 항상 유효해야 함 (댕글링 참조 불가)

NLL (Non-Lexical Lifetimes): 참조의 마지막 사용 이후에는
가변 참조가 허용됨 (스마트!)`,
  whyItMatters:
`'소유권을 넘기지 않고 쓰려면?' → 참조(&)를 사용합니다.
함수에 값을 넘길 때 소유권을 이동하면 돌려받을 때까지 못 씁니다.
참조로 넘기면 소유권을 유지한 채로 함수가 사용할 수 있습니다.`,
  diagram:
`  Borrowing 규칙 시각화
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

  ✗ 불변 + 가변 동시에 (에러!):
  let r1 = &s;
  let r2 = &mut s;  // 에러!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: & = 불변 참조(여러 개 OK), &mut = 가변 참조(하나만)
//    참조 동안 원본은 이동 불가 → 데이터 경쟁 원천 차단
// ══════════════════════════════════════════════════════

fn main() {
    let s1 = String::from("hello");

    // ─── & 불변 참조: 소유권 이전 없이 빌림 ───
    // &s1 → s1을 빌려서 함수에 전달, 함수가 끝나면 참조 반환
    let len = calculate_length(&s1);
    // s1은 여전히 유효! (소유권이 이동하지 않았음)
    println!("{} has {} chars", s1, len);  // ✓ 출력: hello has 5 chars

    // ─── &mut 가변 참조: 원본 수정 가능 ───
    let mut s2 = String::from("hello");
    change(&mut s2);  // &mut으로 가변 참조 전달
    println!("{}", s2);  // 출력: hello, world!

    // ─── 빌림 규칙 시연 (NLL - Non-Lexical Lifetimes) ───
    let mut s3 = String::from("test");

    // 불변 참조는 여러 개 동시에 가능
    let r1 = &s3;
    let r2 = &s3;
    println!("{}, {}", r1, r2);  // r1, r2의 마지막 사용!

    // ★ r1, r2를 더 이상 안 쓰므로 여기서 가변 참조 가능! (NLL)
    let r3 = &mut s3;
    r3.push_str("!!!");
    println!("{}", r3);  // 출력: test!!!

    // ─── 슬라이스: 문자열의 일부에 대한 참조 ───
    let s4 = String::from("hello world");
    let word = first_word(&s4);
    println!("First word: {}", word);  // 출력: hello
}

// &String: String을 빌려서 읽기만 함 (소유권 이전 없음)
fn calculate_length(s: &String) -> usize {
    s.len()
}  // ★ 참조만 반납, s1은 drop되지 않음!

// &mut String: 가변 참조로 원본 수정
fn change(s: &mut String) {
    s.push_str(", world!");  // 참조로 원본 String 수정
}

// &str: 문자열 슬라이스 (String이든 &str이든 수용)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {           // 공백 문자를 찾으면
            return &s[0..i];        // 공백 전까지의 슬라이스 반환
        }
    }
    &s[..]  // 공백이 없으면 전체 반환
}`,

  keyPoints: [
    "& : 불변 참조 (여러 개 동시 가능) / &mut: 가변 참조 (하나만 가능)",
    "참조 기간 동안 원래 소유자가 값을 못 움직임",
    "가변 참조 하나만 제한 → 컴파일 타임에 데이터 경쟁 원천 차단",
    "슬라이스(&[T], &str): 컬렉션 일부에 대한 참조"
  ],
  comparisons: [
    ["header","&T (불변 참조)","&mut T (가변 참조)"],
    ["diff","여러 개 동시 가능","한 번에 하나만"],
    ["diff","읽기만 가능","읽기/쓰기 가능"],
    ["equal","소유권 이전 없음","소유권 이전 없음"],
    ["diff","원본 수정 불가","원본 수정 가능"]
  ]
},

// ──────────────────────────────────────────────────────
// 11. Option<T>
// ──────────────────────────────────────────────────────
{
  title: "Option<T> — null을 타입으로",
  category: "핵심",
  explanation:
`Rust에는 null/undefined가 없습니다. Option<T>으로 '값이 없을 수 있음'을 타입에 표현합니다.

  Some(T): 값이 있음
  None:    값이 없음

주요 메서드:
  .unwrap(): Some이면 값, None이면 panic
  .unwrap_or(default): None이면 기본값
  .map(f): Some이면 f 적용, None이면 None
  .and_then(f): Some이면 f(T) → Option (체이닝)`,
  whyItMatters:
`Tony Hoare가 null을 발명하고 '10억 달러짜리 실수'라 불렀습니다.
Rust는 타입 시스템으로 이 문제를 해결했습니다.
API 반환값이 Option<T>이면 '없을 수 있다'는 의미가 타입에 명시됩니다.`,
  diagram:
`  Option<T>의 메모리 레이아웃
  ──────────────────────────────────

  Option<i32>:
  ┌────────┬────────────┐
  │ 태그   │ 값          │
  │ 0(None)│ (미사용)    │
  │ 1(Some)│ 42          │
  └────────┴────────────┘

  특별 최적화: Option<&T>는 0비용!
  │ 0x0    → None        │
  │ 0x1234 → Some(&T)   │
  추가 메모리 0바이트!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: null 대신 Option<T> 사용
//    None 처리 안 하면 컴파일 에러 → NullPointerException 불가!
// ══════════════════════════════════════════════════════

// Option<String>을 반환 → "값이 없을 수 있음"을 타입에 명시
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),  // 값 있음
        2 => Some(String::from("Bob")),
        _ => None,                          // 값 없음
    }
}

// and_then: Option 체이닝 (None이면 자동으로 None 전파)
fn get_email(user_id: u32) -> Option<String> {
    find_user(user_id).and_then(|name| {
        Some(format!("{}@example.com", name.to_lowercase()))
    })
}

fn main() {
    // ─── match: 모든 경우 처리 (가장 명시적) ───
    match find_user(1) {
        Some(name) => println!("Found: {}", name),  // 출력됨
        None => println!("Not found"),
    }

    // ─── if let: 한 가지 경우만 처리 ───
    if let Some(name) = find_user(2) {
        println!("Found: {}", name);  // 출력: Found: Bob
    }
    // None인 경우는 자동으로 무시됨

    // ─── unwrap_or_else: None일 때 기본값 제공 ───
    let name = find_user(99).unwrap_or_else(|| String::from("Unknown"));
    println!("User: {}", name);  // 출력: User: Unknown

    // ─── map: Some일 때만 변환 ───
    let len = find_user(1).map(|n| n.len());
    println!("Name length: {:?}", len);  // 출력: Some(5)
    // find_user(99)라면 → None.map(...) → None

    // ─── and_then: Option 체이닝 ───
    if let Some(email) = get_email(1) {
        println!("Email: {}", email);  // 출력: alice@example.com
    }

    // ─── 표준 라이브러리의 Option 활용 ───
    let v = vec![1, 2, 3];
    println!("{:?}", v.get(5));    // None (안전한 인덱스 접근)
    println!("{:?}", v.first());   // Some(1)

    let s = "hello";
    println!("{:?}", s.find('l')); // Some(2)
    println!("{:?}", s.find('z')); // None
}`,

  keyPoints: [
    "null이 없음 → Option<T>로 없음을 타입에 표현",
    ".unwrap(): 위험, None이면 panic / .unwrap_or(): 기본값으로 안전 처리",
    ".map(): Some에만 함수 적용, None은 그대로 / .and_then(): Option 체이닝",
    "if let Some(x) = opt { ... }: 한 가지 경우만 처리할 때 간결"
  ],
  comparisons: [
    ["header","null 방식 (TS/Java)","Option<T> 방식 (Rust)"],
    ["diff","null/undefined 존재","null/undefined 없음"],
    ["diff","체크 안 해도 컴파일 됨","처리 안 하면 컴파일 에러"],
    ["diff","NullPointerException 가능","불가능 (타입이 보장)"],
    ["win","","컴파일러가 None 처리 강제"]
  ]
},

// ──────────────────────────────────────────────────────
// 12. Result<T, E>
// ──────────────────────────────────────────────────────
{
  title: "Result<T, E> & 에러 처리",
  category: "핵심",
  explanation:
`Result는 '실패할 수 있음'을 타입으로 표현합니다.

  Result<T, E>:
    Ok(T):  성공, 값 포함
    Err(E): 실패, 에러 포함

try/catch가 없습니다! 에러는 반환값으로 전달됩니다.
핵심 연산자 ?: 에러가 있으면 즉시 현재 함수에서 Err를 반환합니다.`,
  whyItMatters:
`NestJS에서 throw/catch로 에러를 던지면 어떤 함수가 어떤 에러를 낼 수 있는지 타입으로 알 수 없습니다.
Result<T, E>는 반환 타입에 에러 타입도 명시됩니다.
? 연산자로 에러 전파가 간결하고, 개발자가 에러 처리를 강제받습니다.`,
  diagram:
`  ? 연산자의 마법:
  ──────────────────────────────────

  let n = parse_positive(s)?;

  // 이렇게 동작:
  match parse_positive(s) {
      Ok(val) => val,          // 성공 → 값 사용
      Err(e)  => return Err(e), // 실패 → 즉시 함수에서 Err 반환
  }`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: try/catch 대신 Result<T,E> 사용
//    ? 연산자로 에러 전파 → 코드 간결 + 에러 처리 강제
// ══════════════════════════════════════════════════════

use std::num::ParseIntError;

// Result<u32, String>: 성공하면 u32, 실패하면 String(에러 메시지)
fn parse_positive(s: &str) -> Result<u32, String> {
    // .parse() → Result<i32, ParseIntError>
    // .map_err() → Err의 타입을 변환 (ParseIntError → String)
    // ? → Err면 즉시 함수에서 Err 반환, Ok면 값 꺼내기
    let n: i32 = s.parse().map_err(|e: ParseIntError| e.to_string())?;

    if n < 0 {
        return Err(format!("{} is negative", n));  // 조기 반환
    }
    Ok(n as u32)  // 성공 시 Ok로 감싸서 반환
}

// ★ ? 연산자: 에러면 즉시 Err 반환 (보일러플레이트 제거!)
fn double_positive(s: &str) -> Result<u32, String> {
    let n = parse_positive(s)?;  // Err → 즉시 return Err(...), Ok → n에 값
    Ok(n * 2)
}

fn main() {
    // ─── match로 성공/실패 분기 ───
    match double_positive("21") {
        Ok(n) => println!("Result: {}", n),   // 출력: Result: 42
        Err(e) => println!("Error: {}", e),
    }

    // ─── 다양한 에러 케이스 ───
    println!("{:?}", double_positive("abc"));   // Err("invalid digit...")
    println!("{:?}", double_positive("-5"));    // Err("-5 is negative")

    // ─── unwrap_or_else: 에러 시 기본값 + 에러 로깅 ───
    let n = double_positive("bad").unwrap_or_else(|e| {
        println!("에러 발생: {}", e);  // 에러 로깅
        0                                // 기본값 반환
    });
    println!("n = {}", n);  // 출력: n = 0

    // ─── map: Ok일 때만 변환 ───
    let result = double_positive("5")
        .map(|n| format!("답: {}", n));
    println!("{:?}", result);  // Some("답: 10")

    // ─── 표준 라이브러리의 Result ───
    let parsed: Result<i32, _> = "42".parse();
    println!("{:?}", parsed);  // Ok(42)

    let bad: Result<i32, _> = "abc".parse();
    println!("{:?}", bad);     // Err(ParseIntError)
}`,

  keyPoints: [
    "? 연산자: 에러면 즉시 현재 함수에서 Err 반환 (가장 자주 쓰는 패턴)",
    "map_err(): Err의 타입을 변환 / map(): Ok의 값을 변환",
    "에러 타입 E는 명시적 — 어떤 에러가 나올지 타입으로 문서화됨",
    "unwrap()은 프로토타입 코드에서만, 실제로는 match/?/unwrap_or 사용"
  ],
  comparisons: [
    ["header","Rust Result","Java/TS try-catch"],
    ["diff","반환값으로 에러 전달","예외 던지기"],
    ["diff","타입으로 에러 명시","런타임에만 알 수 있음"],
    ["win","? 연산자로 간결 전파","try-catch 중첩"],
    ["win","에러 처리 강제","처리 누락 가능"],
    ["diff","map_err로 에러 변환","catch에서 변환"]
  ]
},

// ──────────────────────────────────────────────────────
// 13. Traits
// ──────────────────────────────────────────────────────
{
  title: "Traits (트레이트) — 인터페이스이자 계약",
  category: "추상화",
  explanation:
`Trait는 '이 타입이 할 수 있는 것'을 정의하는 계약입니다.
기본 구현(default implementation) 제공 가능
표준 라이브러리 트레이트(Display, Debug, Iterator...)를 구현하면 언어 기능 활용 가능

impl Trait: 정적 디스패치 (컴파일 타임, 빠름)
dyn Trait: 동적 디스패치 (런타임, 유연함)`,
  whyItMatters:
`상속 없이 다형성을 달성하는 Rust의 핵심 메커니즘입니다.
표준 라이브러리 트레이트를 구현하면 +연산자, {}출력, 비교 등을 쓸 수 있습니다.`,
  diagram:
`  Trait 시스템 구조도
  ──────────────────────────────────

  trait Animal {               ┌──────────────┐
      fn name(&self) -> &str;  │   Animal     │
      fn sound(&self) -> &str; │  (트레이트)   │
      fn describe(&self) { ... }└──┬─────┬────┘
  }                                │     │
                              impl Animal  impl Animal
                                   │     │
                              ┌───────┴──┐ ┌┴────────┐
                              │   Dog    │ │   Cat   │
                              │ sound:멍 │ │sound:야옹│
                              └──────────┘ └─────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: trait = 인터페이스 + 기본 구현 가능
//    impl Trait = 정적(빠름), &dyn Trait = 동적(유연)
// ══════════════════════════════════════════════════════

use std::fmt;

// 트레이트 정의: 메서드 시그니처 + 기본 구현
trait Animal {
    fn name(&self) -> &str;     // 반드시 구현해야 함 (추상 메서드)
    fn sound(&self) -> &str;    // 반드시 구현해야 함

    // ★ 기본 구현 (override 가능) — JS의 default method와 비슷
    fn describe(&self) -> String {
        format!("{}: {} 소리를 낸다", self.name(), self.sound())
    }
}

struct Dog { name: String }
struct Cat { name: String }

// ─── impl Animal for Dog: 트레이트 구현 ───
impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "멍" }
    // describe()는 기본 구현 사용
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "야옹" }
    // ★ 기본 구현을 override (덮어쓰기)
    fn describe(&self) -> String {
        format!("고양이 {} (도도함)", self.name())
    }
}

// ─── 표준 Display 트레이트 구현 → println!("{}") 사용 가능 ───
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

// ─── impl Trait: 정적 디스패치 (컴파일 타임에 타입 결정) ───
fn make_sound(animal: &impl Animal) {
    println!("{}", animal.describe());
}

// ─── &dyn Trait: 동적 디스패치 (런타임에 타입 결정) ───
// 여러 타입을 하나의 컬렉션에 담을 때 필요
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

    make_sound(&dog);   // 정적 디스패치
    make_sound(&cat);
    println!("{}", dog);  // Display 트레이트 덕분에 출력 가능!

    // ★ 서로 다른 타입을 하나의 Vec에 담으려면 &dyn 필요
    let animals: Vec<&dyn Animal> = vec![&dog, &cat, &parrot];
    make_all_sounds(&animals);
}`,

  keyPoints: [
    "trait: 메서드 시그니처 정의 + 기본 구현 제공 가능",
    "impl Trait: 정적 디스패치 (컴파일 타임 해결, 빠름)",
    "&dyn Trait: 동적 디스패치 (런타임 해결, 유연함)",
    "표준 트레이트(Display, Iterator, From 등) 구현으로 언어 기능 활용"
  ],
  comparisons: [
    ["header","impl Trait (정적)","&dyn Trait (동적)"],
    ["diff","컴파일 타임 결정","런타임에 결정"],
    ["diff","단형화 — 타입별 코드 생성","vtable 간접 호출"],
    ["win","오버헤드 없음","유연한 컬렉션"],
    ["diff","fn(x: &impl Trait)","fn(x: &dyn Trait)"]
  ]
},

// ──────────────────────────────────────────────────────
// 14. Generics
// ──────────────────────────────────────────────────────
{
  title: "Generics (제네릭) — 타입 매개변수",
  category: "추상화",
  explanation:
`같은 로직을 다양한 타입에 재사용할 때 제네릭을 씁니다.
컴파일 타임에 구체적인 타입으로 '단형화(monomorphization)'되어
런타임 오버헤드가 없습니다.

T: PartialOrd 처럼 트레이트 바운드로 제약을 줍니다.
where 구문으로 복잡한 바운드를 정리할 수 있습니다.`,
  whyItMatters:
`TypeScript의 제네릭과 개념은 같지만, 런타임 동작이 다릅니다.
TypeScript: 타입 정보가 런타임에 사라짐 (type erasure)
Rust: 컴파일 타임에 구체 타입으로 확정 (단형화) → 런타임 비용 없음`,
  diagram:
`  단형화 (Monomorphization) 시각화
  ──────────────────────────────────

  fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

  컴파일 타임에 이렇게 확장:

  largest(&[34, 50, 25, 100])
       ↓
  fn largest_i32(list: &[i32]) -> &i32 { ... }

  largest(&['y', 'm', 'a', 'q'])
       ↓
  fn largest_char(list: &[char]) -> &char { ... }

  런타임: 오버헤드 0!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 제네릭은 컴파일 타임에 구체 타입으로 확장
//    → 런타임 오버헤드 0 (TS와 다름!)
// ══════════════════════════════════════════════════════

// T: PartialOrd → "T는 비교 가능해야 한다"는 제약 (트레이트 바운드)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {    // PartialOrd가 있어야 > 연산 가능
            largest = item;
        }
    }
    largest
}

// ─── 제네릭 구조체 ───
#[derive(Debug)]
struct Wrapper<T> {    // T: 임의의 타입
    value: T,
}

// ★ impl<T: Display> → Display를 구현한 T만 이 메서드 사용 가능
impl<T: std::fmt::Display> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }

    fn show(&self) {
        println!("Wrapped: {}", self.value);  // {} 출력은 Display 필요
    }
}

// ─── where 구문: 복잡한 바운드를 깔끔하게 정리 ───
// 함수 시그니처가 길어질 때 유용
fn complex_fn<T, U>(t: T, u: U) -> String
where
    T: std::fmt::Display + Clone,  // T는 출력 가능 + 복제 가능
    U: std::fmt::Debug,             // U는 디버그 출력 가능
{
    format!("{:?} + {}", u, t.clone())  // {:?} = Debug, {} = Display
}

// ─── 제네릭 Enum (Option, Result가 이렇게 구현됨) ───
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    // i32 배열에서 최댓값
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));  // 100

    // char 배열에서 최댓값 (같은 함수 재사용!)
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));  // y

    // Wrapper<i32>와 Wrapper<&str> — 같은 구조체, 다른 타입
    let w1 = Wrapper::new(42);       // Wrapper<i32>
    let w2 = Wrapper::new("hello");  // Wrapper<&str>
    w1.show();  // 출력: Wrapped: 42
    w2.show();  // 출력: Wrapped: hello

    // where 구문 함수 사용
    println!("{}", complex_fn("Rust", vec![1, 2, 3]));
    // 출력: [1, 2, 3] + Rust
}`,

  keyPoints: [
    "단형화: 컴파일 타임에 구체 타입으로 확정 → 런타임 오버헤드 없음",
    "트레이트 바운드 T: Trait → '이 타입은 이 능력이 있어야 한다'",
    "where 구문: 복잡한 바운드를 함수 시그니처 밖으로 분리해서 가독성 향상",
    "Vec<T>, Option<T>, Result<T,E> 모두 제네릭으로 구현됨"
  ],
  comparisons: [
    ["header","Rust 제네릭","TypeScript 제네릭"],
    ["diff","단형화 (타입별 코드 생성)","type erasure (런타임에 삭제)"],
    ["diff","런타임 오버헤드 0","런타임에 타입 정보 없음"],
    ["diff","트레이트 바운드로 제약","extends / keyof 제약"],
    ["win","컴파일 타임 완전 체크","런타임 에러 가능"]
  ]
},

// ──────────────────────────────────────────────────────
// 15. Closures & Iterators
// ──────────────────────────────────────────────────────
{
  title: "Closures & Iterators — 함수형 패턴",
  category: "추상화",
  explanation:
`클로저: 주변 환경(변수)을 캡처하는 익명 함수
  |x| x + 1     // 인자 하나, 타입 추론
  move |x| x + n // n의 소유권을 캡처

Fn 트레이트 3종:
  Fn: 불변 참조 / FnMut: 가변 참조 / FnOnce: 소유권

Iterator: 지연 계산(lazy) 체인
  map, filter, fold, take, skip, flatten, zip, enumerate...`,
  whyItMatters:
`NestJS의 Array 메서드(map, filter, reduce)와 비슷하지만 성능이 다릅니다.
Rust Iterator는 lazy: 실제로 소비(collect/for_each)되기 전까지 계산하지 않습니다.`,
  diagram:
`  Iterator 체인: Lazy 평가
  ──────────────────────────────────

  data.iter()
      .filter(...)     ← 아직 안 함
      .map(...)        ← 아직 안 함
      .take(3)         ← 아직 안 함
      .collect()       ← 여기서 실행!

  [1,2,3,4,5,6,7,8,9,10]
       │
    filter(|x| x%2==0)    [2,4,6,8,10]
       │
    map(|x| x*x)          [4,16,36,64,100]
       │
    take(3)               [4,16,36]
       │
    collect()             → Vec<i32>

  실제로는 중간 Vec 없이 한 번에 처리!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 클로저 = 환경 캡처 함수, Iterator = lazy 체인
//    중간 컬렉션 생성 없이 효율적 처리
// ══════════════════════════════════════════════════════

fn main() {
    // ─── 클로저 기본: |매개변수| { 본문 } ───
    // 타입 명시 생략 가능 (컴파일러가 추론)
    let add = |x: i32, y: i32| x + y;
    println!("{}", add(3, 4));  // 출력: 7

    // ─── 환경 캡처: 주변 변수를 가져다 씀 ───
    let n = 5;
    let add_n = |x| x + n;  // ★ n을 캡처! (Fn: 불변 참조로 빌림)
    println!("{}", add_n(3));  // 출력: 8
    println!("{}", n);          // ✓ n도 여전히 유효 (빌렸을 뿐)

    // ─── FnMut: 가변 참조로 캡처 ───
    let mut count = 0;
    let mut inc = || { count += 1; count };  // count를 &mut로 캡처
    println!("{}", inc());  // 출력: 1
    println!("{}", inc());  // 출력: 2

    // ─── move 클로저: 소유권 이전 (스레드에서 필수!) ───
    let text = String::from("hello");
    let show = move || println!("{}", text);  // ★ text의 소유권을 클로저로 이동
    show();
    // println!("{}", text);  // 에러! text는 이미 클로저로 이동됨

    // ─── Iterator 체인: lazy! collect() 전까지 실행 안 함 ───
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = data.iter()
        .filter(|&&x| x % 2 == 0)  // 짝수만 걸러내기 (lazy)
        .map(|&x| x * x)           // 제곱 (lazy)
        .take(3)                    // 앞의 3개만 (lazy)
        .collect();                 // ★ 여기서 실제 실행!
    println!("{:?}", result);       // 출력: [4, 16, 36]

    // ─── fold: 누산 (JS의 reduce와 같음) ───
    let sum: i32 = data.iter().fold(
        0,              // 초기값
        |acc, &x| acc + x  // acc: 누적값, x: 현재값
    );
    println!("Sum: {}", sum);  // 출력: Sum: 55

    // ─── flat_map: 중첩 해제 ───
    let words = vec!["hello world", "foo bar"];
    let split: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())  // 각 문자열을 단어로 분리 + 펼치기
        .collect();
    println!("{:?}", split);  // 출력: ["hello", "world", "foo", "bar"]

    // ─── 커스텀 Iterator 구현 ───
    struct Counter { count: u32 }
    impl Iterator for Counter {
        type Item = u32;  // 연관 타입 (반환할 요소의 타입)
        fn next(&mut self) -> Option<u32> {
            self.count += 1;
            if self.count <= 5 { Some(self.count) } else { None }
        }
    }
    // Iterator를 구현하면 for, sum, map 등 모두 사용 가능!
    let total: u32 = Counter { count: 0 }.sum();
    println!("Counter sum: {}", total);  // 출력: 15 (1+2+3+4+5)
}`,

  keyPoints: [
    "Fn/FnMut/FnOnce: 캡처 방식에 따른 분류 (컴파일러가 자동 선택)",
    "move |...| { ... }: 소유권 이전 — 스레드/비동기 코드에서 필수",
    "Iterator는 lazy: collect()/for_each()까지 실제 계산 안 함",
    "체인 (filter→map→take): 중간 Vec 생성 없이 효율적으로 처리"
  ],
  comparisons: [
    ["header","Fn","FnMut","FnOnce"],
    ["diff","불변 참조 캡처","가변 참조 캡처","소유권 캡처"],
    ["diff","여러 번 호출 가능","여러 번 호출 가능","한 번만 호출"],
    ["diff","&T로 환경 접근","&mut T로 환경 접근","T 소비"]
  ]
},

// ──────────────────────────────────────────────────────
// 16. Lifetimes
// ──────────────────────────────────────────────────────
{
  title: "Lifetime ('a) — 참조의 유효 기간",
  category: "추상화",
  explanation:
`라이프타임은 참조가 '언제까지 유효한지'를 컴파일러에게 알려주는 주석입니다.
대부분 컴파일러가 자동 추론(lifetime elision)합니다.

명시적으로 써야 하는 경우:
  1. 함수가 참조를 받고 참조를 반환할 때
  2. 구조체가 참조를 필드로 가질 때

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
'x와 y 중 짧은 수명만큼 반환값이 살아있다'는 의미`,
  whyItMatters:
`댕글링 참조를 컴파일 타임에 막습니다.
다른 언어에서 null pointer exception이 나는 상황을 컴파일러가 잡습니다.
처음에는 가장 어렵게 느껴지지만, 명시해야 하는 경우는 많지 않습니다.`,
  diagram:
`  라이프타임 타임라인
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
           반환값은 'a 안에서만 유효`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 라이프타임은 컴파일러를 위한 주석 (런타임 비용 0)
//    대부분 자동 추론, 여러 참조가 얽히면 명시 필요
// ══════════════════════════════════════════════════════

// ★ 'a: x와 y 중 짧은 수명을 반환값의 수명으로 제한
// "반환값은 x나 y보다 오래 살지 않는다"는 보증
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ─── 구조체에 참조 필드가 있으면 라이프타임 명시 필수 ───
// "이 구조체는 content 참조가 유효한 동안만 사용 가능하다"
struct Important<'a> {
    content: &'a str,  // ★ 참조를 저장 → 얼마나 오래 유효한지 명시 필요
}

impl<'a> Important<'a> {
    fn announce(&self) -> &str {
        self.content  // &self의 라이프타임으로 자동 추론 (elision)
    }
}

fn main() {
    // ─── 라이프타임이 안전한 사용 예 ───
    let s1 = String::from("long string");  // s1: 긴 수명
    let result;
    {
        let s2 = String::from("short");    // s2: 짧은 수명 (이 블록 안에서만)
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);   // ✓ OK: 같은 스코프 안이므로 안전
    }
    // println!("{}", result);  // ✗ 에러! s2가 이미 drop되어 result가 무효
    // 컴파일러가 이것을 감지함! → 댕글링 참조 불가

    // ─── 'static: 프로그램 전체 수명 ───
    // 문자열 리터럴은 바이너리에 저장되어 프로그램이 끝날 때까지 유효
    let lit: &'static str = "I have a static lifetime";

    // ─── 구조체 라이프타임 실전 예제 ───
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];  // novel의 일부를 참조
    }
    // first_sentence는 novel이 살아있는 동안 유효
    let important = Important { content: first_sentence };
    println!("{}", important.announce());  // 출력: Call me Ishmael
}`,

  keyPoints: [
    "라이프타임은 컴파일러를 위한 주석 — 런타임 비용 없음",
    "대부분 자동 추론, 여러 참조가 얽히면 명시 필요",
    "'a: 두 참조 중 겹치는 수명(짧은 쪽)으로 제한",
    "'static: 프로그램 전체 수명 (문자열 리터럴이 대표적)"
  ],
  comparisons: [
    ["header","명시 필요","자동 추론 (Elision)"],
    ["diff","여러 참조 입력+참조 반환","입력 참조 하나"],
    ["diff","구조체에 참조 필드","함수 파라미터만"],
    ["diff","impl<'a> Type<'a>","&self 메서드"]
  ]
},

// ──────────────────────────────────────────────────────
// 17. Smart Pointers
// ──────────────────────────────────────────────────────
{
  title: "Smart Pointers (Box, Rc, Arc)",
  category: "동시성",
  explanation:
`Box<T>: Heap 할당, 단일 소유권
Rc<T>: Reference Counting, 단일 스레드 공유 소유권
Arc<T>: Atomic RC, 멀티스레드 안전
RefCell<T>: 런타임에 빌림 규칙 검사 (내부 가변성)`,
  whyItMatters:
`'여러 곳에서 같은 데이터를 소유하고 싶다' → Rc<T> 또는 Arc<T>
Rc<RefCell<T>>: 단일 스레드 공유 + 가변 (그래프 구조)
Arc<Mutex<T>>: 멀티스레드 공유 + 가변`,
  diagram:
`  스마트 포인터 메모리 구조
  ──────────────────────────────────

  Box<T>:
  스택              Heap
  ┌───────┐        ┌─────┐
  │ Box   │        │  5  │
  │ ptr───┼───────►│     │
  └───────┘        └─────┘
  단일 소유권

  Rc<T>:
  ┌────┐ ┌────┐ ┌────┐    ┌───────┐
  │ a  │ │ b  │ │ c  │    │ value │
  │ptr─┼─┤ptr─┼─┤ptr─┼───►│  5    │
  └────┘ └────┘ └────┘    │count:3│
                          └───────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Box = Heap 단일 소유, Rc = 공유 소유, Arc = 스레드 안전
//    RefCell = 런타임 빌림 검사 (내부 가변성)
// ══════════════════════════════════════════════════════

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    // ─── Box<T>: Heap에 데이터 할당 ───
    // 재귀 타입(트리, 리스트) 정의 시 크기를 컴파일 타임에 알기 위해 사용
    let b = Box::new(5);
    println!("Box: {}", *b);  // *로 역참조. 출력: Box: 5

    // ─── Box<dyn Trait>: 동적 디스패치 ───
    // 컴파일 타임에 타입을 모르지만 같은 트레이트를 구현하는 객체들을 모을 때
    trait Speak { fn speak(&self); }
    struct Dog;
    struct Cat;
    impl Speak for Dog { fn speak(&self) { println!("멍!"); } }
    impl Speak for Cat { fn speak(&self) { println!("야옹!"); } }

    // ★ Box<dyn Speak>: Dog든 Cat든 담을 수 있음
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &animals { a.speak(); }

    // ─── Rc<T>: 단일 스레드 공유 소유권 ───
    // 여러 곳에서 같은 데이터를 읽을 때
    let a = Rc::new(5);          // 참조 카운트 = 1
    let b = Rc::clone(&a);       // ★ clone은 복사가 아님! 카운트만 증가
    let c = Rc::clone(&a);       // 카운트 = 3
    println!("Count: {}", Rc::strong_count(&a));  // 출력: 3
    drop(b);                      // 카운트 = 2 (데이터는 유지됨)
    println!("After drop: {}", Rc::strong_count(&a));  // 출력: 2

    // ─── Rc<RefCell<T>>: 공유 + 가변 (단일 스레드) ───
    // RefCell: 컴파일 타임 대신 런타임에 빌림 규칙 검사
    let shared = Rc::new(RefCell::new(0));  // 가변 공유 데이터
    let clone1 = Rc::clone(&shared);        // 같은 데이터를 가리키는 또 다른 소유자
    let clone2 = Rc::clone(&shared);

    *clone1.borrow_mut() += 10;  // ★ borrow_mut()로 가변 접근
    *clone2.borrow_mut() += 20;  // 다른 소유자에서도 수정 가능!
    println!("Shared: {}", shared.borrow());  // 출력: Shared: 30

    // ─── Arc<T>: 멀티스레드 안전 공유 소유권 ───
    // Rc의 스레드 안전 버전 (원자적 참조 카운트)
    let counter = Arc::new(0);
    let counter_clone = Arc::clone(&counter);
    println!("Arc: {}", counter_clone);  // 출력: Arc: 0
}`,

  keyPoints: [
    "Box<T>: Heap 할당, 재귀 타입, Box<dyn Trait> 동적 디스패치",
    "Rc<T>: 단일 스레드 공유 소유권 / Arc<T>: 멀티스레드",
    "RefCell<T>: 컴파일 타임 대신 런타임에 빌림 검사 (내부 가변성)",
    "Rc<RefCell<T>>: 단스레드 공유+가변 / Arc<Mutex<T>>: 멀티스레드"
  ],
  comparisons: [
    ["header","Box<T>","Rc<T>","Arc<T>"],
    ["diff","단일 소유권","공유 소유권","공유 소유권"],
    ["diff","스레드 안전","단일 스레드만","멀티스레드 OK"],
    ["equal","Heap 할당","Heap 할당","Heap 할당"],
    ["diff","카운트 없음","참조 카운트","원자적 카운트"]
  ]
},

// ──────────────────────────────────────────────────────
// 18. Threads & Mutex
// ──────────────────────────────────────────────────────
{
  title: "Threads & Mutex (멀티스레딩)",
  category: "동시성",
  explanation:
`Rust 멀티스레딩은 '두려움 없는 동시성(fearless concurrency)'을 지향합니다.
Send, Sync 트레이트로 스레드 안전성을 컴파일 타임에 보장합니다.

thread::spawn: 새 스레드 생성 (move 클로저로 소유권 이전)
Mutex<T>: 한 번에 하나의 스레드만 접근 가능
Arc<Mutex<T>>: 멀티스레드 공유 가변 상태의 표준 패턴`,
  whyItMatters:
`Java/Node.js에서 공유 상태 버그(race condition)가 런타임에 발생합니다.
Rust는 Send/Sync 체크로 컴파일 타임에 잡습니다.
NestJS는 이벤트 루프 단일 스레드이지만, Rust는 진정한 병렬 실행이 가능합니다.`,
  diagram:
`  Arc<Mutex<T>> 패턴 시각화
  ──────────────────────────────────

  Thread 1        Thread 2        Thread 3
  ┌──────┐       ┌──────┐       ┌──────┐
  │ clone│       │ clone│       │ clone│
  │  Arc │       │  Arc │       │  Arc │
  └──┬───┘       └──┬───┘       └──┬───┘
     │              │              │
     └──────────────┼──────────────┘
                    │
             ┌──────▼──────┐
             │   Arc<T>    │
             │  ┌────────┐ │
             │  │ Mutex  │ │
             │  │ ┌────┐ │ │
             │  │ │ 0  │ │ │
             │  │ └────┘ │ │
             │  └────────┘ │
             └─────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Arc<Mutex<T>> = 멀티스레드 공유 가변 상태
//    move 클로저로 스레드에 소유권 이전
//    Mutex::lock() → 한 번에 하나의 스레드만 접근
// ══════════════════════════════════════════════════════

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // ─── 기본 스레드 생성 ───
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("스레드: {}", i);
            thread::sleep(Duration::from_millis(10));  // 10ms 대기
        }
    });
    // 메인 스레드도 동시에 실행됨 (병렬!)
    for i in 1..=3 {
        println!("메인: {}", i);
        thread::sleep(Duration::from_millis(10));
    }
    handle.join().unwrap();  // ★ 스레드 종료까지 대기 (안 하면 메인이 먼저 끝날 수 있음)

    // ─── move 클로저: 변수 소유권을 스레드로 이전 ───
    let data = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {  // ★ move: data의 소유권을 클로저로 이동
        println!("데이터: {:?}", data);    // 스레드 안에서 data 사용 가능
    });
    // println!("{:?}", data);  // 에러! data는 이미 스레드로 이동됨
    handle2.join().unwrap();

    // ─── ★ Arc<Mutex<T>>: 여러 스레드에서 공유 카운터 ───
    // Arc: 원자적 참조 카운트 → 여러 스레드에서 소유권 공유
    // Mutex: 상호 배제 → 한 번에 하나의 스레드만 접근
    let counter = Arc::new(Mutex::new(0));  // 공유 카운터 (초기값 0)
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);  // ★ clone은 복사가 아님! 참조 카운트만 증가
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // ★ 락 획득 (다른 스레드는 대기)
            *num += 1;                               // 안전하게 값 수정
        });  // num이 drop되면서 락 자동 해제!
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();  // 모든 스레드 종료 대기
    }

    // 모든 스레드가 +1 했으므로 결과는 항상 5 (race condition 불가!)
    println!("최종 카운터: {}", *counter.lock().unwrap());  // 출력: 5
}`,

  keyPoints: [
    "thread::spawn + move 클로저: 소유권을 스레드로 이전",
    "Arc<Mutex<T>>: 멀티스레드 공유 가변 상태의 표준 패턴",
    "Mutex::lock(): 잠금 획득, MutexGuard drop 시 자동 해제",
    "Send/Sync: 컴파일 타임에 스레드 안전성 보장"
  ],
  comparisons: [
    ["header","Rust","Node.js"],
    ["diff","진정한 병렬 (OS 스레드)","이벤트 루프 (단일 스레드)"],
    ["diff","Arc<Mutex<T>>로 공유","공유 상태 없음"],
    ["win","컴파일 타임 race condition 방지","런타임才 감지"],
    ["diff","move로 소유권 명확","비동기 콜백/async-await"]
  ]
},

// ──────────────────────────────────────────────────────
// 19. Channels
// ──────────────────────────────────────────────────────
{
  title: "Channels — 메시지 패싱으로 통신",
  category: "동시성",
  explanation:
`'메모리를 공유해서 통신하지 말고, 통신해서 메모리를 공유하라'

mpsc: Multiple Producer, Single Consumer
tx.send(val): val의 소유권을 채널로 이전
rx.recv(): 블로킹 수신
tx는 clone() 가능 → 여러 스레드에서 전송 가능`,
  whyItMatters:
`Arc<Mutex<T>>보다 간단한 경우가 많습니다.
Producer-Consumer 패턴, 작업 큐, 결과 수집에 유용합니다.`,
  diagram:
`  Channel 구조 시각화
  ──────────────────────────────────

  Producer(tx)                       Consumer(rx)
  ┌──────────┐                      ┌──────────┐
  │ tx       │    채널 (FIFO 큐)     │ rx       │
  │ send(val)├────►│ val1 │ val2 │──►│ recv()   │
  └──────────┘     └──────┴──────┘   └──────────┘
  소유권 이동 →    순서 보장          블로킹 대기`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: mpsc = 다중 생산자, 단일 소비자
//    send(val) → val의 소유권이 이동 → 데이터 경쟁 불가!
// ══════════════════════════════════════════════════════

use std::sync::mpsc;  // multi-producer, single-consumer
use std::thread;
use std::time::Duration;

fn main() {
    // ─── 기본 채널 통신 ───
    let (tx, rx) = mpsc::channel();  // tx: 송신자, rx: 수신자

    thread::spawn(move || {           // move로 tx의 소유권을 스레드로 이전
        let val = String::from("hello");
        tx.send(val).unwrap();  // ★ val의 소유권이 채널로 이동! send 후 val 사용 불가
    });

    let received = rx.recv().unwrap();  // ★ 블로킹: 값이 올 때까지 대기
    println!("Got: {}", received);      // 출력: Got: hello

    // ─── 여러 값 전송 + 반복자로 수신 ───
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "thread"];
        for val in vals {
            tx2.send(val).unwrap();                   // 하나씩 전송
            thread::sleep(Duration::from_millis(50)); // 50ms 간격
        }
    });  // tx2가 drop됨 → 채널이 닫힘

    // ★ rx를 반복자처럼 사용 → 채널이 닫힐 때까지 반복
    for received in rx2 {
        println!("Received: {}", received);
    }

    // ─── 여러 Producer: tx를 clone() ───
    let (tx3, rx3) = mpsc::channel();
    let tx3_clone = tx3.clone();  // ★ clone으로 여러 생산자 생성

    thread::spawn(move || {
        tx3.send(String::from("Producer 1")).unwrap();
    });
    thread::spawn(move || {
        tx3_clone.send(String::from("Producer 2")).unwrap();
    });

    // 수신은 순서가 보장되지 않음 (Producer 1이 먼저 올 수도, 2가 먼저 올 수도)
    for _ in 0..2 {
        println!("{}", rx3.recv().unwrap());
    }
}`,

  keyPoints: [
    "mpsc: 다중 생산자(tx.clone()), 단일 소비자(rx)",
    "send(val): 소유권 이전 → 전송 후 원본 접근 불가 = 데이터 경쟁 없음",
    "rx를 for in으로 사용: 모든 tx가 drop될 때까지 반복",
    "Mutex 공유보다 채널이 더 안전하고 구조가 명확한 경우가 많음"
  ],
  comparisons: [
    ["header","Channel","Arc<Mutex<T>>"],
    ["diff","메시지 패싱","공유 메모리"],
    ["diff","소유권 이전","공유 참조"],
    ["left","Producer-Consumer 패턴","공유 상태 패턴"],
    ["win","데드락 위험 적음","데드락 주의 필요"]
  ]
},

// ──────────────────────────────────────────────────────
// 20. Module System
// ──────────────────────────────────────────────────────
{
  title: "Module System (모듈, use, pub)",
  category: "실전",
  explanation:
`mod: 모듈 선언 / pub: 공개 가시성 / use: 경로 단축 (import)
crate: 현재 크레이트 루트 / super: 부모 모듈 / self: 현재 모듈

모든 것은 기본적으로 private입니다. pub을 붙여야 외부에서 접근 가능합니다.`,
  whyItMatters:
`NestJS에서 폴더/파일로 코드를 조직화하는 것과 같습니다.
하지만 Rust는 가시성(visibility)을 명시적으로 제어합니다.
라이브러리 설계 시 pub으로 공개 API를, 나머지는 private으로 유지합니다.`,
  diagram:
`  모듈 트리 구조
  ──────────────────────────────────

  crate (src/lib.rs 또는 src/main.rs)
  ├── network (src/network.rs)
  │   ├── server (src/server.rs)
  │   │   └── connect()  [pub]
  │   └── client (src/client.rs)
  │       └── connect()  [pub]
  └── utils (src/utils.rs)
      └── helper()       [pub]`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 기본 private, pub으로 공개, use로 경로 단축
//    파일 시스템 = 모듈 트리
// ══════════════════════════════════════════════════════

// ─── mod: 모듈 선언 (중첩 가능) ───
mod front_of_house {
    pub mod hosting {  // pub: 외부에서 접근 가능
        pub fn add_to_waitlist() {    // pub 함수
            println!("Waitlist에 추가");
        }
        pub fn seat_at_table() {
            println!("자리 안내");
        }
    }

    mod serving {  // pub 없음 → 외부에서 접근 불가!
        pub fn take_order() {
            println!("주문 받기");
        }
        fn serve_order() {  // private 함수
            println!("서빙");
        }
    }
}

// ─── use: 경로 단축 (import와 비슷) ───
use front_of_house::hosting;  // 이제 hosting::함수명() 으로 호출 가능

// ─── 구조체의 필드 가시성 ───
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,       // pub: 외부에서 읽기/쓰기 가능
        seasonal_fruit: String,  // ★ private: 외부에서 접근 불가!
    }

    impl Breakfast {
        // pub 생성자 → seasonal_fruit을 간접적으로만 설정 가능
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),  // 생성자 안에서는 private 필드 접근 OK
            }
        }
    }
}

fn main() {
    // ─── 전체 경로로 접근 ───
    front_of_house::hosting::add_to_waitlist();

    // ─── use로 단축한 경로 ───
    hosting::seat_at_table();  // front_of_house::hosting:: 생략!

    // ─── pub 구조체: pub 필드만 접근 가능 ───
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");       // ✓ pub 필드 → 접근 OK
    println!("Toast: {}", meal.toast);
    // meal.seasonal_fruit;  // ✗ 에러! private 필드

    // ─── use as: 별칭 지정 ───
    use front_of_house::hosting as host;  // hosting 대신 host로 사용
    host::add_to_waitlist();
}`,

  keyPoints: [
    "mod: 모듈 선언 / pub: 공개 가시성 / use: 경로 단축",
    "기본적으로 모든 것은 private — pub 필요",
    "crate:: 최상위 / super:: 부모 / self:: 현재",
    "파일 시스템이 곧 모듈 트리 (src/network.rs = mod network)"
  ],
  comparisons: [
    ["header","Rust 모듈","NestJS/TS 모듈"],
    ["diff","mod + pub 선언","import/export"],
    ["diff","기본 private","기본 접근 가능"],
    ["left","crate:: 경로","상대/절대 경로"],
    ["diff","컴파일 타임 검사","런타임에만 에러"]
  ]
},

// ──────────────────────────────────────────────────────
// 21. Testing
// ──────────────────────────────────────────────────────
{
  title: "Testing (테스트) — 안전한 리팩토링",
  category: "실전",
  explanation:
`Rust는 테스트를 언어 차원에서 지원합니다. 외부 프레임워크 불필요!

  #[test]: 테스트 함수 표시
  assert!, assert_eq!, assert_ne!: 단언 매크로
  #[should_panic]: 패닉이 발생해야 성공
  #[cfg(test)]: 배포 빌드에 포함되지 않음

실행: cargo test`,
  whyItMatters:
`Jest/Mocha 같은 외부 도구 없이 내장 테스트 프레임워크를 사용합니다.
cargo test 한 명령으로 모든 테스트를 병렬 실행합니다.`,
  diagram:
`  단위 테스트 구조:
  ──────────────────────────────────

  #[cfg(test)]        ← 배포 빌드에 제외
  mod tests {
      use super::*;   ← 부모 모듈 전체 import

      #[test]         ← 테스트 함수 표시
      fn test_add() {
          assert_eq!(add(2, 3), 5);
      }
  }`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: #[test]로 테스트 함수 표시, cargo test로 실행
//    #[cfg(test)]로 배포 빌드에서 테스트 코드 제외
// ══════════════════════════════════════════════════════

// ─── 테스트 대상 함수들 ───
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ─── 단위 테스트 모듈 ───
// ★ #[cfg(test)]: 이 모듈은 cargo test 할 때만 컴파일됨 (배포 빌드에서 제외)
#[cfg(test)]
mod tests {
    use super::*;  // 부모 모듈의 모든 것을 import

    #[test]  // ★ 이 속성이 있어야 cargo test가 인식함
    fn test_add() {
        // assert_eq!: 두 값이 같은지 비교
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_divide_some() {
        assert_eq!(divide(10, 2), Some(5));  // 성공 케이스
    }

    #[test]
    fn test_divide_none() {
        assert_eq!(divide(10, 0), None);  // 0으로 나누면 None
    }

    #[test]
    fn test_greet() {
        let result = greet("Rust");
        // assert!: 조건이 참인지 확인
        assert!(result.contains("Rust"));
        assert!(result.starts_with("Hello"));
    }

    // ─── Result를 반환하는 테스트: ? 연산자 사용 가능 ───
    #[test]
    fn test_result() -> Result<(), String> {
        let val = divide(10, 2)
            .ok_or("division failed")?;  // None이면 테스트 실패
        assert_eq!(val, 5);
        Ok(())
    }
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 2) = {:?}", divide(10, 2));
    println!("{}", greet("Rust"));
    println!("\\ncargo test 로 테스트를 실행하세요!");
}`,

  keyPoints: [
    "#[test]: 테스트 함수 표시 / cargo test로 실행",
    "assert_eq! == 비교 / assert_ne! != 비교 / assert! 참 거짓",
    "#[should_panic]: 패닉 발생을 기대하는 테스트",
    "#[cfg(test)]: 배포 빌드에서 테스트 코드 제외"
  ],
  comparisons: [
    ["header","Rust 내장 테스트","Jest (Node.js)"],
    ["diff","#[test] 어노테이션","describe/it 함수"],
    ["diff","assert_eq! 매크로","expect().toBe()"],
    ["left","cargo test로 실행","npm test로 실행"],
    ["win","외부 의존성 없음","jest 설치 필요"],
    ["win","병렬 실행 기본","설정 필요"]
  ]
},

// ──────────────────────────────────────────────────────
// 22. Error Handling 실전 패턴
// ──────────────────────────────────────────────────────
{
  title: "Error Handling 실전 패턴",
  category: "실전",
  explanation:
`실전 에러 처리 패턴:
  1. thiserror: 커스텀 에러 타입 정의 (라이브러리용)
  2. anyhow: 간편한 에러 처리 (앱 코드용)
  3. From 트레이트: ? 연산자로 자동 에러 변환
  4. match로 에러 복구`,
  whyItMatters:
`NestJS에서 에러 처리는 try-catch + 커스텀 Exception이 일반적입니다.
Rust에서는 타입 시스템을 활용해 어떤 에러가 발생할 수 있는지
함수 시그니처로 명확히 알 수 있습니다.`,
  diagram:
`  에러 처리 아키텍처
  ──────────────────────────────────

  라이브러리 crate:
  ┌─────────────────────────────────┐
  │ thiserror                       │
  │ #[derive(Error, Debug)]         │
  │ enum AppError {                 │
  │   NotFound(String),  ← 명확한 타입│
  │   Io(#[from] io::Error), ← 자동 │
  │ }                               │
  └─────────────────────────────────┘

  애플리케이션 crate:
  ┌─────────────────────────────────┐
  │ anyhow                          │
  │ fn main() -> Result<()> {       │
  │   File::open(path)              │
  │     .context("설정 파일 열기")?;│
  │   Ok(())                        │
  │ }                               │
  └─────────────────────────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: From 트레이트 → ? 연산자로 자동 에러 변환
//    match로 에러 복구, ?로 에러 전파
// ══════════════════════════════════════════════════════

use std::fmt;
use std::num::ParseIntError;

// ─── 커스텀 에러 타입 정의 (thiserror 없이 수동 구현) ───
#[derive(Debug)]
enum AppError {
    NotFound(String),        // 리소스를 찾을 수 없음
    ParseError(ParseIntError), // 파싱 실패 (표준 에러를 래핑)
    InvalidInput(String),    // 잘못된 입력값
}

// Display 트레이트: 사용자 친화적 에러 메시지
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ParseError(e) => write!(f, "Parse Error: {}", e),
            AppError::InvalidInput(msg) => write!(f, "Invalid: {}", msg),
        }
    }
}

// ★ From 트레이트: ? 연산자가 자동으로 에러 타입을 변환할 수 있게 함
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)  // ParseIntError → AppError 자동 변환
    }
}

// ─── ? 연산자: From 덕분에 자동 에러 변환 ───
fn parse_config(input: &str) -> Result<i32, AppError> {
    // .parse() → Result<i32, ParseIntError>
    // ? → Err(ParseIntError)를 만나면 From::from()으로 AppError::ParseError로 변환 후 반환
    let n: i32 = input.parse()?;
    if n <= 0 {
        return Err(AppError::InvalidInput(format!("{} must be positive", n)));
    }
    Ok(n)
}

fn find_user(id: i32) -> Result<String, AppError> {
    match id {
        1 => Ok(String::from("Alice")),
        _ => Err(AppError::NotFound(format!("user {}", id))),
    }
}

fn main() {
    // ─── 성공 케이스 ───
    match parse_config("42") {
        Ok(n) => println!("Config value: {}", n),   // 출력됨
        Err(e) => println!("Error: {}", e),
    }

    // ─── 다양한 에러 케이스 ───
    println!("{:?}", parse_config("abc"));    // ParseError (숫자 아님)
    println!("{:?}", parse_config("-5"));     // InvalidInput (음수)
    println!("{:?}", find_user(99));          // NotFound (사용자 없음)

    // ─── match로 선택적 복구: 특정 에러만 처리 ───
    match find_user(1) {
        Ok(name) => println!("Found: {}", name),
        Err(AppError::NotFound(msg)) => {
            // NotFound 에러만 특별 처리 (복구)
            println!("사용자 없음: {} → 기본값 사용", msg);
        }
        Err(e) => println!("다른 에러: {}", e),  // 나머지 에러
    }
}`,

  keyPoints: [
    "라이브러리: thiserror로 명확한 에러 타입 정의",
    "애플리케이션: anyhow로 간편한 에러 처리 + 컨텍스트",
    "From 트레이트 구현 → ? 연산자로 자동 에러 변환",
    "match로 에러 복구, ?로 에러 전파 — 상황에 맞게 선택"
  ],
  comparisons: [
    ["header","thiserror","anyhow"],
    ["diff","라이브러리용","애플리케이션용"],
    ["diff","명확한 에러 타입","통합 에러 타입"],
    ["diff","API 문서화에 좋음","빠른 프로토타이핑"],
    ["diff","#[derive(Error)]","anyhow::Result<T>"]
  ]
},

// ╔═══════════════════════════════════════════════════════╗
// ║  자료구조 (Data Structures)
// ╚═══════════════════════════════════════════════════════╝

// ──────────────────────────────────────────────────────
// 23. 연결 리스트 (Linked List)
// ──────────────────────────────────────────────────────
{
  title: "연결 리스트 (Linked List)",
  category: "자료구조",
  explanation:
`연결 리스트는 노드들이 포인터로 연결된 선형 자료구조입니다.

  단일 연결 리스트: 각 노드가 next 포인터만 가짐
  이중 연결 리스트: prev + next 포인터를 가짐

  배열 vs 연결 리스트:
    배열: 인덱스 접근 O(1), 삽입/삭제 O(n)
    연결 리스트: 인덱스 접근 O(n), 삽입/삭제 O(1) (위치를 알 때)

Rust에서 포인터 대신 Box<T>로 Heap 노드를 연결합니다.
소유권 모델 덕분에 메모리 누수가 원천 차단됩니다!`,
  whyItMatters:
`연결 리스트는 포인터/참조의 기본을 이해하는 최고의 학습 도구입니다.
실무에서는 Vec이 더 빠른 경우가 많지만, LRU 캐시, Undo 스택 등에 활용됩니다.
Rust의 소유권 모델이 연결 리스트 구현에 어떤 영향을 주는지 체감할 수 있습니다.`,
  diagram:
`  단일 연결 리스트 구조
  ──────────────────────────────────

  head                                         tail
    ▼                                             ▼
  ┌─────┬───┐   ┌─────┬───┐   ┌─────┬───┐   ┌─────┬────┐
  │  1  │ ──┼──►│  2  │ ──┼──►│  3  │ ──┼──►│  4  │null│
  └─────┴───┘   └─────┴───┘   └─────┴───┘   └─────┴────┘

  push_front: O(1)           push_back: O(n) or O(1)*
  ┌─────┬───┐                (*tail 포인터가 있으면)
  │  0  │ ──┼──► head
  └─────┴───┘

  이중 연결 리스트:
  ┌───┬─────┬───┐   ┌───┬─────┬───┐
  │◄──│  1  │──►│◄──│  2  │──►│
  └───┴─────┴───┘   └───┴─────┴───┘
  prev  data  next    prev  data  next`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Box<T>로 Heap 노드 연결, Option으로 끝 표시
//    소유권 모델이 메모리 누수를 원천 차단!
// ══════════════════════════════════════════════════════

use std::fmt;

// ─── 단일 연결 리스트 ───
// Option<Box<Node>>: 다음 노드가 있거나(None) 없거나(Box)
#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,  // ★ Box: Heap 할당, Option: null 대신
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }

    // 리스트 끝에 노드 추가 (재귀)
    fn append(&mut self, data: i32) {
        match &mut self.next {
            Some(node) => node.append(data),  // 다음 노드로 이동
            None => self.next = Some(Box::new(Node::new(data))),  // 끝에 추가
        }
    }

    // 리스트 출력 (재귀)
    fn print_all(&self) {
        print!("{} ", self.data);
        if let Some(node) = &self.next {
            node.print_all();
        }
    }

    // 리스트 길이 (재귀)
    fn len(&self) -> usize {
        match &self.next {
            Some(node) => 1 + node.len(),
            None => 1,
        }
    }
}

fn main() {
    // ─── 연결 리스트 생성 ───
    let mut head = Node::new(1);
    head.append(2);  // 끝에 추가: O(n)
    head.append(3);
    head.append(4);

    print!("List: ");
    head.print_all();  // 출력: List: 1 2 3 4
    println!();
    println!("Length: {}", head.len());  // 출력: Length: 4

    // ─── 앞에 삽입: O(1) ───
    let mut new_head = Node::new(0);
    new_head.next = Some(Box::new(head));  // 기존 리스트를 new_head 뒤에 연결
    print!("After prepend: ");
    new_head.print_all();  // 출력: After prepend: 0 1 2 3 4
    println!();

    // ─── head가 스코프를 벗어나면 전체 노드가 자동 drop! ───
    // ★ Rust의 소유권: head가 전체 리스트를 소유 → 순차적 drop → 메모리 누수 불가
    println!("Done! 메모리 자동 해제");
}`,
  keyPoints: [
    "Box<T>로 Heap 노드 연결 → 소유권 기반 자동 메모리 관리",
    "Option<Box<Node>>: null 대신 None으로 안전하게 끝 표시",
    "삽입/삭제: 위치를 알면 O(1), 검색이 필요하면 O(n)",
    "Rust에서는 순환 참조를 방지하기 위해 Rc<RefCell<>> 또는 약한 참조(Weak) 사용"
  ],
  comparisons: [
    ["header","배열 (Vec<T>)","연결 리스트 (Linked List)"],
    ["diff","인덱스 접근 O(1)","인덱스 접근 O(n)"],
    ["diff","삽입/삭제 O(n)","삽입/삭제 O(1) (위치 알 때)"],
    ["diff","연속 메모리 (캐시 친화적)","분산 메모리 (캐시 비친화적)"],
    ["diff","크기 변경 시 재할당","크기 변경 비용 없음"]
  ]
},

// ──────────────────────────────────────────────────────
// 24. 스택과 큐 (Stack & Queue)
// ──────────────────────────────────────────────────────
{
  title: "스택과 큐 (Stack & Queue)",
  category: "자료구조",
  explanation:
`스택 (LIFO: Last In, First Out)
  마지막에 넣은 데이터를 먼저 꺼냄
  push/pop 모두 O(1)
  활용: 함수 호출 스택, 괄호 짝 맞추기, Undo

큐 (FIFO: First In, First Out)
  먼저 넣은 데이터를 먼저 꺼냄
  enqueue/dequeue 모두 O(1)
  활용: BFS, 작업 스케줄러, 버퍼

Rust의 Vec은 스택으로 사용 가능 (push/pop)
VecDeque은 큐로 사용 가능 (push_back/pop_front)`,
  whyItMatters:
`스택은 함수 호출, 재귀, 메모리 관리의 기본입니다.
큐는 네트워크 패킷 처리, 작업 스케줄링의 핵심입니다.
이 두 자료구조를 이해하면 OS, 네트워크, 알고리즘 전반을 이해할 수 있습니다.`,
  diagram:
`  스택 (LIFO)                    큐 (FIFO)
  ─────────────────              ─────────────────

  push ──► ┌───┐                ┌───┐                ┌───┐ ──► dequeue
           │ 3 │ ◄── pop        │ 1 │ ──► ... ──►    │ 3 │
           ├───┤                ├───┤     ┌───┐     ├───┤
           │ 2 │                │ 2 │ ──► │ 2 │     │ 2 │
           ├───┤                └───┘     └───┘     └───┘
           │ 1 │                  ▲                   ▲
           └───┘               enqueue              enqueue

  함수 호출 스택:                BFS 큐:
  main() → add() → add()
  ┌──────────────┐              [시작] → [A, B] → [B, C, D] → ...
  │ add(3, 4)   │ ← 먼저 끝남
  │ add(1, 2)   │
  │ main()       │ ← 마지막에 끝남
  └──────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 스택 = Vec의 push/pop, 큐 = VecDeque의 push_back/pop_front
// ══════════════════════════════════════════════════════

use std::collections::VecDeque;

fn main() {
    // ═══════════ 스택 (LIFO): Vec 사용 ═══════════
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);        // push: O(1)
    stack.push(2);
    stack.push(3);
    println!("Stack: {:?}", stack);  // [1, 2, 3]

    let top = stack.pop();           // pop: O(1) → 마지막 요소 제거
    println!("Popped: {:?}", top);   // Some(3)
    println!("Stack: {:?}", stack);  // [1, 2]

    // ★ 실전 예제: 괄호 짝 맞추기
    let code = "fn main() { vec![1, 2] }";
    println!("괄호 유효: {}", is_balanced(code));  // true
    println!("괄호 유효: {}", is_balanced("({[")); // false

    // ═══════════ 큐 (FIFO): VecDeque 사용 ═══════════
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(1);     // enqueue: O(1)
    queue.push_back(2);
    queue.push_back(3);
    println!("Queue: {:?}", queue);  // [1, 2, 3]

    let front = queue.pop_front();    // dequeue: O(1) → 첫 요소 제거
    println!("Dequeued: {:?}", front);  // Some(1)
    println!("Queue: {:?}", queue);     // [2, 3]

    // ★ 실전 예제: BFS 최단 거리
    let graph = vec![
        vec![1, 2],    // 노드 0 → 1, 2 연결
        vec![0, 3],    // 노드 1 → 0, 3
        vec![0, 3],    // 노드 2 → 0, 3
        vec![1, 2],    // 노드 3 → 1, 2
    ];
    let distances = bfs(&graph, 0);
    println!("0번부터 거리: {:?}", distances);  // [0, 1, 1, 2]
}

// 스택 활용: 괄호 유효성 검사
fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),   // 여는 괄호 push
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}
        }
    }
    stack.is_empty()  // 스택이 비어야 모두 짝 맞음
}

// 큐 활용: BFS 최단 거리
fn bfs(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);          // 시작 노드 enqueue

    while let Some(node) = queue.pop_front() {  // dequeue
        for &neighbor in &graph[node] {
            if dist[neighbor] == usize::MAX {    // 아직 방문 안 함
                dist[neighbor] = dist[node] + 1; // 거리 +1
                queue.push_back(neighbor);        // enqueue
            }
        }
    }
    dist
}`,
  keyPoints: [
    "스택(LIFO): Vec의 push/pop → 함수 호출, Undo, 괄호 검사",
    "큐(FIFO): VecDeque의 push_back/pop_front → BFS, 스케줄러",
    "괄호 짝 맞추기: 여는 괄호 push, 닫는 괄호 pop해서 비교",
    "BFS: 큐로 가까운 노드부터 방문 → 최단 거리 보장"
  ],
  comparisons: [
    ["header","스택 (Stack)","큐 (Queue)"],
    ["diff","LIFO (후입선출)","FIFO (선입선출)"],
    ["diff","push/pop (같은 쪽)","enqueue/dequeue (반대쪽)"],
    ["diff","함수 호출 스택, Undo","BFS, 작업 대기열"],
    ["equal","둘 다 O(1) 삽입/삭제","둘 다 O(1) 삽입/삭제"]
  ]
},

// ──────────────────────────────────────────────────────
// 25. 트리와 이진 탐색 트리
// ──────────────────────────────────────────────────────
{
  title: "트리와 이진 탐색 트리 (BST)",
  category: "자료구조",
  explanation:
`트리: 계층적 자료구조. 루트에서 시작해 자식 노드로 뻗어나감

  이진 트리: 각 노드가 최대 2개의 자식
  이진 탐색 트리(BST): 왼쪽 < 부모 < 오른쪽 정렬 규칙

  BST 핵심 연산:
    검색: O(log n) 평균, O(n) 최악
    삽입: O(log n) 평균
    삭제: O(log n) 평균
    중위 순회: 정렬된 순서로 출력

  최악의 경우(한쪽으로 치우침) → 균형 트리(AVL, Red-Black)로 해결`,
  whyItMatters:
`BST는 검색, 정렬, 범위 쿼리의 기본입니다.
데이터베이스 인덱스(B-Tree), 파일 시스템, 의사결정 트리 등에 활용됩니다.
Rust의 BTreeMap이 균형 BST 기반으로 동작합니다.`,
  diagram:
`  이진 탐색 트리 (BST)
  ──────────────────────────────────

          8
        /   \
       3     10
      / \      \
     1   6     14
        / \    /
       4   7  13

  ★ BST 규칙: 왼쪽 < 부모 < 오른쪽
  - 3의 왼쪽: 1 (< 3) ✓
  - 3의 오른쪽: 6 (> 3) ✓
  - 8의 왼쪽: 3 (< 8) ✓
  - 8의 오른쪽: 10 (> 8) ✓

  중위 순회(in-order): 1 → 3 → 4 → 6 → 7 → 8 → 10 → 13 → 14
  = 정렬된 순서!

  최악의 경우 (치우침):
  1 → 2 → 3 → 4 → 5  (사실상 연결 리스트)
  검색: O(n) → 균형 트리로 해결!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: BST = 왼쪽 < 부모 < 오른쪽
//    중위 순회하면 정렬된 순서!
// ══════════════════════════════════════════════════════

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,   // 왼쪽 자식 (값이 더 작음)
    right: Option<Box<TreeNode>>,  // 오른쪽 자식 (값이 더 큼)
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { value, left: None, right: None }
    }

    // ─── BST 삽입: O(log n) 평균 ───
    fn insert(&mut self, value: i32) {
        if value < self.value {
            // 값이 더 작으면 왼쪽으로
            match &mut self.left {
                Some(node) => node.insert(value),  // 재귀로 내려감
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if value > self.value {
            // 값이 더 크면 오른쪽으로
            match &mut self.right {
                Some(node) => node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
        // 같은 값은 무시
    }

    // ─── BST 검색: O(log n) 평균 ───
    fn search(&self, target: i32) -> bool {
        if target == self.value {
            return true;              // 찾음!
        } else if target < self.value {
            match &self.left {
                Some(node) => node.search(target),   // 왼쪽에서 계속
                None => false,                        // 없음
            }
        } else {
            match &self.right {
                Some(node) => node.search(target),   // 오른쪽에서 계속
                None => false,
            }
        }
    }

    // ─── 중위 순회: 왼쪽 → 자신 → 오른쪽 = 정렬 순서! ───
    fn in_order(&self, result: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.in_order(result);  // 1. 왼쪽 자식 먼저
        }
        result.push(self.value);    // 2. 자신
        if let Some(right) = &self.right {
            right.in_order(result); // 3. 오른쪽 자식
        }
    }
}

fn main() {
    let mut root = TreeNode::new(8);
    root.insert(3);    //     8
    root.insert(10);   //    / \\
    root.insert(1);    //   3   10
    root.insert(6);    //  / \\   \\
    root.insert(14);   // 1   6   14
    root.insert(4);    //    / \\  /
    root.insert(7);    //   4   7 13
    root.insert(13);

    // 검색
    println!("검색 6: {}", root.search(6));   // true
    println!("검색 99: {}", root.search(99)); // false

    // 중위 순회 = 정렬된 순서!
    let mut sorted = Vec::new();
    root.in_order(&mut sorted);
    println!("정렬: {:?}", sorted);  // [1, 3, 4, 6, 7, 8, 10, 13, 14]
}`,
  keyPoints: [
    "BST: 왼쪽 < 부모 < 오른쪽 → 검색/삽입 O(log n)",
    "중위 순회(in-order)하면 정렬된 순서로 출력",
    "최악 O(n) → 균형 트리(AVL, Red-Black)로 보장",
    "Box<TreeNode>로 Heap에 노드 할당, 소유권으로 자동 메모리 관리"
  ],
  comparisons: [
    ["header","배열 (정렬)","BST"],
    ["diff","검색 O(log n) 이진 탐색","검색 O(log n) 트리 탐색"],
    ["diff","삽입 O(n)","삽입 O(log n)"],
    ["diff","범위 쿼리 O(n)","범위 쿼리 O(log n + k)"],
    ["diff","연속 메모리","분산 메모리"]
  ]
},

// ──────────────────────────────────────────────────────
// 26. 해시 테이블 (Hash Table)
// ──────────────────────────────────────────────────────
{
  title: "해시 테이블 (Hash Table)",
  category: "자료구조",
  explanation:
`해시 테이블: 키를 해시 함수로 배열 인덱스에 매핑하여 O(1) 평균 접근

  핵심 구조:
    버킷 배열 → 각 버킷에 키-값 쌍 저장
    해시 함수 → 키를 인덱스로 변환
    충돌 처리 → 체이닝(연결 리스트) 또는 개방 주소법

  연산 복잡도:
    삽입/검색/삭제: O(1) 평균, O(n) 최악

  Rust의 HashMap이 해시 테이블 구현체입니다.
  SipHash 1-3으로 해시 충돌 공격(SWF)을 방어합니다.`,
  whyItMatters:
`해시 테이블은 실무에서 가장 많이 쓰이는 자료구조입니다.
캐시(Redis), DB 인덱스(Hash Index), DNS, 라우팅 테이블 등에 활용됩니다.
JavaScript의 Object, Python의 dict, Rust의 HashMap 모두 해시 테이블입니다.`,
  diagram:
`  해시 테이블 구조
  ──────────────────────────────────

  key → [해시 함수] → index → [버킷 배열]

  예: 버킷 크기 = 8
  ┌───────┬─────────────────────┐
  │ 인덱스 │ 버킷 (체이닝)       │
  ├───────┼─────────────────────┤
  │   0   │                     │
  │   1   │ ("age", 25)         │
  │   2   │                     │
  │   3   │ ("name","Kim")→("city","Seoul")  ← 충돌!
  │   4   │                     │
  │   5   │ ("job","dev")       │
  │   6   │                     │
  │   7   │                     │
  └───────┴─────────────────────┘

  "name".hash() % 8 = 3
  "city".hash() % 8 = 3  ← 같은 버킷! → 체이닝으로 연결

  로드 팩터 = n/k (데이터 수 / 버킷 수)
  보통 0.75 초과 시 리해싱(배열 크기 2배)`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 해시 함수로 O(1) 평균 접근, 충돌은 체이닝으로 처리
//    Rust의 HashMap이 이 구조를 그대로 구현
// ══════════════════════════════════════════════════════

use std::collections::HashMap;

fn main() {
    // ─── 기본 사용법 ───
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 90);   // O(1) 평균
    scores.insert(String::from("Bob"), 75);
    scores.insert(String::from("Charlie"), 85);

    // 검색: O(1) 평균
    println!("Alice: {:?}", scores.get("Alice"));  // Some(90)
    println!("Unknown: {:?}", scores.get("Dave")); // None

    // ─── 실전 패턴: 단어 빈도수 세기 ───
    let text = "hello world hello rust hello world";
    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // ★ entry().or_insert() + count: 0에서 시작해 증가
        let count = word_count.entry(word).or_insert(0);
        *count += 1;  // 가변 참조로 값 증가
    }
    println!("단어 빈도: {:?}", word_count);
    // {"world": 2, "hello": 3, "rust": 1}

    // ─── 실전 패턴: Two Sum ───
    // "배열에서 두 수의 합이 target이 되는 인덱스 찾기"
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(&nums, target);
    println!("Two Sum: {:?}", result);  // Some((0, 1))

    // ─── 실전 패턴: 그룹화 ───
    let people = vec![("A", "Seoul"), ("B", "Busan"), ("C", "Seoul")];
    let mut cities: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, city) in &people {
        cities.entry(city).or_insert(Vec::new()).push(name);
    }
    println!("도시별: {:?}", cities);  // {"Seoul": ["A","C"], "Busan": ["B"]}
}

// 해시 테이블로 O(n) Two Sum 해결 (브루트포스는 O(n²))
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;     // 찾아야 할 값
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));           // 이전에 본 값과 짝!
        }
        seen.insert(num, i);               // 현재 값 저장
    }
    None
}`,
  keyPoints: [
    "해시 함수: 키 → 인덱스 변환 → O(1) 평균 접근",
    "충돌 처리: 체이닝(연결 리스트) 또는 개방 주소법",
    "entry().or_insert(): upsert 패턴으로 빈도수/그룹화에 활용",
    "Two Sum: 해시 테이블로 O(n²) → O(n) 최적화"
  ],
  comparisons: [
    ["header","배열 (정렬)","BST","해시 테이블"],
    ["diff","검색 O(log n)","검색 O(log n)","검색 O(1) 평균"],
    ["diff","순서 보장","순서 보장","순서 없음"],
    ["diff","범위 쿼리 가능","범위 쿼리 가능","범위 쿼리 불가"],
    ["diff","메모리 적음","중간","메모리 많음"]
  ]
},

// ╔═══════════════════════════════════════════════════════╗
// ║  운영체제 (Operating Systems)
// ╚═══════════════════════════════════════════════════════╝

// ──────────────────────────────────────────────────────
// 27. 프로세스와 스레드
// ──────────────────────────────────────────────────────
{
  title: "프로세스와 스레드",
  category: "운영체제",
  explanation:
`프로세스: 실행 중인 프로그램의 인스턴스
  독립된 메모리 공간 (코드, 데이터, 힙, 스택)
  컨텍스트 스위칭 비용이 큼

스레드: 프로세스 내 실행 단위
  메모리 공간(힙, 데이터)을 공유
  컨텍스트 스위칭 비용이 작음
  각자 고유한 스택과 레지스터 보유

  Rust에서:
    thread::spawn → OS 네이티브 스레드 (1:1 모델)
    async/await → 논그린 스레드 (M:N 모델의 일종)
    Go의 고루틴 vs Rust의 스레드: 서로 다른 동시성 모델`,
  whyItMatters:
`백엔드 성능 튜닝의 핵심은 프로세스/스레드 모델을 이해하는 것입니다.
NestJS는 단일 프로세스 + 이벤트 루프, Rust는 멀티스레드 직접 제어.
컨테이너(Docker)는 프로세스 격리 기술의 일종입니다.`,
  diagram:
`  프로세스 vs 스레드 메모리 구조
  ──────────────────────────────────

  ┌─── 프로세스 ──────────────────────┐
  │  ┌─ 스레드 1 ─┐  ┌─ 스레드 2 ─┐ │
  │  │ Stack 1    │  │ Stack 2    │ │  ← 각자 고유
  │  │ Registers  │  │ Registers  │ │  ← 각자 고유
  │  └────────────┘  └────────────┘ │
  │                                    │
  │  ┌─ 공유 영역 ─────────────────┐ │
  │  │ Heap (동적 할당)           │ │  ← 공유!
  │  │ Data (전역 변수)           │ │  ← 공유!
  │  │ Text (코드)                │ │  ← 공유!
  │  └────────────────────────────┘ │
  └────────────────────────────────┘

  컨텍스트 스위칭 비용:
  프로세스: 높음 (메모리 공간 전체 교체, TLB 플러시)
  스레드: 낮음 (스택+레지스터만 교체, 캐시 유지)

  Node.js (싱글 스레드):
  ┌─ Main Thread ──────────┐
  │ Event Loop              │
  │  ┌───┐ ┌───┐ ┌───┐   │
  │  │ CB│→│ CB│→│ CB│   │  ← 콜백 큐
  │  └───┘ └───┘ └───┘   │
  └────────────────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 프로세스 = 독립 메모리, 스레드 = 메모리 공유
//    Rust의 thread::spawn = OS 네이티브 스레드 (1:1)
// ══════════════════════════════════════════════════════

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    println!("메인 스레드 PID: {}", std::process::id());

    // ─── 1:1 모델: 각 thread::spawn = OS 커널 스레드 1개 ───
    let handle = thread::spawn(|| {
        println!("  자식 스레드 시작");
        thread::sleep(Duration::from_millis(100));
        println!("  자식 스레드 종료");
    });
    handle.join().unwrap();

    // ─── 스레드는 힙 메모리를 공유한다 (Arc<Mutex<T>>) ───
    // 여러 스레드가 같은 데이터를 수정 → 데이터 경쟁 주의!
    let shared_data = Arc::new(Mutex::new(vec![]));  // 공유 벡터
    let mut handles = vec![];

    for id in 0..3 {
        let data = Arc::clone(&shared_data);  // 참조 카운트 증가
        let h = thread::spawn(move || {
            let mut v = data.lock().unwrap();  // ★ 뮤텍스로 상호 배제
            v.push(format!("thread-{}", id));  // 공유 힙에 안전하게 쓰기
            println!("  스레드 {} 쓰기 완료", id);
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    println!("공유 결과: {:?}", *shared_data.lock().unwrap());
    // 출력: ["thread-0", "thread-1", "thread-2"] (순서는 다를 수 있음)

    // ─── 프로세스 vs 스레드 비교 ───
    // 프로세스 분리: std::process::Command (완전히 독립된 메모리)
    let output = std::process::Command::new("echo")
        .arg("별도 프로세스 실행")
        .output()
        .expect("실행 실패");
    println!("자식 프로세스: {}",
        String::from_utf8_lossy(&output.stdout).trim());
}`,
  keyPoints: [
    "프로세스: 독립 메모리, 스위칭 비용 큼, 안전 (격리)",
    "스레드: 메모리 공유, 스위칭 비용 작음, 데이터 경쟁 주의",
    "Rust thread::spawn = OS 네이티브 스레드 (1:1 모델)",
    "Arc<Mutex<T>>로 스레드 간 안전하게 공유 데이터 수정"
  ],
  comparisons: [
    ["header","프로세스","스레드"],
    ["diff","독립 메모리 공간","메모리 공유"],
    ["diff","컨텍스트 스위칭 비용 큼","컨텍스트 스위칭 비용 작음"],
    ["diff","IPC 필요 (파이프, 소켓)","직접 메모리 접근"],
    ["diff","안전 (격리)","데이터 경쟁 위험"]
  ]
},

// ──────────────────────────────────────────────────────
// 28. 메모리 관리
// ──────────────────────────────────────────────────────
{
  title: "메모리 관리 (스택, 힙, 가상 메모리)",
  category: "운영체제",
  explanation:
`메모리 계층:
  레지스터 → L1/L2 캐시 → RAM → 디스크 (SSD/HDD)

  스택(Stack):
    컴파일 타임에 크기 결정, LIFO, 매우 빠름
    함수 호출 시 프레임 생성, 반환 시 해제

  힙(Heap):
    런타임에 크기 결정, 동적 할당
    malloc/free, new/delete, GC
    단편화(Fragmentation) 문제

  가상 메모리:
    프로세스마다 독립된 주소 공간
    페이지 테이블로 물리 메모리 매핑
    페이지 폴트 → 디스크에서 로드

  Rust의 선택: 소유권 + drop = GC 없는 자동 관리`,
  whyItMatters:
`메모리 관리를 이해하면 성능 문제의 원인을 파악할 수 있습니다.
OOM, 메모리 누수, 캐시 미스 → 실무에서 자주 겪는 문제.
Rust의 소유권 모델이 OS 수준의 메모리 관리와 어떻게 연결되는지 이해가 필수.`,
  diagram:
`  프로세스 메모리 레이아웃
  ──────────────────────────────────

  높은 주소 ┌──────────────────┐
            │   Stack ↓        │ ← 지역 변수, 함수 프레임
            │   ...            │    (스레드마다 고유)
            │                  │
            ├──────────────────┤
            │   Shared Libs    │ ← libc, std
            ├──────────────────┤
            │   Heap ↑         │ ← Box, String, Vec
            │   ...            │    (동적 할당)
            ├──────────────────┤
            │   BSS            │ ← 초기화 안 된 전역
            │   Data           │ ← 초기화된 전역
            │   Text (Code)    │ ← 기계어 명령
  낮은 주소 └──────────────────┘

  가상 메모리 → 물리 메모리 매핑:
  ┌─────────────┐     ┌─────────────┐
  │ 가상 주소    │     │ 물리 주소    │
  │ Page 0: 0x0 │     │ Frame 3     │
  │ Page 1: 4KB │────►│ Frame 7     │
  │ Page 2: 8KB │     │ Frame 1     │
  └─────────────┘     └─────────────┘
   페이지 테이블로 매핑`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 스택 = 빠르고 자동, 힙 = 유연하지만 비용 있음
//    Rust의 소유권 = 힙 메모리를 GC 없이 자동 관리
// ═══════════════════════════════════════════════════════

fn main() {
    // ─── 스택 할당: 컴파일 타임에 크기 결정 ───
    let x: i32 = 42;              // 스택에 4바이트
    let arr: [i32; 3] = [1,2,3]; // 스택에 12바이트
    println!("스택 변수: {} {:?}", x, arr);

    // ─── 힙 할당: 런타임에 크기 결정 ───
    let s = String::from("hello");   // 힙에 문자열 데이터
    let v = vec![1, 2, 3, 4, 5];   // 힙에 배열 데이터
    let b = Box::new(42);            // 힙에 정수
    println!("힙 변수: {} {:?} {}", s, v, b);

    // ─── 소유권 = 자동 메모리 관리 ───
    {
        let local = String::from("임시 데이터");
        println!("{}", local);
    } // ★ local이 drop()됨 → 힙 메모리 즉시 해제!
    // Java라면 GC가 나중에 수거, C라면 수동 free 필요

    // ─── 함수 호출과 스택 프레임 ───
    fn add(a: i32, b: i32) -> i32 {
        // 스택 프레임 생성: [return addr][a][b]
        let result = a + b;  // 스택에 result
        result               // 반환값 → 호출자의 스택으로 복사
    }  // 스택 프레임 해제

    let sum = add(3, 4);
    println!("sum = {}", sum);

    // ─── 메모리 크기 확인 ───
    println!("i32 크기: {} bytes", std::mem::size_of::<i32>());     // 4
    println!("&str 크기: {} bytes", std::mem::size_of::<&str>());   // 16 (ptr+len)
    println!("String 크기: {} bytes", std::mem::size_of::<String>()); // 24 (ptr+len+cap)
    println!("Box<i32> 크기: {} bytes", std::mem::size_of::<Box<i32>>()); // 8 (ptr)

    // ─── 힙 할당이 비싼 이유 ───
    // 1. 메모리 할당자(malloc/jemalloc) 호출
    // 2. 단편화 가능성
    // 3. 캐시 비친화적 (비연속 메모리)
    // → 성능이 중요하면 스택/Vec 재사용 선호!
}`,
  keyPoints: [
    "스택: 빠름, 자동 관리, 크기 고정 → 지역 변수, 함수 프레임",
    "힙: 유연, 수동/GC/소유권 관리, 단편화 위험 → 동적 데이터",
    "가상 메모리: 프로세스마다 독립 주소 공간, 페이지 단위 매핑",
    "Rust의 소유권: 힙 메모리를 GC 없이 스코프 기반 자동 해제"
  ],
  comparisons: [
    ["header","스택","힙"],
    ["diff","매우 빠름 (포인터 이동만)","느림 (할당자 호출)"],
    ["diff","크기 컴파일 타임 결정","크기 런타임 결정"],
    ["diff","자동 관리 (스코프)","수동/GC/소유권 관리"],
    ["diff","캐시 친화적 (연속)","캐시 비친화적 (분산)"]
  ]
},

// ──────────────────────────────────────────────────────
// 29. 동기화와 락
// ──────────────────────────────────────────────────────
{
  title: "동기화와 락 (Mutex, RwLock, Semaphore)",
  category: "운영체제",
  explanation:
`동기화: 여러 스레드가 공유 자원에 안전하게 접근하는 메커니즘

  Mutex (상호 배제):
    한 번에 하나의 스레드만 접근
    lock()/unlock()으로 보호

  RwLock (읽기-쓰기 락):
    읽기: 여러 스레드 동시 가능 (Shared)
    쓰기: 하나의 스레드만 (Exclusive)

  세마포어 (Semaphore):
    N개의 허가를 관리 (뮤텍스는 1개만)
    제한된 리소스 풀 관리에 유용

  교착상태(Deadlock) 4조건:
    상호 배제 + 점유 대기 + 비선점 + 순환 대기`,
  whyItMatters:
`백엔드에서 동시성 버그는 재현이 어렵고 디버깅이 힘듭니다.
DB 커넥션 풀, 파일 잠금, 분산 락 모두 동기화 개념의 연장입니다.
Rust는 컴파일 타임에 데이터 경쟁을 방지합니다 (Send + Sync).`,
  diagram:
`  락의 동작 원리
  ──────────────────────────────────

  Mutex:
  Thread A: lock() ──► [크리티컬 섹션] ──► unlock()
  Thread B: lock() ──── 대기 ──────────────────► lock() ──► [CS] ──► unlock()
  Thread C: lock() ──── 대기 ────────────────────────────────────► lock()

  RwLock:
  Reader 1: read()  ──► [읽기] ──► (동시 OK!)
  Reader 2: read()  ──► [읽기] ──► (동시 OK!)
  Writer:   write() ────── 대기(리더 완료까지) ──► [쓰기] ──► unlock()
  Reader 3: read()  ────────────── 대기(라이터 완료까지) ──► [읽기]

  교착상태 (Deadlock):
  Thread A: lock(X) ──► lock(Y) 대기 ────────────────── ▓▓▓▓
  Thread B: lock(Y) ──► lock(X) 대기 ────────────────── ▓▓▓▓
  → 서로 상대방의 락을 기다리며 영원히 진행 불가!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Mutex = 1명만, RwLock = 읽기 여러명/쓰기 1명
//    락 획득 순서를 일관되게 하면 교착상태 방지!
// ══════════════════════════════════════════════════════

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // ─── Mutex: 한 번에 하나만 접근 ───
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // ★ 락 획득
            *num += 1;                               // 안전하게 수정
        });  // num drop → 락 자동 해제!
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }
    println!("Mutex 결과: {}", *counter.lock().unwrap());  // 10

    // ─── RwLock: 읽기는 여러 스레드, 쓰기는 하나만 ───
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles2 = vec![];

    // 읽기 스레드 3개 (동시에 읽기 가능!)
    for id in 0..3 {
        let data = Arc::clone(&data);
        let h = thread::spawn(move || {
            let r = data.read().unwrap();  // ★ 읽기 락 (여러 개 동시 OK)
            println!("  Reader {}: {:?}", id, *r);
        });
        handles2.push(h);
    }

    // 쓰기 스레드 (읽기 완료 후에만 실행됨)
    {
        let data = Arc::clone(&data);
        let mut w = data.write().unwrap();  // ★ 쓰기 락 (배타적)
        w.push(4);
        println!("  Writer: {:?}", *w);  // [1, 2, 3, 4]
    }  // 쓰기 락 해제

    for h in handles2 { h.join().unwrap(); }
    println!("RwLock 결과: {:?}", *data.read().unwrap());

    // ─── 교착상태 방지 팁 ───
    // 1. 항상 같은 순서로 락 획득 (A→B 순서 통일)
    // 2. 락 보유 시간 최소화
    // 3. Rust: MutexGuard가 drop에서 자동 unlock → 실수 방지
}`,
  keyPoints: [
    "Mutex: 한 스레드만 접근 → 카운터, 공유 상태 보호",
    "RwLock: 읽기 여러 개 + 쓰기 1개 → 설정/캐시에 유용",
    "Rust: MutexGuard가 drop에서 자동 unlock → unlock 실수 방지",
    "교착상태: 락 획득 순서 통일, 락 시간 최소화로 방지"
  ],
  comparisons: [
    ["header","Mutex","RwLock"],
    ["diff","한 스레드만 접근","읽기 여러 + 쓰기 1개"],
    ["diff","구현 단순","구현 복잡"],
    ["left","카운터, 공유 상태","설정, 캐시"],
    ["diff","lock()/unlock()","read()/write()"]
  ]
},

// ──────────────────────────────────────────────────────
// 30. 파일 시스템과 I/O
// ──────────────────────────────────────────────────────
{
  title: "파일 시스템과 I/O",
  category: "운영체제",
  explanation:
`파일 시스템: 디스크의 데이터를 계층적 디렉토리로 관리

  핵심 개념:
    inode: 파일의 메타데이터 (크기, 권한, 위치)
    디렉토리: 파일 이름 → inode 매핑
    블록: 디스크 I/O의 최소 단위 (보통 4KB)

  I/O 모델:
    블로킹 I/O: 요청 후 완료까지 대기
    논블로킹 I/O: 요청 후 즉시 반환 (준비되면 알림)
    비동기 I/O: 커널이 백그라운드에서 처리

  Rust의 I/O:
    std::fs: 블로킹 파일 I/O
    std::net: 블로킹 네트워크 I/O
    tokio: 비동기 I/O 프레임워크`,
  whyItMatters:
`모든 백엔드 작업은 결국 I/O입니다: 파일 읽기, DB 쿼리, HTTP 요청.
블로킹 vs 비동기 I/O의 차이를 이해하면 시스템 설계 능력이 크게 향상됩니다.
Node.js의 비동기 I/O, Rust의 tokio가 같은 문제를 다르게 해결합니다.`,
  diagram:
`  파일 시스템 계층 구조
  ──────────────────────────────────

  애플리케이션
      │ std::fs::read()
      ▼
  VFS (가상 파일 시스템) ← ext4, APFS, NTFS 통일 인터페이스
      │
      ▼
  페이지 캐시 ← 자주 접근하는 데이터를 RAM에 캐시
      │ (캐시 미스면)
      ▼
  블록 디바이스 드라이버
      │
      ▼
  디스크 (SSD/HDD)

  inode 구조:
  ┌─────────────────────┐
  │ 파일 크기: 1024     │
  │ 권한: rw-r--r--     │
  │ 소유자: uid 1000    │
  │ 블록 위치: [5,6,7] │  ← 실제 데이터 위치
  │ 수정 시간: ...       │
  └─────────────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: I/O는 블로킹/논블로킹/비동기 세 가지 모델
//    파일 = 커널 버퍼 → 페이지 캐시 → 디스크의 계층
// ══════════════════════════════════════════════════════

use std::fs;
use std::io::{self, Read, Write, BufReader, BufWriter};

fn main() -> io::Result<()> {
    // ─── 파일 쓰기 (블로킹 I/O) ───
    // 커널 버퍼 → 페이지 캐시 → (나중에) 디스크에 기록
    fs::write("/tmp/cs_bite_test.txt", "Hello, Rust I/O!")?;
    println!("파일 쓰기 완료");

    // ─── 파일 읽기 (블로킹 I/O) ───
    let content = fs::read_to_string("/tmp/cs_bite_test.txt")?;
    println!("파일 내용: {}", content);  // Hello, Rust I/O!

    // ─── 버퍼링된 I/O: 시스템 콜 횟수 감소 ───
    // BufWriter: 데이터를 모았다가 한 번에 write → 효율 ↑
    {
        let file = fs::File::create("/tmp/cs_bite_buf.txt")?;
        let mut writer = BufWriter::new(file);
        for i in 0..1000 {
            writeln!(writer, "Line {}", i)?;  // 버퍼에 모음
        }
        writer.flush()?;  // ★ 버퍼의 내용을 실제로 디스크에 기록
    }

    // BufReader: 데이터를 큰 덩어리로 읽어서 버퍼에서 제공
    {
        let file = fs::File::open("/tmp/cs_bite_buf.txt")?;
        let reader = BufReader::new(file);
        let line_count = reader.lines().count();
        println!("줄 수: {}", line_count);  // 1000
    }

    // ─── 파일 메타데이터 (inode 정보) ───
    let meta = fs::metadata("/tmp/cs_bite_test.txt")?;
    println!("파일 크기: {} bytes", meta.len());
    println!("읽기 가능: {}", meta.permissions().readonly());

    // ─── 디렉토리 순회 ───
    println!("\n/tmp 내용 (일부):" );
    for entry in fs::read_dir("/tmp")?.take(5) {
        let entry = entry?;
        println!("  {}", entry.path().display());
    }

    // ─── 정리 ───
    fs::remove_file("/tmp/cs_bite_test.txt")?;
    fs::remove_file("/tmp/cs_bite_buf.txt")?;
    println!("\n정리 완료");

    Ok(())
}`,
  keyPoints: [
    "파일 I/O: 커널 버퍼 → 페이지 캐시 → 디스크의 계층적 처리",
    "BufReader/BufWriter: 시스템 콜 횟수를 줄여 성능 향상",
    "블로킹 I/O vs 비동기 I/O: Node.js의 libuv vs Rust의 tokio",
    "inode: 파일 메타데이터, 페이지 캐시: 자주 쓰는 데이터 RAM 캐시"
  ],
  comparisons: [
    ["header","블로킹 I/O","비동기 I/O (tokio)"],
    ["diff","요청 후 대기","요청 후 즉시 반환"],
    ["diff","구현 단순","구현 복잡"],
    ["diff","스레드당 1요청","스레드당 수천 요청"],
    ["left","std::fs, std::net","tokio::fs, tokio::net"]
  ]
},

// ╔═══════════════════════════════════════════════════════╗
// ║  네트워크 (Networking)
// ╚═══════════════════════════════════════════════════════╝

// ──────────────────────────────────────────────────────
// 31. TCP/UDP와 소켓 프로그래밍
// ──────────────────────────────────────────────────────
{
  title: "TCP/UDP와 소켓 프로그래밍",
  category: "네트워크",
  explanation:
`TCP (Transmission Control Protocol):
  연결 지향, 신뢰성 보장, 순서 보장
  3-way handshake로 연결, 4-way handshake로 종료
  HTTP, HTTPS, 웹소켓에서 사용

  UDP (User Datagram Protocol):
  비연결, 신뢰성 없음, 빠름
  스트리밍, DNS, 게임에서 사용

  소켓: 네트워크 통신의 끝점
  IP 주소 + 포트 번호로 식별

  Rust의 std::net: 블로킹 소켓 I/O 제공`,
  whyItMatters:
`모든 웹 서비스는 TCP/UDP 위에서 동작합니다.
HTTP → TCP 위에서 동작하는 프로토콜입니다.
NestJS의 HTTP 서버도 결국 TCP 소켓을 열고 클라이언트를 기다립니다.`,
  diagram:
`  TCP 3-way Handshake
  ──────────────────────────────────

  Client                        Server
    │                              │
    │──── SYN ────────────────────►│  1. 연결 요청
    │◄─── SYN+ACK ───────────────│  2. 요청 수락
    │──── ACK ────────────────────►│  3. 확인
    │                              │
    │◄═══ 데이터 교환 ═══════════►│
    │                              │
    │──── FIN ────────────────────►│  연결 종료
    │◄─── FIN+ACK ───────────────│

  TCP vs UDP:
  ──────────────────────────────────
  TCP: ──[패1]──[패2]──[패3]──► 순서 보장, 재전송
  UDP: ──[패1]....[패3]──────────► 빠름, 손실 가능`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: TCP = 신뢰성 보장, UDP = 빠르지만 손실 가능
//    소켓 = IP + 포트로 통신하는 끝점
// ══════════════════════════════════════════════════════

use std::net::{TcpListener, TcpStream, UdpSocket};
use std::io::{Read, Write};
use std::thread;

fn main() {
    // ═══════════ TCP 서버 ═══════════
    // bind → listen → accept 루프
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();  // 0 = 자동 포트 할당
    let port = listener.local_addr().unwrap().port();
    println!("TCP 서버 시작: 127.0.0.1:{}", port);

    // 서버를 별도 스레드에서 실행
    let server_handle = thread::spawn(move || {
        for stream in listener.incoming().take(1) {  // 1개 연결만 처리
            let mut stream = stream.unwrap();
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf).unwrap();  // 데이터 읽기
            let msg = String::from_utf8_lossy(&buf[..n]);
            println!("  서버 수신: {}", msg);

            let response = format!("Echo: {}", msg);
            stream.write_all(response.as_bytes()).unwrap();  // 응답 전송
        }
    });

    // ═══════════ TCP 클라이언트 ═══════════
    // connect → write → read
    let mut stream = TcpStream::connect(
        format!("127.0.0.1:{}", port)
    ).unwrap();
    stream.write_all(b"Hello TCP!").unwrap();  // 데이터 전송

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    println!("  클라이언트 수신: {}",
        String::from_utf8_lossy(&buf[..n]));  // Echo: Hello TCP!

    server_handle.join().unwrap();

    // ═══════════ UDP ═══════════
    let server = UdpSocket::bind("127.0.0.1:0").unwrap();
    let client = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server.local_addr().unwrap();

    // ★ 연결 없이 바로 전송!
    client.send_to(b"Hello UDP!", server_addr).unwrap();
    let mut buf = [0; 1024];
    let (n, _) = server.recv_from(&mut buf).unwrap();
    println!("  UDP 수신: {}",
        String::from_utf8_lossy(&buf[..n]));  // Hello UDP!
}`,
  keyPoints: [
    "TCP: 연결 지향, 신뢰성 보장, 순서 보장 → HTTP, 웹소켓",
    "UDP: 비연결, 빠름, 손실 가능 → DNS, 스트리밍, 게임",
    "소켓 프로그래밍: bind → listen → accept (서버) / connect (클라이언트)",
    "Rust std::net: 블로킹 I/O, tokio::net: 비동기 I/O"
  ],
  comparisons: [
    ["header","TCP","UDP"],
    ["diff","연결 지향 (3-way handshake)","비연결"],
    ["diff","신뢰성 보장 (재전송)","신뢰성 없음 (손실 가능)"],
    ["diff","순서 보장","순서 보장 안 함"],
    ["diff","HTTP, HTTPS, 웹소켓","DNS, DHCP, 스트리밍"]
  ]
},

// ──────────────────────────────────────────────────────
// 32. HTTP 프로토콜
// ──────────────────────────────────────────────────────
{
  title: "HTTP 프로토콜",
  category: "네트워크",
  explanation:
`HTTP: 웹 통신의 기본 프로토콜 (TCP 위에서 동작)

  요청-응답 모델:
    클라이언트 → 요청(Request) → 서버
    서버 → 응답(Response) → 클라이언트

  HTTP 메서드:
    GET: 조회, POST: 생성, PUT: 전체 수정, PATCH: 부분 수정, DELETE: 삭제

  상태 코드:
    2xx: 성공, 3xx: 리다이렉트, 4xx: 클라이언트 에러, 5xx: 서버 에러

  HTTP/1.1 → HTTP/2 (멀티플렉싱) → HTTP/3 (UDP 기반 QUIC)

  NestJS의 @Get(), @Post()가 HTTP를 추상화한 것입니다.`,
  whyItMatters:
`백엔드 개발 = HTTP 프로토콜 이해가 필수입니다.
NestJS가 자동으로 처리해주는 부분(HTTP 파싱, 라우팅)의 원리를 알면
성능 튜닝, 디버깅, API 설계 능력이 크게 향상됩니다.`,
  diagram:
`  HTTP 요청/응답 구조
  ──────────────────────────────────

  요청 (Request):
  ┌─────────────────────────────┐
  │ GET /api/users HTTP/1.1     │ ← 시작 줄 (메서드 + 경로 + 버전)
  │ Host: localhost:3000        │ ← 헤더
  │ Accept: application/json   │
  │ Authorization: Bearer xxx   │
  ├─────────────────────────────┤
  │ (빈 줄)                     │
  ├─────────────────────────────┤
  │ { "name": "Alice" }        │ ← 바디 (POST/PUT만)
  └─────────────────────────────┘

  응답 (Response):
  ┌─────────────────────────────┐
  │ HTTP/1.1 200 OK             │ ← 상태 줄
  │ Content-Type: application/json│
  │ Content-Length: 27          │
  ├─────────────────────────────┤
  │ { "id": 1, "name": "Alice" }│ ← 바디
  └─────────────────────────────┘

  상태 코드:
  200 OK | 201 Created | 204 No Content
  301 Moved | 302 Found
  400 Bad Request | 401 Unauthorized | 404 Not Found
  500 Internal Server Error`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: HTTP = 요청(메서드+경로+헤더+바디) + 응답(상태코드+헤더+바디)
//    NestJS의 @Get() 등이 이 구조를 추상화한 것
// ══════════════════════════════════════════════════════

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 간단한 HTTP 서버 구현 (NestJS의 내부 동작 원리!)
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("HTTP 서버: http://127.0.0.1:{}", port);

    for stream in listener.incoming().take(2) {
        let mut stream = stream.unwrap();
        let mut buf = [0; 4096];
        let n = stream.read(&mut buf).unwrap();
        let request = String::from_utf8_lossy(&buf[..n]);

        // ─── HTTP 요청 파싱 ───
        let first_line = request.lines().next().unwrap_or("");
        println!("요청: {}", first_line);

        // 메서드와 경로 추출
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let (method, path) = (parts.get(0).unwrap_or(&""), parts.get(1).unwrap_or(&"/"));
        println!("  메서드: {}, 경로: {}", method, path);

        // ─── 라우팅 (NestJS의 @Get()이 하는 일!) ───
        let (status, body) = match (*method, *path) {
            ("GET", "/") => ("200 OK", "{\"message\":\"Welcome!\"}"),
            ("GET", "/api/health") => ("200 OK", "{\"status\":\"healthy\"}"),
            (_, path) if path.starts_with("/api/") => {
                ("404 Not Found", "{\"error\":\"Not Found\"}")
            }
            _ => ("404 Not Found", "{\"error\":\"Not Found\"}"),
        };

        // ─── HTTP 응답 생성 ───
        let response = format!(
            "HTTP/1.1 {}\r\n\          ← 상태 줄
             Content-Type: application/json\r\n\  ← 헤더
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",                                         ← 바디
            status,
            body.len(),
            body
        );

        stream.write_all(response.as_bytes()).unwrap();
        println!("  응답: {}", status);
    }
}`,
  keyPoints: [
    "HTTP 요청: 메서드(GET/POST/...) + 경로(/api/...) + 헤더 + 바디",
    "HTTP 응답: 상태 코드(200/404/500) + 헤더 + 바디",
    "NestJS의 @Get(), @Post()가 HTTP 라우팅을 추상화한 것",
    "HTTP/2: 멀티플렉싱, HTTP/3: UDP(QUIC) 기반으로 진화"
  ],
  comparisons: [
    ["header","HTTP/1.1","HTTP/2","HTTP/3"],
    ["diff","텍스트 프로토콜","바이너리 프레임","QUIC (UDP)"],
    ["diff","요청당 1 TCP 연결","멀티플렉싱","연결 설정 0-RTT"],
    ["diff","헤드 오브 라인 블로킹","스트림 병렬","패킷 손실 격리"]
  ]
},

// ──────────────────────────────────────────────────────
// 33. DNS와 로드 밸런싱
// ──────────────────────────────────────────────────────
{
  title: "DNS와 로드 밸런싱",
  category: "네트워크",
  explanation:
`DNS (Domain Name System):
  도메인 이름 → IP 주소 변환
  계층적 구조: 루트 → TLD → 권한 있는 네임 서버
  캐싱: 브라우저 → OS → 라우터 → ISP

  로드 밸런싱:
    여러 서버에 트래픽을 분산
    L4: 전송 계층 (TCP/UDP) 기반
    L7: 응용 계층 (HTTP) 기반

  알고리즘:
    Round Robin, Least Connections, IP Hash
    Weighted, Health Check

  NestJS 앞에 Nginx/ALB가 로드 밸런서 역할을 합니다.`,
  whyItMatters:
`사용자가 api.myservice.com을 입력할 때 일어나는 일을 이해하면
전체 시스템 아키텍처를 그릴 수 있습니다.
트래픽 증가 시 로드 밸런서 도입이 첫 번째 확장 전략입니다.`,
  diagram:
`  DNS 해석 과정
  ──────────────────────────────────

  사용자: "api.example.com"
      │
      ▼
  ┌─ DNS 해석 순서 ─────────────────────────────┐
  │ 1. 브라우저 캐시 → Hit? → 끝                 │
  │ 2. OS 캐시 → Hit? → 끝                       │
  │ 3. ISP DNS (8.8.8.8) → Hit? → 끝            │
  │ 4. 루트 DNS (.) → "com TLD로 가세요"          │
  │ 5. com TLD → "example.com NS로 가세요"        │
  │ 6. example.com NS → "93.184.216.34"           │
  └───────────────────────────────────────────────┘
      │
      ▼
  로드 밸런서 (93.184.216.34)
  ┌─────────────────────────┐
  │ Round Robin:            │
  │ 요청1 → Server A (10.0.0.1) │
  │ 요청2 → Server B (10.0.0.2) │
  │ 요청3 → Server C (10.0.0.3) │
  │ 요청4 → Server A ...    │
  └─────────────────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: DNS = 도메인→IP 변환, 로드밸런서 = 트래픽 분산
//    NestJS 앞에 Nginx/ALB가 로드 밸런서 역할
// ══════════════════════════════════════════════════════

use std::net::UdpSocket;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    // ─── 간단한 DNS 시뮬레이션: 도메인 → IP 매핑 ───
    // 실제 DNS는 UDP 53번 포트에서 동작
    let dns_records: Vec<(&str, &str)> = vec![
        ("localhost", "127.0.0.1"),
        ("example.com", "93.184.216.34"),
        ("google.com", "142.250.80.46"),
        ("api.example.com", "93.184.216.34"),  // 로드밸런서 IP
    ];

    // DNS 조회 (실제로는 HashMap 사용)
    let domain = "api.example.com";
    let ip = dns_records.iter()
        .find(|(d, _)| *d == domain)
        .map(|(_, ip)| *ip)
        .unwrap_or("0.0.0.0");
    println!("DNS: {} → {}", domain, ip);

    // ─── 로드 밸런서 시뮬레이션 ───
    let servers = vec![
        "10.0.0.1:3000",  // NestJS 서버 1
        "10.0.0.2:3000",  // NestJS 서버 2
        "10.0.0.3:3000",  // NestJS 서버 3
    ];

    // Round Robin: 순차 분산
    let counter = Arc::new(AtomicUsize::new(0));

    println!("\nRound Robin 로드 밸런싱:");
    for i in 1..=7 {
        let idx = counter.fetch_add(1, Ordering::SeqCst) % servers.len();
        println!("  요청{} → {}",
            i, servers[idx]);
    }
    // 요청1 → 10.0.0.1, 요청2 → 10.0.0.2, 요청3 → 10.0.0.3
    // 요청4 → 10.0.0.1, 요청5 → 10.0.0.2, 요청6 → 10.0.0.3
    // 요청7 → 10.0.0.1

    // ─── 헬스 체크 시뮬레이션 ───
    let healthy_servers: Vec<&str> = servers.iter()
        .filter(|&&addr| {
            // 실제로는 TcpStream::connect_timeout으로 확인
            !addr.contains("10.0.0.2")  // 서버 2가 다운되었다고 가정
        })
        .copied()
        .collect();
    println!("\n건강한 서버: {:?}", healthy_servers);
    // ["10.0.0.1:3000", "10.0.0.3:3000"]
}`,
  keyPoints: [
    "DNS: 도메인 → IP 변환, 계층적 조회 + 다단계 캐싱",
    "로드 밸런서: Round Robin, Least Connections로 트래픽 분산",
    "L4(TCP/UDP) vs L7(HTTP) 로드 밸런싱",
    "헬스 체크: 장애 서버를 자동으로 제외"
  ],
  comparisons: [
    ["header","L4 로드 밸런서","L7 로드 밸런서"],
    ["diff","TCP/UDP 수준","HTTP 수준"],
    ["diff","IP+포트 기반 분산","URL, 헤더, 쿠키 기반 분산"],
    ["left","빠름","지능적 라우팅"],
    ["left","NLB, HAProxy","ALB, Nginx"]
  ]
},

// ──────────────────────────────────────────────────────
// 34. REST API 설계
// ──────────────────────────────────────────────────────
{
  title: "REST API 설계 원칙",
  category: "네트워크",
  explanation:
`REST (Representational State Transfer):
  자원(Resource)을 URL로 식별, HTTP 메서드로 조작

  6가지 원칙:
    1. 클라이언트-서버 분리
    2. 무상태 (Stateless)
    3. 캐시 가능 (Cacheable)
    4. 통일된 인터페이스 (Uniform Interface)
    5. 계층화 시스템 (Layered System)
    6. 코드 온 디맨드 (선택)

  URL 설계: 자원은 명사, 복수형 사용
    GET /users        → 사용자 목록
    GET /users/1      → 1번 사용자
    POST /users       → 사용자 생성
    PATCH /users/1    → 1번 사용자 부분 수정
    DELETE /users/1   → 1번 사용자 삭제`,
  whyItMatters:
`NestJS 컨트롤러(@Controller, @Get)가 REST를 구현합니다.
잘 설계된 API는 프론트엔드와의 협업 효율, 유지보수성에 직결됩니다.
REST 원칙을 이해하면 NestJS의 설계 철학도 함께 이해됩니다.`,
  diagram:
`  REST API 엔드포인트 설계
  ──────────────────────────────────

  자원: 사용자(User)
  ┌──────┬──────────────┬────────────────────┬──────────────┐
  │메서드│ 경로           │ 동작                │ 상태 코드     │
  ├──────┼──────────────┼────────────────────┼──────────────┤
  │ GET  │ /users       │ 사용자 목록 조회     │ 200          │
  │ GET  │ /users/:id   │ 특정 사용자 조회     │ 200 / 404   │
  │ POST │ /users       │ 새 사용자 생성       │ 201          │
  │ PATCH│ /users/:id   │ 사용자 정보 수정     │ 200 / 404   │
  │DELETE│ /users/:id   │ 사용자 삭제          │ 204 / 404   │
  └──────┴──────────────┴────────────────────┴──────────────┘

  NestJS 매핑:
  @Controller('users')
  class UsersController {
    @Get()       → findAll()
    @Get(':id')  → findOne(@Param() id)
    @Post()      → create(@Body() dto)
    @Patch(':id')→ update(@Param() id, @Body() dto)
    @Delete(':id')→ remove(@Param() id)
  }`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: REST = 자원(URL) + HTTP 메서드(GET/POST/...)
//    NestJS의 @Get(), @Post()가 REST 구현을 추상화
// ══════════════════════════════════════════════════════

use std::collections::HashMap;

// ─── 메모리 기반 간단 REST API 시뮬레이션 ───

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

struct UserService {
    users: HashMap<u32, User>,     // DB 역할
    next_id: u32,                    // Auto Increment
}

impl UserService {
    fn new() -> Self {
        UserService { users: HashMap::new(), next_id: 1 }
    }

    // GET /users → 전체 목록 (200)
    fn find_all(&self) -> Vec<&User> {
        self.users.values().collect()  // SELECT * FROM users
    }

    // GET /users/:id → 단일 조회 (200 or 404)
    fn find_one(&self, id: u32) -> Option<&User> {
        self.users.get(&id)  // SELECT * FROM users WHERE id = ?
    }

    // POST /users → 생성 (201)
    fn create(&mut self, name: String, email: String) -> User {
        let id = self.next_id;
        self.next_id += 1;
        let user = User { id, name, email };
        self.users.insert(id, user.clone());
        user  // INSERT INTO users VALUES (...)
    }

    // DELETE /users/:id → 삭제 (204 or 404)
    fn delete(&mut self, id: u32) -> bool {
        self.users.remove(&id).is_some()  // DELETE FROM users WHERE id = ?
    }
}

fn main() {
    let mut service = UserService::new();

    // POST /users (생성)
    let u1 = service.create("Alice".into(), "alice@mail.com".into());
    let u2 = service.create("Bob".into(), "bob@mail.com".into());
    println!("POST /users → 201 Created: {:?}", u1);

    // GET /users (전체 조회)
    println!("GET /users → 200: {:?}", service.find_all()
        .iter().map(|u| &u.name).collect::<Vec<_>>());

    // GET /users/1 (단일 조회)
    match service.find_one(1) {
        Some(user) => println!("GET /users/1 → 200: {:?}", user),
        None => println!("GET /users/1 → 404 Not Found"),
    }

    // GET /users/99 (없는 리소스)
    match service.find_one(99) {
        Some(user) => println!("GET /users/99 → 200: {:?}", user),
        None => println!("GET /users/99 → 404 Not Found"),  // 출력됨
    }

    // DELETE /users/1
    if service.delete(1) {
        println!("DELETE /users/1 → 204 No Content");
    }
}`,
  keyPoints: [
    "REST: 자원(URL) + HTTP 메서드(GET/POST/PUT/PATCH/DELETE)로 CRUD 구현",
    "URL은 명사 복수형(/users), 동사 금지(/getUsers X)",
    "상태 코드로 결과 표현: 200(성공), 201(생성), 204(삭제), 404(없음)",
    "무상태(Stateless): 각 요청이 독립적, 세션 의존 금지"
  ],
  comparisons: [
    ["header","REST","GraphQL","gRPC"],
    ["diff","리소스 중심(URL)","쿼리 중심(Schema)","서비스 중심(Proto)"],
    ["diff","HTTP 메서드","단일 엔드포인트","HTTP/2 바이너리"],
    ["diff","Over/Under fetching 가능","클라이언트가 필요한 것만","강타입 스키마"],
    ["left","NestJS @Controller","NestJS @Resolver","tonic (Rust)"]
  ]
},

// ╔═══════════════════════════════════════════════════════╗
// ║  데이터베이스 (Database)
// ╚═══════════════════════════════════════════════════════╝

// ──────────────────────────────────────────────────────
// 35. 관계형 데이터베이스와 SQL
// ──────────────────────────────────────────────────────
{
  title: "관계형 데이터베이스와 SQL",
  category: "데이터베이스",
  explanation:
`RDBMS: 테이블(행과 열)로 데이터를 관리, SQL로 조작

  ACID 트랜잭션:
    Atomicity (원자성): 전부 성공 또는 전부 실패
    Consistency (일관성): 제약 조건 위반 불가
    Isolation (격리성): 동시 트랜잭션 간 간섭 없음
    Durability (지속성): 커밋 후 데이터 영구 보존

  정규화: 데이터 중복 제거
    1NF: 원자값만 저장
    2NF: 부분 함수 종속 제거
    3NF: 이행 함수 종속 제거

  인덱스: B-Tree로 검색 속도 향상 (O(n) → O(log n))
  조인: 관계 있는 테이블을 연결`,
  whyItMatters:
`NestJS + TypeORM/Prisma가 RDBMS를 추상화합니다.
SQL과 인덱스를 이해하면 쿼리 성능 문제를 직접 해결할 수 있습니다.
ACID는 금융, 주문 등 비즈니스 로직의 기본 요구사항입니다.`,
  diagram:
`  관계형 데이터베이스 구조
  ──────────────────────────────────

  users 테이블:                posts 테이블:
  ┌────┬───────┬──────────────┐    ┌────┬─────────┬─────────┬─────────┐
  │ id │ name  │ email        │    │ id │ title   │ user_id │ content │
  ├────┼───────┼──────────────┤    ├────┼─────────┼─────────┼─────────┤
  │  1 │ Alice │ alice@ex.com │    │  1 │ Rust    │       1 │ Hello   │
  │  2 │ Bob   │ bob@ex.com   │    │  2 │ NestJS  │       1 │ World   │
  └────┴───────┴──────────────┘    │  3 │ Go     │       2 │ Nice    │
                                   └────┴─────────┴─────────┴─────────┘
    ▲ user_id (외래키)              ▲
    └────────────────────────────────┘

  JOIN 쿼리:
  SELECT u.name, p.title
  FROM users u
  JOIN posts p ON u.id = p.user_id;

  결과: Alice-Rust, Alice-NestJS, Bob-Go

  B-Tree 인덱스:
          [M]
        /     \
      [D]     [T]
     / \      / \\
   [A] [H] [P] [Z]
   검색: O(log n) → 풀스캔 O(n)보다 빠름!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: 테이블 = 행+열, SQL = CRUD, ACID = 트랜잭션 보장
//    인덱스 = B-Tree로 O(log n) 검색
// ══════════════════════════════════════════════════════

use std::collections::HashMap;

// ─── 메모리 RDBMS 시뮬레이션 ───

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Clone)]
struct Post {
    id: u32,
    title: String,
    user_id: u32,  // ★ 외래키 (Foreign Key)
}

struct Database {
    users: HashMap<u32, User>,      // users 테이블
    posts: HashMap<u32, Post>,      // posts 테이블
    user_index: HashMap<String, u32>, // 인덱스 (name → id)
}

impl Database {
    fn new() -> Self {
        Database {
            users: HashMap::new(),
            posts: HashMap::new(),
            user_index: HashMap::new(),
        }
    }

    // INSERT INTO users (name, email) VALUES (...)
    fn insert_user(&mut self, name: &str, email: &str) -> u32 {
        let id = (self.users.len() + 1) as u32;
        self.users.insert(id, User {
            id, name: name.into(), email: email.into()
        });
        self.user_index.insert(name.into(), id);  // 인덱스 갱신
        id
    }

    // INSERT INTO posts (title, user_id) VALUES (...)
    fn insert_post(&mut self, title: &str, user_id: u32) {
        let id = (self.posts.len() + 1) as u32;
        self.posts.insert(id, Post {
            id, title: title.into(), user_id
        });
    }

    // ★ SELECT u.name, p.title FROM users u JOIN posts p ON u.id = p.user_id
    fn join_users_posts(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for post in self.posts.values() {
            if let Some(user) = self.users.get(&post.user_id) {
                result.push((user.name.clone(), post.title.clone()));
            }
        }
        result
    }

    // ★ 인덱스 검색: O(1) vs 풀스캔: O(n)
    fn find_by_name(&self, name: &str) -> Option<&User> {
        // 인덱스로 id를 O(1)에 찾고, users에서 조회
        self.user_index.get(name)
            .and_then(|&id| self.users.get(&id))
    }
}

fn main() {
    let mut db = Database::new();

    // INSERT
    db.insert_user("Alice", "alice@mail.com");  // id=1
    db.insert_user("Bob", "bob@mail.com");       // id=2
    db.insert_post("Rust 입문", 1);   // Alice의 글
    db.insert_post("NestJS 팁", 1);   // Alice의 글
    db.insert_post("Go 기초", 2);     // Bob의 글

    // JOIN 쿼리
    println!("=== JOIN 결과 ===");
    for (name, title) in db.join_users_posts() {
        println!("  {} - {}", name, title);
    }
    // Alice - Rust 입문
    // Alice - NestJS 팁
    // Bob - Go 기초

    // 인덱스 검색
    println!("\n=== 인덱스 검색 ===");
    if let Some(user) = db.find_by_name("Alice") {
        println!("  찾음: {:?}", user);
    }
}`,
  keyPoints: [
    "RDBMS: 테이블(행+열) + SQL + ACID 트랜잭션",
    "정규화: 중복 제거 (1NF→2NF→3NF), 무결성 보장",
    "인덱스: B-Tree로 검색 O(log n), 단점: 쓰기 속도 저하",
    "JOIN: 외래키로 관계 있는 테이블 연결"
  ],
  comparisons: [
    ["header","SQL (RDBMS)","NoSQL"],
    ["diff","스키마 고정","스키마 유연"],
    ["diff","ACID 트랜잭션","BASE ( eventual consistency)"],
    ["diff","JOIN 가능","JOIN 없음 (비정규화)"],
    ["diff","MySQL, PostgreSQL","MongoDB, Redis"]
  ]
},

// ──────────────────────────────────────────────────────
// 36. 트랜잭션과 동시성 제어
// ──────────────────────────────────────────────────────
{
  title: "트랜잭션과 동시성 제어",
  category: "데이터베이스",
  explanation:
`트랜잭션: 하나의 논리적 작업 단위 (전부 성공 또는 전부 실패)

  BEGIN → 작업들 → COMMIT (성공) / ROLLBACK (실패)

  동시성 문제:
    Dirty Read: 커밋 안 된 데이터를 다른 트랜잭션이 읽음
    Non-Repeatable Read: 같은 쿼리 결과가 다름
    Phantom Read: 새 행이 나타나거나 사라짐

  격리 수준 (Isolation Level):
    READ UNCOMMITTED → READ COMMITTED → REPEATABLE READ → SERIALIZABLE
    (낮음: 빠름, 위험)                                    (높음: 느림, 안전)

  낙관적 락 vs 비관적 락:
    낙관적: 버전 번호로 충돌 감지 (대부분 충돌 없을 때)
    비관적: SELECT ... FOR UPDATE로 미리 잠금 (충돌 많을 때)`,
  whyItMatters:
`결제, 재고 관리, 예약 시스템은 트랜잭션 없이 구현할 수 없습니다.
NestJS + TypeORM의 @Transaction이 데코레이터가 내부적으로 하는 일을 이해하면
동시성 버그를 예방할 수 있습니다.`,
  diagram:
`  트랜잭션: 계좌 이체 예제
  ──────────────────────────────────

  A → B에게 10000원 이체:
  BEGIN;
    UPDATE accounts SET balance = balance - 10000 WHERE id = 'A';  -- ①
    -- 여기서 장애 발생하면??
    UPDATE accounts SET balance = balance + 10000 WHERE id = 'B';  -- ②
c  COMMIT;

  ①만 실행되고 ②가 누락되면? → 돈이 사라짐!
  → ROLLBACK으로 ①도 취소해야 함 = 원자성

  격리 수준:
  ┌───────────────────────┬───────┬──────────┬─────────┐
  │ 격리 수준               │ Dirty │ Non-Rep  │ Phantom │
  ├───────────────────────┼───────┼──────────┼─────────┤
  │ READ UNCOMMITTED      │  발생 │   발생    │  발생    │
  │ READ COMMITTED        │  방지 │   발생    │  발생    │
  │ REPEATABLE READ (MySQL)│  방지 │   방지    │  발생*   │
  │ SERIALIZABLE          │  방지 │   방지    │  방지    │
  └───────────────────────┴───────┴──────────┴─────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: BEGIN → 작업 → COMMIT/ROLLBACK = 전부 성공 또는 전부 실패
//    격리 수준으로 동시성 문제 제어
// ══════════════════════════════════════════════════════

use std::collections::HashMap;

// 간단한 계좌 시스템 (트랜잭션 시뮬레이션)
struct Bank {
    accounts: HashMap<String, u64>,  // 계좌명 → 잔액
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: HashMap::new() }
    }

    fn create_account(&mut self, name: &str, balance: u64) {
        self.accounts.insert(name.to_string(), balance);
    }

    // ─── 트랜잭션: 계좌 이체 ───
    // 전부 성공 or 전부 실패 (원자성)
    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        // BEGIN TRANSACTION

        // ① 출금 계좌 확인
        let from_balance = *self.accounts.get(from)
            .ok_or(format!("계좌 없음: {}", from))?;

        // ② 잔액 충분한지 확인
        if from_balance < amount {
            // ★ ROLLBACK: 아무것도 변경하지 않고 에러 반환
            return Err(format!("잔액 부족: {} < {}", from_balance, amount));
        }

        // ③ 실행 (원자적 갱신)
        let to_balance = *self.accounts.get(to)
            .ok_or(format!("계좌 없음: {}", to))?;

        // ★ 실제 갱신 (둘 다 적용되거나 둘 다 안 됨)
        self.accounts.insert(from.to_string(), from_balance - amount);
        self.accounts.insert(to.to_string(), to_balance + amount);

        // COMMIT
        println!("  이체 성공: {} → {} ({}원)", from, to, amount);
        Ok(())
    }

    fn balance(&self, name: &str) -> u64 {
        *self.accounts.get(name).unwrap_or(&0)
    }
}

fn main() {
    let mut bank = Bank::new();
    bank.create_account("Alice", 50000);  // Alice: 50000원
    bank.create_account("Bob", 30000);    // Bob: 30000원

    println!("이전: Alice={} Bob={}", bank.balance("Alice"), bank.balance("Bob"));

    // ─── 정상 이체 (COMMIT) ───
    match bank.transfer("Alice", "Bob", 10000) {
        Ok(()) => println!("  COMMIT"),
        Err(e) => println!("  ROLLBACK: {}", e),
    }
    println!("이후: Alice={} Bob={}", bank.balance("Alice"), bank.balance("Bob"));
    // Alice=40000 Bob=40000

    // ─── 잔액 부족 (ROLLBACK) ───
    match bank.transfer("Alice", "Bob", 999999) {
        Ok(()) => println!("  COMMIT"),
        Err(e) => println!("  ROLLBACK: {}", e),  // 잔액 부족!
    }
    println!("롤백 후: Alice={} Bob={}", bank.balance("Alice"), bank.balance("Bob"));
    // 그대로 Alice=40000 Bob=40000 (변경 없음!)

    // ─── 존재하지 않는 계좌 ───
    match bank.transfer("Alice", "Charlie", 1000) {
        Ok(()) => println!("  COMMIT"),
        Err(e) => println!("  ROLLBACK: {}", e),  // 계좌 없음: Charlie
    }
}`,
  keyPoints: [
    "ACID: 원자성(전부 성공/실패), 일관성, 격리성, 지속성",
    "격리 수준: UNCOMMITTED < COMMITTED < REPEATABLE READ < SERIALIZABLE",
    "낙관적 락: 버전으로 충돌 감지 / 비관적 락: SELECT FOR UPDATE",
    "NestJS의 @Transaction이 BEGIN/COMMIT/ROLLBACK을 자동 처리"
  ],
  comparisons: [
    ["header","낙관적 락","비관적 락"],
    ["diff","버전 번호로 충돌 감지","미리 행 잠금"],
    ["diff","충돌 시 재시도","잠금 대기"],
    ["left","충돌 적은 경우","충돌 많은 경우"],
    ["left","성능 좋음","성능 저하 가능"]
  ]
},

// ──────────────────────────────────────────────────────
// 37. 인덱스와 쿼리 최적화
// ──────────────────────────────────────────────────────
{
  title: "인덱스와 쿼리 최적화",
  category: "데이터베이스",
  explanation:
`인덱스: 특정 컬럼의 검색 속도를 높이는 자료구조 (보통 B-Tree)

  인덱스가 없으면 → 풀 테이블 스캔: O(n)
  인덱스가 있으면 → B-Tree 검색: O(log n)

  인덱스 종류:
    B-Tree: 기본, 범위 검색 가능 (=, >, <, BETWEEN)
    Hash: 동등 검색만 (=) 빠름
    Composite: 여러 컬럼 조합 (컬럼 순서 중요!)
    Unique: 중복 불가 + 인덱스
    Full-text: 텍스트 검색 (LIKE보다 빠름)

  주의:
    너무 많은 인덱스 → INSERT/UPDATE/DELETE 느려짐
    커버링 인덱스: 인덱스만으로 결과 반환 (테이블 접근 불필요)
    EXPLAIN으로 실행 계획 확인 필수!`,
  whyItMatters:
`100만 행에서 WHERE name = 'Alice'를 찾을 때:
  인덱스 없음: 100만 행 순회 (수 초)
  인덱스 있음: ~20번 비교 (수 ms)
실무에서 쿼리 튜닝은 인덱스 설계에서 시작합니다.
NestJS + TypeORM의 @Index() 데코레이터가 인덱스 생성을 관리합니다.`,
  diagram:
`  B-Tree 인덱스 구조
  ──────────────────────────────────

  users 테이블 (id에 인덱스):

                 [50]
               /      \
            [25]      [75]
           /    \     /   \\
         [10]  [35] [60] [90]

  WHERE id = 35 탐색:
  [50] → 35 < 50 → 왼쪽
  [25] → 35 > 25 → 오른쪽
  [35] → 찾음! (3단계, 전체 7개 중)

  100만 행: log₂(1,000,000) ≈ 20단계!

  Composite Index: (category, created_at)
  ┌──────────────────────────────┐
  │ WHERE category='Rust'       │ ✓ 인덱스 사용
  │ WHERE category='Rust'       │ ✓ 인덱스 사용
  │   AND created_at > '2024-01' │
  │ WHERE created_at > '2024-01'│ ✗ 인덱스 못 씀! (왼쪽 접두사 규칙)
  └──────────────────────────────┘

  EXPLAIN 결과:
  ┌──────────────────────────────────────────┐
  │ type: ref  → 인덱스 사용 ✓              │
  │ type: ALL  → 풀 스캔 ✗                  │
  │ rows: 5    → 5행만 검사                 │
  │ rows: 1000000 → 백만 행 검사 (문제!)    │
  └──────────────────────────────────────────┘`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: B-Tree 인덱스 → O(log n) 검색
//    EXPLAIN으로 실행 계획 확인 필수!
//    너무 많은 인덱스 → 쓰기 성능 저하
// ══════════════════════════════════════════════════════

use std::collections::BTreeMap;  // Rust의 B-Tree 구현 (인덱스와 같은 원리!)

#[derive(Debug, Clone)]
struct Article {
    id: u32,
    title: String,
    category: String,
    views: u32,
}

struct ArticleStore {
    articles: BTreeMap<u32, Article>,       // 기본키 인덱스 (id)
    category_index: BTreeMap<String, Vec<u32>>, // 컬럼 인덱스 (category)
}

impl ArticleStore {
    fn new() -> Self {
        ArticleStore {
            articles: BTreeMap::new(),
            category_index: BTreeMap::new(),
        }
    }

    fn insert(&mut self, article: Article) {
        let id = article.id;
        // 카테고리 인덱스 갱신
        self.category_index
            .entry(article.category.clone())
            .or_default()
            .push(id);
        self.articles.insert(id, article);
    }

    // ─── 기본키 검색: O(log n) ───
    // SELECT * FROM articles WHERE id = ?
    fn find_by_id(&self, id: u32) -> Option<&Article> {
        self.articles.get(&id)  // B-Tree 검색: O(log n)
    }

    // ─── 인덱스 검색: O(log n) + O(k) ───
    // SELECT * FROM articles WHERE category = 'Rust'
    fn find_by_category(&self, category: &str) -> Vec<&Article> {
        // 1. 인덱스에서 ID 목록 조회: O(log n)
        match self.category_index.get(category) {
            Some(ids) => ids.iter()
                .filter_map(|id| self.articles.get(id))  // 2. 본문 조회
                .collect(),
            None => vec![],
        }
    }

    // ─── 풀 스캔: O(n) — 인덱스 없는 컬럼 검색 ───
    // SELECT * FROM articles WHERE views > 1000 (인덱스 없음!)
    fn find_by_views_scan(&self, min_views: u32) -> Vec<&Article> {
        self.articles.values()  // ★ 전체 순회: O(n)
            .filter(|a| a.views > min_views)
            .collect()
    }

    // ─── 통계 ───
    fn count(&self) -> usize { self.articles.len() }
}

fn main() {
    let mut store = ArticleStore::new();

    // 데이터 삽입 (INSERT + 인덱스 갱신)
    store.insert(Article { id: 1, title: "Rust 기초".into(), category: "Rust".into(), views: 1500 });
    store.insert(Article { id: 2, title: "NestJS 팁".into(), category: "NestJS".into(), views: 800 });
    store.insert(Article { id: 3, title: "Rust 소유권".into(), category: "Rust".into(), views: 2000 });

    // 기본키 검색 (인덱스 사용)
    println!("ID=1: {:?}", store.find_by_id(1).unwrap().title);

    // 카테고리 검색 (인덱스 사용)
    println!("Rust 글: {:?}",
        store.find_by_category("Rust").iter().map(|a| &a.title).collect::<Vec<_>>());
    // ["Rust 기초", "Rust 소유권"]

    // 풀 스캔 (인덱스 없음 — 느림!)
    println!("조회수 >1000: {:?}",
        store.find_by_views_scan(1000).iter().map(|a| &a.title).collect::<Vec<_>>());
    // ["Rust 기초", "Rust 소유권"] (전체 스캔)

    println!("총 {}개", store.count());
}`,
  keyPoints: [
    "B-Tree 인덱스: O(log n) 검색, 범위 쿼리 가능",
    "Composite Index: (a, b)면 WHERE a만도 가능, WHERE b만은 불가",
    "EXPLAIN으로 type=ref(인덱스) vs type=ALL(풀스캔) 확인",
    "인덱스 비용: 읽기 빨라지지만 쓰기(INSERT/UPDATE) 느려짐"
  ],
  comparisons: [
    ["header","풀 테이블 스캔","인덱스 검색"],
    ["diff","O(n) — 모든 행 확인","O(log n) — B-Tree 탐색"],
    ["diff","쓰기 빠름","쓰기 느림 (인덱스 갱신)"],
    ["diff","데이터 적을 때 유리","데이터 많을 때 필수"],
    ["left","인덱스 없는 컬럼","WHERE, JOIN, ORDER BY 컬럼"]
  ]
},

// ──────────────────────────────────────────────────────
// 38. NoSQL과 캐싱
// ──────────────────────────────────────────────────────
{
  title: "NoSQL과 캐싱 (Redis)",
  category: "데이터베이스",
  explanation:
`NoSQL: 비관계형 데이터베이스
  Document (MongoDB): JSON 형태, 유연한 스키마
  Key-Value (Redis): 초고속, 인메모리
  Column (Cassandra): 대규모 쓰기
  Graph (Neo4j): 관계 중심

  Redis 활용:
    캐싱: 자주 쓰는 데이터를 메모리에 저장 (DB 조회 대신)
    세션 저장소: 로그인 상태 관리
    Rate Limiting: API 호출 횟수 제한
    Pub/Sub: 실시간 알림
   排行榜(Sorted Set): 리더보드

  캐싱 전략:
    Cache Aside: 앱이 캐시 확인 → 없으면 DB → 캐시 저장
    Write Through: 쓸 때 캐시+DB 동시 갱신
    Write Back: 캐시에만 쓰고 나중에 DB에 반영`,
  whyItMatters:
`NestJS에서 Redis는 필수적인 인프라입니다.
캐싱으로 API 응답 시간을 ms 단위로 줄일 수 있습니다.
Rate Limiting, 세션, 실시간 기능에 Redis를 사용합니다.`,
  diagram:
`  Cache Aside 패턴 (가장 일반적)
  ──────────────────────────────────

  클라이언트 → API 서버 (NestJS)
                  │
            Redis 캐시 조회
                  │
         ┌── Hit ──┤── Miss ──┐
         │         │           │
    캐시에서 반환   │     DB에서 조회
    (1ms)          │     (50ms)
                       │
                  캐시에 저장 (TTL 설정)
                       │
                  결과 반환

  Redis 데이터 구조:
  ──────────────────────────────────

  String:  SET user:1:name "Alice"
           GET user:1:name  → "Alice"

  Hash:    HSET user:1 name "Alice" age 30
           HGET user:1 name  → "Alice"

  List:    LPUSH queue:emails "email1"
           RPOP queue:emails  → "email1"

  Sorted Set: ZADD leaderboard 1500 "Alice"
             ZRANGE leaderboard 0 -1 REV  → 랭킹 순서!`,
  code:
`// ══════════════════════════════════════════════════════
// 📌 핵심: Redis = 인메모리 Key-Value 저장소
//    Cache Aside: 캐시 확인 → 없으면 DB → 캐시 저장
//    TTL로 자동 만료 관리
// ══════════════════════════════════════════════════════

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ─── 캐시 엔트리 ───
struct CacheEntry {
    value: String,
    expires_at: Instant,  // ★ TTL (Time To Live)
}

// ─── 간단한 Redis 시뮬레이션 ───
struct RedisCache {
    store: HashMap<String, CacheEntry>,
    hits: u64,
    misses: u64,
}

impl RedisCache {
    fn new() -> Self {
        RedisCache { store: HashMap::new(), hits: 0, misses: 0 }
    }

    // SET key value EX ttl_seconds
    fn set(&mut self, key: &str, value: &str, ttl: Duration) {
        self.store.insert(key.to_string(), CacheEntry {
            value: value.to_string(),
            expires_at: Instant::now() + ttl,
        });
    }

    // GET key (TTL 확인)
    fn get(&mut self, key: &str) -> Option<String> {
        match self.store.get(key) {
            Some(entry) => {
                if Instant::now() > entry.expires_at {
                    self.store.remove(key);  // ★ 만료 → 자동 삭제
                    self.misses += 1;
                    None
                } else {
                    self.hits += 1;
                    Some(entry.value.clone())
                }
            }
            None => { self.misses += 1; None }
        }
    }

    // DEL key
    fn del(&mut self, key: &str) { self.store.remove(key); }

    // 캐시 통계
    fn stats(&self) -> (u64, u64) { (self.hits, self.misses) }
}

// DB 시뮬레이션 (느림!)
fn db_query(id: u32) -> String {
    std::thread::sleep(Duration::from_millis(50));  // DB 조회 50ms 시뮬
    format!("User#{}: Alice, alice@mail.com", id)
}

fn main() {
    let mut cache = RedisCache::new();

    // ─── Cache Aside 패턴 ───
    let cache_key = "user:1:profile";

    // 1차 요청: 캐시 Miss → DB 조회 → 캐시 저장
    let result = match cache.get(cache_key) {
        Some(cached) => {
            println!("  캐시 Hit! ({}ms)", 0);
            cached
        }
        None => {
            println!("  캐시 Miss → DB 조회...");
            let data = db_query(1);  // 50ms 소요
            cache.set(cache_key, &data, Duration::from_secs(300)); // TTL 5분
            data
        }
    };
    println!("  결과: {}", result);

    // 2차 요청: 캐시 Hit! (0ms)
    if let Some(cached) = cache.get(cache_key) {
        println!("  캐시 Hit! → {}", cached.split(',').next().unwrap_or(""));
    }

    // ─── Rate Limiting 시뮬레이션 ───
    // API 호출 횟수를 카운트해서 제한
    let rate_key = "ratelimit:api:192.168.1.1";
    cache.set(rate_key, "0", Duration::from_secs(60));  // 1분 윈도우

    // ─── 통계 ───
    let (hits, misses) = cache.stats();
    println!("\n캐시 통계: hits={}, misses={}", hits, misses);
}`,
  keyPoints: [
    "Redis: 인메모리 Key-Value → 서브 ms 응답, 캐싱/세션/Rate Limiting",
    "Cache Aside: 캐시 확인 → Miss면 DB → 캐시 저장 (TTL 설정)",
    "TTL: 자동 만료 → stale 데이터 방지",
    "캐시 적중률(Hit Rate) = hits / (hits + misses) 모니터링 필수"
  ],
  comparisons: [
    ["header","RDBMS (PostgreSQL)","Redis"],
    ["diff","디스크 기반","인메모리"],
    ["diff","수십 ms","수십 μs (1000배 빠름)"],
    ["diff","ACID 트랜잭션","단순 연산"],
    ["diff","영구 저장","휘발성 (AOF 옵션)"],
    ["left","주 데이터 저장소","캐시, 세션, 큐"]
  ]
}

];
