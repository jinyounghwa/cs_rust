use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Traits (트레이트) — 인터페이스이자 계약",
        category: "추상화",
        explanation: "\
Trait는 '이 타입이 할 수 있는 것'을 정의하는 계약입니다.
NestJS의 Interface와 비슷하지만 더 강력합니다:
  - 기본 구현(default implementation) 제공 가능
  - 표준 라이브러리 트레이트를 구현해서 언어 기능 활용 가능
    (Display, Debug, Iterator, From, PartialOrd, ...)

트레이트 바운드: 함수가 받을 타입에 제약을 줍니다.
  fn print(shape: &impl Drawable) { ... }

impl Trait: 정적 디스패치 (컴파일 타임, 빠름)
dyn Trait: 동적 디스패치 (런타임, 유연함)",
        why_it_matters: "\
상속 없이 다형성을 달성하는 Rust의 핵심 메커니즘입니다.
표준 라이브러리 트레이트를 구현하면 +연산자, {}출력, 비교 등을 쓸 수 있습니다.
Trait를 구현한다 = 그 계약을 이행한다.",
        diagram: "\
  Trait 시스템 구조도
  ──────────────────────────────────

  trait Animal {               ┌──────────────┐
      fn name(&self) -> &str;  │   Animal     │
      fn sound(&self) -> &str; │  (트레이트)   │
      fn describe(&self) { ... }└──┬─────┬────┘
  }                                │     │
                              impl Animal  impl Animal
                                   │     │
  ┌─────────┐              ┌───────┴──┐ ┌┴────────┐
  │  Dog    │              │   Dog    │ │   Cat   │
  │ sound:멍│              │ sound:멍 │ │sound:야옹│
  └─────────┘              └──────────┘ └─────────┘

  정적 vs 동적 디스패치:
  ──────────────────────────────────

  impl Trait (정적):            &dyn Trait (동적):
  fn make_sound(a: &impl Animal) fn make_sound(a: &dyn Animal)

  컴파일 타임에                런타임에
  구체 타입 확정               vtable로 타입 조회

  호출: 직접                   호출: vtable 간접
  속도: ⚡ 빠름                속도: 약간 느림
  유연성: 제한적               유준성: 높음

  표준 트레이트 구현 효과:
  ──────────────────────────────────
  Display  → println!(\"{}\", x) 사용 가능
  Debug    → println!(\"{:?}\", x) 사용 가능
  Clone    → x.clone() 가능
  Iterator → for item in x 가능
  From     → T::from(val) 가능",
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
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "야옹" }
    fn describe(&self) -> String {
        format!("고양이 {} (도도함)", self.name())
    }
}

// 표준 Display 트레이트 구현
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

// impl Trait 문법 (정적 디스패치)
fn make_sound(animal: &impl Animal) {
    println!("{}", animal.describe());
}

// 동적 디스패치
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
    println!("{}", dog);  // Display 트레이트

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
        comparisons: &[
            "header|impl Trait (정적)|&dyn Trait (동적)",
            "diff|컴파일 타임 결정|런타임에 결정",
            "diff|단형화 — 타입별 코드 생성|vtable 간접 호출",
            "win|오버헤드 없음|유연한 컬렉션",
            "diff|fn(x: &impl Trait)|fn(x: &dyn Trait)",
        ],
    }
}
