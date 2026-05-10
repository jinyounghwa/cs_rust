use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Generics (제네릭) — 타입 매개변수",
        category: "추상화",
        explanation: "\
같은 로직을 다양한 타입에 재사용할 때 제네릭을 씁니다.
컴파일 타임에 구체적인 타입으로 '단형화(monomorphization)'되어
런타임 오버헤드가 없습니다.

  fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

T는 타입 매개변수, PartialOrd는 트레이트 바운드입니다.
'T는 반드시 PartialOrd를 구현해야 한다'는 의미입니다.

where 구문으로 복잡한 바운드를 정리할 수 있습니다.
Vec<T>, Option<T>, Result<T,E> 모두 제네릭으로 구현됩니다.",
        why_it_matters: "\
TypeScript의 제네릭과 개념은 같지만, 런타임 동작이 다릅니다.
TypeScript: 타입 정보가 런타임에 사라짐 (type erasure)
Rust: 컴파일 타임에 구체 타입으로 확정 (단형화) → 런타임 비용 없음",
        diagram: "\
  단형화 (Monomorphization) 시각화
  ──────────────────────────────────

  fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

  컴파일 타임에 이렇게 확장:

  largest(&[34, 50, 25, 100])
       ↓
  fn largest_i32(list: &[i32]) -> &i32 { ... }

  largest(&['y', 'm', 'a', 'q'])
       ↓
  fn largest_char(list: &[char]) -> &char { ... }

  ┌──────────────────────────────────────┐
  │ 소스코드:                             │
  │   fn largest<T>(...)                  │  ← 하나의 함수
  │                                      │
  │ 컴파일 후:                             │
  │   fn largest_i32(...)                 │  ← i32용
  │   fn largest_char(...)               │  ← char용
  │                                      │
  │ 런타임: 오버헤드 0!                   │
  └──────────────────────────────────────┘

  트레이트 바운드가 보장하는 것:
  ──────────────────────────────────

  T: PartialOrd  → 비교 연산(>, <) 가능
  T: Display     → 출력({}) 가능
  T: Clone       → 복제 가능
  T: Debug       → 디버그 출력 가능",
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
        comparisons: &[
            "header|Rust 제네릭|TypeScript 제네릭",
            "diff|단형화 (타입별 코드 생성)|type erasure (런타임에 삭제)",
            "diff|런타임 오버헤드 0|런타임에 타입 정보 없음",
            "diff|트레이트 바운드로 제약|extends / keyof 제약",
            "win|컴파일 타임 완전 체크|런타임 에러 가능",
        ],
    }
}
