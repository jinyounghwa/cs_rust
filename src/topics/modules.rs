use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Module System (모듈, use, pub)",
        category: "실전",
        explanation: "\
Rust의 모듈 시스템은 코드를 논리적으로 조직화하는 방법입니다.

  mod: 모듈 선언 (파일 또는 블록)
  pub: 가시성 제어 (public)
  use: 경로 단축 (import)
  crate: 현재 크레이트의 루트
  super: 부모 모듈
  self: 현재 모듈

파일 시스템과 모듈은 자동 매핑됩니다:
  src/main.rs      → crate 루트 (실행 파일)
  src/lib.rs       → crate 루트 (라이브러리)
  src/network.rs   → mod network;
  src/server/mod.rs → mod server; (디렉토리 모듈)

모든 것은 기본적으로 private입니다. pub을 붙여야 외부에서 접근 가능합니다.",
        why_it_matters: "\
NestJS에서 폴더/파일로 코드를 조직화하는 것과 같습니다.
하지만 Rust는 가시성(visibility)을 명시적으로 제어합니다.
라이브러리 설계 시 pub으로 공개 API를, 나머지는 private으로 유지합니다.",
        diagram: "\
  모듈 트리 구조
  ──────────────────────────────────

  crate (src/lib.rs 또는 src/main.rs)
  ├── network (src/network.rs)
  │   ├── server (src/server.rs)
  │   │   └── connect()  [pub]
  │   └── client (src/client.rs)
  │       └── connect()  [pub]
  └── utils (src/utils.rs)
      └── helper()       [pub]

  파일 ↔ 모듈 자동 매핑:
  ──────────────────────────────────
  src/
  ├── main.rs           mod network; mod utils;
  ├── network.rs        pub mod server; pub mod client;
  ├── server.rs         pub fn connect() { ... }
  ├── client.rs         pub fn connect() { ... }
  └── utils.rs          pub fn helper() { ... }

  가시성 규칙:
  ──────────────────────────────────
  fn item()         → private (같은 모듈만)
  pub fn item()     → public (어디서나)
  pub(crate) fn ... → 같은 크레이트 내에서만
  pub(super) fn ... → 부모 모듈에서만

  use 경로 단축:
  ──────────────────────────────────
  // 전체 경로
  crate::network::server::connect();

  // use로 단축
  use crate::network::server::connect;
  connect();  // 간결!",
        code: r#"// 가상 모듈 구조 시연 (단일 파일 안에서)

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Waitlist에 추가");
        }
        pub fn seat_at_table() {
            println!("자리 안내");
        }
    }

    mod serving {
        pub fn take_order() {
            println!("주문 받기");
        }
        fn serve_order() {
            println!("서빙");  // private
        }
    }
}

// use로 경로 단축
use front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,     // pub 필드
        seasonal_fruit: String, // private 필드
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }
}

fn main() {
    // 전체 경로로 접근
    front_of_house::hosting::add_to_waitlist();

    // use로 단축한 경로
    hosting::seat_at_table();

    // pub 구조체 사용 (pub 필드만 접근 가능)
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    println!("Toast: {}", meal.toast);
    // meal.seasonal_fruit; // 에러! private 필드

    // use as로 별칭
    use front_of_house::hosting as host;
    host::add_to_waitlist();
}
"#,
        key_points: &[
            "mod: 모듈 선언 / pub: 공개 가시성 / use: 경로 단축",
            "기본적으로 모든 것은 private — pub 필요",
            "crate:: 최상위 / super:: 부모 / self:: 현재",
            "파일 시스템이 곧 모듈 트리 (src/network.rs = mod network)",
        ],
        comparisons: &[
            "header|Rust 모듈|NestJS/TS 모듈",
            "diff|mod + pub 선언|import/export",
            "diff|기본 private|기본 접근 가능",
            "left|crate:: 경로|상대/절대 경로",
            "diff|컴파일 타임 검사|런타임에만 에러",
        ],
    }
}
