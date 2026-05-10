use colored::*;
use rand::Rng;
use std::env;
use std::process::Command;

struct CSTopic {
    title: &'static str,
    explanation: &'static str,
    code: &'static str,
    key_points: [&'static str; 3],
}

const TOPICS: &[CSTopic] = &[
    CSTopic {
        title: "Ownership & Borrowing",
        explanation: "Rust의 메모리 관리의 핵심. 값의 소유권은 한 번에 하나의 바인딩만 가능하며, \
                      빌림(&)을 통해 임시 접근을 허용한다. NestJS의 객체 관리와 달리 GC 없이 컴파일 타임에 메모리 안정성을 보장.",
        code: r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // 소유권 이동
    // println!("{}", s1);  // 에러! s1은 더 이상 유효하지 않음

    let s3 = String::from("world");
    let s4 = &s3;  // 빌림, s3의 소유권은 유지
    println!("{}, {}", s4, s3);  // 둘 다 유효
}
"#,
        key_points: [
            "값은 스코프를 벗어나면 drop() 호출",
            "빌림(&T, &mut T)으로 임시 접근 가능",
            "컴파일러가 메모리 안정성 검사",
        ],
    },
    CSTopic {
        title: "Result<T, E> & Error Handling",
        explanation: "try/catch 대신 Result 타입으로 에러를 명시적으로 처리. \
                      성공(Ok) 또는 실패(Err)를 타입 레벨에서 표현하므로 에러 처리를 빼먹을 수 없다.",
        code: r#"fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // ? 연산자로 간결하게: divide(10, 0)?
}
"#,
        key_points: [
            "Exception 없음, Result로 명시적 에러 전파",
            "? 연산자로 보일러플레이트 감소",
            "타입 시스템이 에러 처리 강제",
        ],
    },
    CSTopic {
        title: "Traits (인터페이스)",
        explanation: "추상화 메커니즘. 여러 타입이 같은 동작(메서드)을 구현하도록 강제한다. \
                      NestJS 인터페이스와 비슷하지만 구현 메서드(default impl)도 제공 가능.",
        code: r#"trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("Drawing circle"); }
    fn area(&self) -> f64 { 3.14 * self.radius * self.radius }
}

impl Drawable for Square {
    fn draw(&self) { println!("Drawing square"); }
    fn area(&self) -> f64 { self.side * self.side }
}

