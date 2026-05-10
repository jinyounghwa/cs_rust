use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Testing (테스트) — 안전한 리팩토링",
        category: "실전",
        explanation: "\
Rust는 테스트를 언어 차원에서 지원합니다. 외부 프레임워크 불필요!

  #[test]: 테스트 함수 표시
  assert!, assert_eq!, assert_ne!: 단언 매크로
  #[should_panic]: 패닉이 발생해야 성공
  #[ignore]: 테스트 스킵
  Result<(), String> 반환 테스트도 가능

실행: cargo test

단위 테스트: 같은 파일에 #[cfg(test)] mod tests { } 로 작성
통합 테스트: tests/ 디렉토리에 별도 파일로 작성",
        why_it_matters: "\
Jest/Mocha 같은 외부 도구 없이 내장 테스트 프레임워크를 사용합니다.
cargo test 한 명령으로 모든 테스트를 병렬 실행합니다.
테스트 모듈은 #[cfg(test)]로 배포 빌드에 포함되지 않습니다.",
        diagram: "\
  테스트 구조와 실행 흐름
  ──────────────────────────────────

  프로젝트 구조:
  src/
  ├── lib.rs           ← 단위 테스트 포함 가능
  ├── main.rs
  └── utils.rs         ← 각 파일에 tests 모듈
  tests/
  ├── integration.rs   ← 통합 테스트
  └── common/

  단위 테스트:
  ──────────────────────────────────
  #[cfg(test)]        ← 배포 빌드에 제외
  mod tests {
      use super::*;   ← 부모 모듈 전체 import

      #[test]         ← 테스트 함수 표시
      fn test_add() {
          assert_eq!(add(2, 3), 5);   ← 성공!
      }

      #[test]
      #[should_panic] ← 패닉 나야 성공
      fn test_panic() {
          panic!(\"oops\");
      }
  }

  cargo test 실행:
  ──────────────────────────────────

  running 3 tests
  test tests::test_add     ... ok
  test tests::test_panic   ... ok
  test tests::test_failing ... FAILED

  test result: FAILED. 2 passed; 1 failed",
        code: r#"// 테스트 대상 함수들
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 단위 테스트 모듈
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_divide_some() {
        assert_eq!(divide(10, 2), Some(5));
    }

    #[test]
    fn test_divide_none() {
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_greet() {
        let result = greet("Rust");
        assert!(result.contains("Rust"));
        assert!(result.starts_with("Hello"));
    }

    #[test]
    //#[should_panic(expected = \"division by zero\")]
    fn test_assert_ne() {
        assert_ne!(add(1, 1), 3);  // 1+1 != 3 → 성공
    }

    // Result를 반환하는 테스트 (? 사용 가능)
    #[test]
    fn test_result() -> Result<(), String> {
        let val = divide(10, 2).ok_or(\"division failed\")?;
        assert_eq!(val, 5);
        Ok(())
    }
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 2) = {:?}", divide(10, 2));
    println!("{}", greet("Rust"));
    println!(\"\\ncargo test 로 테스트를 실행하세요!\");
}
"#,
        key_points: &[
            "#[test]: 테스트 함수 표시 / cargo test로 실행",
            "assert_eq! == 비교 / assert_ne! != 비교 / assert! 참 거짓",
            "#[should_panic]: 패닉 발생을 기대하는 테스트",
            "#[cfg(test)]: 배포 빌드에서 테스트 코드 제외",
        ],
        comparisons: &[
            "header|Rust 내장 테스트|Jest (Node.js)",
            "diff|#pragma test 어노테이션|describe/it 함수",
            "diff|assert_eq! 매크로|expect().toBe()",
            "left|cargo test로 실행|npm test로 실행",
            "win|외부 의존성 없음|jest 설치 필요",
            "win|병렬 실행 기본|설정 필요",
        ],
    }
}
