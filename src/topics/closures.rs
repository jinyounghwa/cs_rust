use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Closures & Iterators — 함수형 패턴",
        category: "추상화",
        explanation: "\
클로저: 주변 환경(변수)을 캡처하는 익명 함수
  |x| x + 1              // 인자 하나, 타입 추론
  move |x| x + n         // n의 소유권을 캡처

Fn 트레이트 3종:
  Fn:     불변 참조로 캡처 (&self)
  FnMut:  가변 참조로 캡처 (&mut self)
  FnOnce: 소유권으로 캡처 (self) — 한 번만 호출 가능

Iterator: 지연 계산(lazy)으로 데이터를 처리하는 체인
  map, filter, fold, take, skip, flatten, zip, enumerate, ...

체인이 길어도 중간 컬렉션을 생성하지 않아서 메모리 효율이 좋습니다.",
        why_it_matters: "\
NestJS의 Array 메서드(map, filter, reduce)와 비슷하지만 성능이 다릅니다.
Rust Iterator는 lazy: 실제로 소비(collect/for_each)되기 전까지 계산하지 않습니다.
Iterator를 직접 구현하면 for 루프를 쓸 수 있습니다.",
        diagram: "\
  Iterator 체인: Lazy 평가
  ──────────────────────────────────

  data.iter()
      .filter(...)     ← 아직 안 함
      .map(...)        ← 아직 안 함
      .take(3)         ← 아직 안 함
      .collect()       ← 여기서 실행!

  중간 배열 생성 없음:
  ──────────────────────────────────

  [1,2,3,4,5,6,7,8,9,10]
       │
    filter(|x| x%2==0)    [2,4,6,8,10]
       │
    map(|x| x*x)          [4,16,36,64,100]
       │
    take(3)               [4,16,36]
       │
    collect()             → Vec<i32>

  실제로는 중간 Vec 없이 한 번에 처리!

  클로저 캡처 방식:
  ──────────────────────────────────

  Fn (불변 참조):        FnMut (가변 참조):     FnOnce (소유권):
  let n = 5;             let mut c = 0;         let s = String::new();
  let f = |x| x + n;    let mut f = || {       let f = move || {
  // n을 &로 빌림           c += 1; c            // s의 소유권 이동
  // n 계속 유효        };                        // s 더 이상 사용 불가
                         // c를 &mut로 빌림",
        code: r#"fn main() {
    // 클로저 기본
    let add = |x: i32, y: i32| x + y;
    println!("{}", add(3, 4));

    // 환경 캡처: n을 &로 빌림
    let n = 5;
    let add_n = |x| x + n;  // Fn: n을 불변 참조로 캡처
    println!("{}", add_n(3));  // 8, n도 여전히 유효

    // FnMut: 가변 참조로 캡처
    let mut count = 0;
    let mut inc = || { count += 1; count };
    println!("{}", inc());  // 1
    println!("{}", inc());  // 2

    // move: 소유권 이전 (스레드에서 필수)
    let text = String::from("hello");
    let show = move || println!("{}", text);
    show();

    // Iterator 체인
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .take(3)
        .collect();
    println!("{:?}", result);  // [4, 16, 36]

    // fold: 누산 (reduce)
    let sum: i32 = data.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // flat_map: 중첩 컬렉션 펼치기
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{:?}", chars);

    // 직접 반복자 구현
    struct Counter { count: u32 }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            self.count += 1;
            if self.count <= 5 { Some(self.count) } else { None }
        }
    }
    let total: u32 = Counter { count: 0 }.sum();
    println!("Counter sum: {}", total);
}
"#,
        key_points: &[
            "Fn/FnMut/FnOnce: 캡처 방식에 따른 분류 (컴파일러가 자동 선택)",
            "move |...| { ... }: 소유권 이전 — 스레드/비동기 코드에서 필수",
            "Iterator는 lazy: collect()/for_each()까지 실제 계산 안 함",
            "체인 (filter→map→take): 중간 Vec 생성 없이 효율적으로 처리",
        ],
        comparisons: &[
            "header|Fn|FnMut|FnOnce",
            "diff|불변 참조 캡처|가변 참조 캡처|소유권 캡처",
            "diff|여러 번 호출 가능|여러 번 호출 가능|한 번만 호출",
            "diff|&T로 환경 접근|&mut T로 환경 접근|T 소비",
        ],
    }
}