fn print_shape(shape: &dyn Drawable) {
    shape.draw();
    println!("Area: {}", shape.area());
}
"#,
        key_points: [
            "구조체 + 트레이트 = 조합 가능한 추상화",
            "dyn Trait로 동적 디스패치 가능",
            "구현 생략 시 컴파일 에러",
        ],
    },
    CSTopic {
        title: "Pattern Matching",
        explanation: "복잡한 if/else 대신 match로 패턴을 분해. 구조를 동시에 검사하고 분해하므로 \
                      버그를 줄이고 의도를 명확하게 표현.",
        code: r#"enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) if r == 255 => println!("Red alert!"),
        Message::ChangeColor(_, _, _) => println!("Color changed"),
    }
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    handle_message(msg);
}
"#,
        key_points: [
            "Enum 분해로 가능한 모든 경우 처리(exhaustiveness check)",
            "guard(if 조건)로 세밀한 패턴 정의",
            "구조와 값을 동시에 추출",
        ],
    },
    CSTopic {
        title: "Closures & Function Traits",
        explanation: "함수처럼 동작하는 익명 함수. Fn, FnMut, FnOnce 세 가지 트레이트로 분류되어 \
                      캡처 방식(빌림/소유)을 명시적으로 관리.",
        code: r#"fn main() {
    let x = 5;
    let add_x = |y| x + y;  // Fn: &self로 캡처
    println!("{}", add_x(3));  // 8

    let mut count = 0;
    let mut inc = || count += 1;  // FnMut: &mut self로 캡처
    inc();
    inc();
    println!("Count: {}", count);  // 2

    let s = String::from("hello");
    let take_s = || s;  // FnOnce: self로 소유권 취득
    take_s();
    // println!("{}", s);  // 에러! s는 이미 소유권 상실
}
"#,
        key_points: [
            "클로저가 환경을 자동으로 캡처",
            "Fn vs FnMut vs FnOnce으로 소유권 추적",
            "고차 함수와 iterator 조합으로 표현력 증대",
        ],
    },
    CSTopic {
        title: "Generics (제네릭)",
        explanation: "타입 매개변수로 다양한 타입을 지원하되, 컴파일 타임에 구체적 타입으로 \
                      단형화(monomorphization)되므로 런타임 오버헤드가 없다.",
        code: r#"struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

impl<T: std::fmt::Display> Pair<T, T> {
    fn print(&self) {
        println!("{}, {}", self.first, self.second);
    }
}

fn main() {
    let pair = Pair::new(5, "hello");
    let nums = Pair::new(3, 4);
    nums.print();  // Display 구현 타입만 호출 가능
}
"#,
        key_points: [
            "Trait bounds로 제네릭 타입 제약",
            "단형화: 컴파일 타임에 구체 타입 생성",
            "런타임 오버헤드 없음 (vs. 동적 디스패치)",
        ],
    },
    CSTopic {
        title: "Lifetime ('a)",
        explanation: "참조의 유효 범위를 명시. 컴파일러가 댕글링 포인터를 방지하도록 강제한다. \
                      런타임 검사 없이 메모리 안정성 보장.",
        code: r#"fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct User<'a> {
    name: &'a str,
    age: u32,
}

