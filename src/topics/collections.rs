use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Vec<T>와 HashMap<K,V> (컬렉션)",
        category: "기초",
        explanation: "\
두 가지 가장 자주 쓰는 컬렉션입니다.

  Vec<T>: 동적 배열 (TypeScript의 배열, Java의 ArrayList)
    - Heap에 저장, 크기 동적 변경
    - 연속된 메모리 블록
    - 인덱스 접근: v[0] (패닉) vs v.get(0) (안전)

  HashMap<K, V>: 해시맵 (TypeScript의 Map, Node의 객체)
    - 키-값 저장, O(1) 평균 조회
    - Heap에 저장, 순서 보장 없음
    - get()은 Option<&V>를 반환 — 키가 없으면 None

둘 다 소유권 규칙을 따릅니다. 값을 넣으면 소유권이 이동됩니다.",
        why_it_matters: "\
Vec은 단순 배열보다 훨씬 많이 씁니다. 동적 크기 + 반복자와 조합하면 강력합니다.
HashMap은 NestJS의 Map<string, T>와 같은 역할.
주의: HashMap은 std::collections에서 import 필요.",
        diagram: "\
  Vec<T>의 메모리 구조
  ──────────────────────────────────

  let v = vec![10, 20, 30, 40, 50];

  스택:                   Heap (연속 메모리):
  ┌──────────┐           ┌───┬───┬───┬───┬───┐
  │ ptr ─────┼──────────►│10 │20 │30 │40 │50 │
  │ len: 5   │           └───┴───┴───┴───┴───┘
  │ cap: 8   │            [0] [1] [2] [3] [4]
  └──────────┘            ↑ cap-len 만큼 여유

  v.push(60) 시:
  여유 공간(cap-len)이 있으면 → Heap 끝에 추가 (빠름)
  여유 공간 없으면 → 더 큰 Heap 재할당 (느림, O(n))

  안전한 접근 vs 위험한 접근:
  ──────────────────────────────────

  v[0]      → 값 직접 반환, 범위 밖이면 패닉!
  v.get(0)  → Some(&값) 반환, 범위 밖이면 None
  v.get(99) → None ← panic 없이 안전하게 처리",
        code: r#"use std::collections::HashMap;

fn main() {
    // Vec 생성
    let mut v: Vec<i32> = Vec::new();  // 빈 Vec
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![10, 20, 30];  // 매크로로 초기화

    // Vec 접근
    println!("v[0] = {}", v[0]);             // 인덱스 (패닉 가능)
    println!("{:?}", v.get(1));              // Option<&i32>, 안전
    println!("len: {}", v.len());

    // Vec 반복
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // Vec 변환
    v.push(99);
    v.sort();
    v.dedup();          // 중복 제거 (정렬 후)
    println!("{:?}", v);

    // HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 75);

    // 조회: Option<&V> 반환
    let alice = scores.get("Alice");
    match alice {
        Some(score) => println!("Alice: {}", score),
        None => println!("Not found"),
    }

    // 없으면 삽입 (or_insert)
    scores.entry(String::from("Charlie")).or_insert(80);

    // 반복
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 값이 있는지 확인
    println!("Bob 있나? {}", scores.contains_key("Bob"));
}
"#,
        key_points: &[
            "vec![...]: 매크로로 간단 초기화 / Vec::new()로 빈 Vec 생성",
            "v[0]: 패닉 가능 / v.get(0): Option<&T> 반환 (안전)",
            "HashMap::get(): Option<&V> — 없는 키 접근을 안전하게 처리",
            "entry().or_insert(): 없으면 삽입, 있으면 기존 값 유지 (upsert)",
        ],
        comparisons: &[
            "header|Vec<T>|HashMap<K,V>",
            "left|순서 보장|순서 보장 안 됨",
            "left|인덱스 접근 O(1)|키 접근 O(1) 평균",
            "left|vec![] 매크로|HashMap::new()",
            "left|push, pop, sort|insert, get, entry",
            "left|연속 메모리|해시 버킷 구조",
        ],
    }
}
