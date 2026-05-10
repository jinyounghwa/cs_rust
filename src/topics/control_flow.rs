use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
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

Rust에서는 for 루프에서 인덱스보다 반복자를 권장합니다.
반복자 최적화로 C의 for 루프와 같은 성능을 냅니다.",
        why_it_matters: "\
loop에서 break value로 반환값을 뽑을 수 있습니다 — 재시도 로직에서 유용합니다.
for in을 선호하는 이유: 배열 범위 초과(off-by-one) 버그가 없고,
컴파일러가 최적화하여 수동 인덱스 루프와 동일한 성능을 보장합니다.",
        diagram: "\
  반복문의 진화: 안전해지는 과정
  ──────────────────────────────────

  ① while (위험)         ② for + 인덱스 (나음)    ③ for in (최고)
  let mut i = 0;         for i in 0..3 {         for item in &items {
  while i < 3 {              println!(              println!(
      println!(i);               items[i]);            item);
      i += 1;               }                       }
  }
  위험: i 업데이트        나음: 범위 고정          최고: 인덱스 불필요
  누락시 무한루프         하지만 인덱스 접근       안전하고 관용적

  loop + break로 값 반환:
  ──────────────────────────────────

  let result = loop {
      count += 1;
      if count == 5 {
          break count * 2;  ──►  result = 10
      }
  };

  중첩 루프 탈출 (라벨):
  ──────────────────────────────────
  'outer: for x in 0..3 {
      for y in 0..3 {
          if x==1 && y==1 {
              break 'outer;  ──► 두 루프 모두 탈출!
          }
      }
  }",
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

    // reverse iterator
    for i in (1..=5).rev() {
        print!("{} ", i);
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
        comparisons: &[
            "header|Rust|TypeScript",
            "diff|if 식 (값 반환)|if 문 (값 없음)",
            "left|loop + break value|while(true) + break",
            "left|for x in iter|for...of",
            "left|0..5 범위 타입|i < 5 수동 체크",
            "left|'label: break 'label|label: break label",
        ],
    }
}