fn main() {
    let s1 = String::from("long");
    let s2 = "short";
    let result = longest(s1.as_str(), s2);
    println!("{}", result);

    let user = User { name: "Alice", age: 30 };
    println!("{}, {}", user.name, user.age);
}
"#,
        key_points: [
            "라이프타임: 컴파일러를 위한 주석",
            "빌린 데이터의 유효 범위 명시",
            "댕글링 참조 컴파일 타임에 방지",
        ],
    },
    CSTopic {
        title: "Channels (메시지 패싱)",
        explanation: "스레드 간 안전한 통신. 채널(sender, receiver)을 통해 소유권을 이전하므로 \
                      메모리 안정성이 보장된다.",
        code: r#"use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
"#,
        key_points: [
            "mpsc: 다중 생산자, 단일 소비자",
            "send()로 소유권 이전, 데이터 경쟁 불가능",
            "스레드 안전성을 타입 시스템으로 보장",
        ],
    },
    CSTopic {
        title: "Structs & Enums",
        explanation: "데이터 그룹화와 값의 표현. Struct는 명명된 필드들의 조합, Enum은 여러 변형 중 하나를 선택. \
                      NestJS 클래스보다 더 간결하고, 타입 안정성이 우수.",
        code: r#"#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

enum Status {
    Active,
    Inactive,
    Banned { reason: String },
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{:?}", user);

    let status = Status::Banned {
        reason: String::from("spam"),
    };
    match status {
        Status::Active => println!("User is active"),
        Status::Inactive => println!("User is inactive"),
        Status::Banned { reason } => println!("Banned: {}", reason),
    }
}
"#,
        key_points: [
            "Struct: 캐리 단위 데이터 조직",
            "Enum: 합 타입(sum type)으로 값의 선택 표현",
            "#[derive(Debug)]로 자동 구현",
        ],
    },
    CSTopic {
        title: "Smart Pointers (Box, Rc)",
        explanation: "Stack 대신 Heap에 데이터를 할당하거나, 다중 소유권을 관리. Box<T>는 단일 소유, \
                      Rc<T>는 공유 소유권. 순환 참조를 피하려면 Weak<T> 사용.",
        code: r#"use std::rc::Rc;

fn main() {
    // Box: 단일 소유권, Heap 할당
    let b = Box::new(5);
    println!("Box: {}", b);

    // Rc: 참조 카운팅, 다중 소유권
    let a = Rc::new(42);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("Count: {}", Rc::strong_count(&a));  // 3
    println!("a: {}, b: {}, c: {}", a, b, c);
}
"#,
        key_points: [
            "Box<T>: Heap 할당, 단일 소유권",
            "Rc<T>: 참조 카운팅으로 다중 소유권",
            "순환 참조 발생 시 memory leak 가능 (Weak<T> 사용)",
        ],
    },
    CSTopic {
        title: "Iterators (고급)",
        explanation: "지연 계산(lazy evaluation)을 활용한 효율적인 데이터 처리. map, filter, fold 등의 \
                      함수형 메서드로 명확한 의도 표현. 성능 최적화.",
        code: r#"fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    // map + filter: 각 요소에 2를 곱하고, 5 이상만 필터링
    let result: Vec<_> = nums.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .collect();
    println!("Result: {:?}", result);  // [6, 8, 10]

    // fold: 누적 계산
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);  // 15

    // for_each: 부작용 발생
    nums.iter().for_each(|x| print!("{} ", x));
}
"#,
        key_points: [
            "Iterator는 lazy: 필요할 때까지 계산 지연",
            "map/filter/fold로 함수형 스타일",
            "collect()로 구체 타입 생성",
        ],
    },
    CSTopic {
        title: "Modules & Crates",
        explanation: "코드 조직화의 핵심. Crate는 바이너리 또는 라이브러리, Module은 내부 계층 구조. \
                      pub으로 공개 범위 제어. 패키지 관리는 Cargo가 담당.",
        code: r#"// lib.rs 또는 main.rs 상단
pub mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

mod internal {
    pub fn secret() {
        println!("This is internal");
    }
}

fn main() {
    println!("2 + 3 = {}", math::add(2, 3));
    println!("4 * 5 = {}", math::multiply(4, 5));

    internal::secret();
}
"#,
        key_points: [
            "pub mod: 공개 모듈, pub fn: 공개 함수",
            "비공개 요소는 mod로 시작하거나 pub 생략",
            "Crate Root: lib.rs (라이브러리) 또는 main.rs (바이너리)",
        ],
    },
    CSTopic {
        title: "From & Into (타입 변환)",
        explanation: "타입 간 변환을 안전하고 명시적으로 처리. From trait을 구현하면 Into는 자동 구현. \
                      as (타입 캐스팅)과 달리 타입 검사와 검증을 강제.",
        code: r#"#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

fn main() {
    // From 사용
    let p1: Point = (10, 20).into();
    println!("{:?}", p1);  // Point { x: 10, y: 20 }

    // 명시적 변환
    let p2 = Point::from((5, 15));
    println!("{:?}", p2);

    // Into는 자동으로 구현됨
    let tuple = (30, 40);
    let p3: Point = tuple.into();
    println!("{:?}", p3);
}
"#,
        key_points: [
            "From trait: T -> Self 변환, 실패할 수 없음",
            "TryFrom: 실패 가능한 변환 (Result 반환)",
            "Into는 From 구현 시 자동 생성 (무료 변환)",
        ],
    },
    CSTopic {
        title: "Macros (매크로)",
        explanation: "컴파일 타임에 코드를 생성. vec!, println!, assert! 등이 매크로. \
                      반복 코드를 줄이고, DSL(Domain Specific Language)을 만들 수 있다.",
        code: r#"macro_rules! say_hello {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

macro_rules! max {
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
}

fn main() {
    say_hello!("Alice");
    say_hello!("Bob");

    let result = max!(10, 20);
    println!("Max: {}", result);

    // 내장 매크로들
    let v = vec![1, 2, 3, 4, 5];
    println!("Vec: {:?}", v);

    assert_eq!(2 + 2, 4);
}
"#,
        key_points: [
            "macro_rules!: 패턴 기반 매크로 정의",
            "($name:expr) 등으로 인자 타입 지정",
            "=> 이후 생성될 코드 작성",
        ],
    },
    CSTopic {
        title: "Option<T> (None 처리)",
        explanation: "값이 없을 수 있는 상황을 타입으로 표현. null 포인터 대신 Option<T>로 \
                      None인 경우를 강제로 처리. NestJS의 optional 필드와 다르게 컴파일 타임 검사.",
        code: r#"fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    let user = find_user(1);

    // match로 처리
    match user {
        Some(name) => println!("Found: {}", name),
        None => println!("User not found"),
    }

    // if let로 간결하게
    if let Some(name) = find_user(1) {
        println!("Name: {}", name);
    }

    // unwrap: None이면 panic!
    // let name = find_user(1).unwrap();

    // unwrap_or: 기본값 제공
    let name = find_user(999).unwrap_or_default();
    println!("Default: {}", name);
}
"#,
        key_points: [
            "Option<T>: Some(value) 또는 None",
            "match, if let, unwrap_or 등으로 안전 처리",
            "None 처리를 빼먹을 수 없음 (컴파일 검사)",
        ],
    },
    CSTopic {
        title: "Testing (단위 테스트)",
        explanation: "코드와 함께 테스트를 작성. #[test] 속성으로 테스트 함수 마킹. \
                      cargo test로 실행. TDD와 리팩토링을 촉진.",
        code: r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2).unwrap(), 5);
        assert!(divide(10, 0).is_err());
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This should panic!");
    }
}
"#,
        key_points: [
            "#[cfg(test)]로 테스트 모듈 격리",
            "#[test]로 테스트 함수 표시",
            "assert!, assert_eq!, assert_ne! 등의 매크로",
        ],
    },
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_random_topic();
    } else {
        match args[1].as_str() {
            "run" => run_last_topic(),
            "list" => list_topics(),
            _ => show_random_topic(),
        }
    }
}

