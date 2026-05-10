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
