use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Enum & Pattern Matching (열거형과 패턴 매칭)",
        category: "핵심",
        explanation: "\
Rust의 Enum은 단순한 값 목록이 아닙니다. 각 변형(variant)이 데이터를 가질 수 있는
'대수적 데이터 타입(Algebraic Data Type)'입니다.

  enum Shape {
    Circle(f64),             // 이름 없는 필드 (Tuple형)
    Rectangle(f64, f64),    // 이름 없는 필드
    Triangle { base: f64, height: f64 }, // 이름 있는 필드 (Struct형)
  }

match는 모든 경우를 반드시 처리해야 합니다 (exhaustiveness check).
처리 안 하면 컴파일 에러! → 버그를 설계 단계에서 잡습니다.
Option<T>와 Result<T, E> 자체가 Enum으로 구현되어 있습니다.",
        why_it_matters: "\
TypeScript의 union type + 구조분해를 합쳐놓은 것과 비슷합니다.
패턴 매칭은 단순 switch-case가 아니라 값, 구조, 타입을 동시에 분해합니다.
컴파일러가 빠진 케이스를 잡아주므로 버그가 줄어듭니다.",
        diagram: "\
  Enum의 메모리 레이아웃 (태그 + 페이로드)
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
   Circle=0, Rect=1, Tri=2

  match 패턴 분해:
  ──────────────────────────────────
  match shape {
  ┌───────────────────────────────────────┐
  │ Shape::Circle(r)         => PI*r*r   │ ← r 바인딩
  │ Shape::Rectangle(w, h)   => w * h    │ ← w, h 바인딩
  │ Shape::Triangle { base, height }      │ ← 이름 필드
  │                     => 0.5*b*h       │
  │ Shape::Triangle { .. }    => ...     │ ← .. 나머지 무시
  └───────────────────────────────────────┘
   모든 variant를 처리해야 함 (빠지면 컴파일 에러!)

  패턴 매칭의 힘:
  ┌──────────────────────────────────────┐
  │ 리터럴  │ 42 => ...                  │
  │ 범위    │ 1..=9 => ...               │
  │ 변수    │ n if n < 0 => ...          │
  │ 와일드  │ _ => ...                    │
  │ 구조분해│ Some(x) => ...             │
  └──────────────────────────────────────┘",
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
            Shape::Triangle { .. } => "삼각형",
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
        1..=9 => println!("한 자리"),
        _ => println!("두 자리 이상"),
    }
}
"#,
        key_points: &[
            "Enum variant는 데이터를 가질 수 있음 (Tuple형, Struct형)",
            "match는 모든 경우 처리 강제 (컴파일 타임 exhaustiveness check)",
            "if let: 한 패턴만 처리할 때 match보다 간결",
            "1..=9: 범위 패턴 / _ : 와일드카드 / guard: if 조건 추가",
        ],
        comparisons: &[
            "header|Rust Enum|TypeScript Union",
            "diff|태그+페이로드 메모리|런타임에만 구분",
            "win|match 완전성 강제|switch break 누락 가능",
            "left|variant가 데이터 보유|discriminant만",
            "left|if let 간결 패턴|typeof 체크 필요",
            "win|컴파일 타임 분해|런타임 타입 가드",
        ],
    }
}