fn show_random_topic() {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..TOPICS.len());
    let topic = &TOPICS[idx];

    print_topic(topic, idx + 1);
}

fn run_last_topic() {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..TOPICS.len());
    let topic = &TOPICS[idx];

    print_topic(topic, idx + 1);
    println!("\n{}", "=".repeat(60).cyan());
    println!("{}", "Running code example...".bright_yellow());
    println!("{}", "=".repeat(60).cyan());

    execute_code(topic.code);
}

fn list_topics() {
    println!("{}", "Available CS Topics:".bright_cyan().bold());
    for (i, topic) in TOPICS.iter().enumerate() {
        println!("  {}. {}", i + 1, topic.title.cyan());
    }
}

fn print_topic(topic: &CSTopic, idx: usize) {
    println!("{}", format!("\n[{}/{}] {}", idx, TOPICS.len(), topic.title).bright_blue().bold());
    println!("{}", "-".repeat(60).bright_black());
    println!();
    println!("{}", topic.explanation);

    println!();
    println!("{}", "Code Example:".bright_green().bold());
    println!("{}", "-".repeat(60).bright_black());
    println!("{}", topic.code.bright_black());
    println!("{}", "-".repeat(60).bright_black());

    println!();
    println!("{}", "Key Points:".bright_yellow().bold());
    for point in &topic.key_points {
        println!("  {} {}", "•".yellow(), point);
    }
}

fn execute_code(code: &str) {
    let mut child = Command::new("rustc")
        .args(&["--edition", "2021", "-", "-o", "/tmp/cs_bite_out"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustc");

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(code.as_bytes());
    }

    let output = child.wait_with_output().expect("Failed to wait for rustc");

    if !output.status.success() {
        println!("{}", "Compilation failed:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr).bright_red());
        return;
    }

    let output = Command::new("/tmp/cs_bite_out")
        .output()
        .expect("Failed to execute code");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{}", "Execution failed:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr).bright_red());
    }
}
