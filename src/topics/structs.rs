use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Struct & impl (데이터와 메서드)",
        category: "핵심",
        explanation: "\
Struct는 연관된 데이터를 묶는 사용자 정의 타입입니다.
impl 블록에서 해당 타입의 메서드를 정의합니다.

  연관 함수 (associated function): self 없음 → 타입으로 호출 (User::new())
  메서드 (method): &self 또는 &mut self → 인스턴스로 호출 (user.name())

NestJS 클래스와 비슷하지만, 상속이 없습니다.
대신 트레이트(Trait)로 공통 동작을 정의합니다.
구조체 업데이트 문법(..other)으로 일부 필드만 변경 가능합니다.",
        why_it_matters: "\
클래스가 없는데 어떻게 OOP를 하냐고? Rust의 답: Struct + Trait 조합입니다.
상속 없음 → 합성(Composition)을 권장 → 더 유연하고 버그가 적습니다.
#[derive(Debug, Clone)]으로 자주 쓰는 트레이트를 자동 구현할 수 있습니다.",
        diagram: "\
  Struct 메모리 레이아웃
  ──────────────────────────────────

  struct User {
      name: String,    // 24바이트 (ptr+len+cap)
      age: u32,        // 4바이트
      active: bool,    // 1바이트
  }

  메모리 상 배치:
  ┌─────────────────────┬──────┬───┬───┐
  │ name (String)       │ age  │act│pad│
  │ 24 bytes            │ 4B   │1B │3B │
  └─────────────────────┴──────┴───┴───┘

  self의 3가지 형태:
  ──────────────────────────────────

  &self           &mut self          self
  (불변 참조)      (가변 참조)        (소유권)
  읽기만 가능      읽기/쓰기 가능     소비됨 (값 이동)

  fn name(&self)   fn deactivate    fn into_name
  -> &str          (&mut self)      (self) -> String
                   self.active=false

  구조체 업데이트 문법:
  ──────────────────────────────────
  let user2 = User {
      name: String::from(\"Bob\"),  ← 변경
      ..user                        ← 나머지는 user에서 복사
  };

  ┌──────────┐     ┌──────────┐
  │ user     │     │ user2    │
  │ Alice,30 │ ──► │ Bob, 30  │
  │ true     │     │ true     │
  └──────────┘     └──────────┘
   (String 이동으로    (새 name 소유)
    user.name 무효)",
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
        comparisons: &[
            "header|Rust Struct|TS/JS Class",
            "diff|상속 없음|extends 가능",
            "diff|impl 블록에 메서드|클래스 안에 메서드",
            "diff|derive로 자동 구현|수동 구현",
            "win|컴파일 타임 필드 체크|런타임才 체크",
            "left|Self 생성자 패턴|constructor",
        ],
    }
}
