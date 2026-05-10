use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Smart Pointers (Box, Rc, Arc)",
        category: "동시성",
        explanation: "\
스마트 포인터: 추가 메타데이터나 기능이 있는 포인터

  Box<T>: Heap 할당, 단일 소유권
    - 재귀 타입 (컴파일 타임에 크기 모를 때)
    - Box<dyn Trait> 동적 디스패치

  Rc<T>: Reference Counting, 단일 스레드 공유 소유권
    - 여러 곳에서 같은 데이터를 읽을 때
    - 멀티스레드 불가!

  Arc<T>: Atomic RC, 멀티스레드 안전
    - 스레드 간 공유할 때

  RefCell<T>: 런타임에 빌림 규칙 검사 (내부 가변성)
    - 컴파일 타임 검사를 런타임으로 미룸",
        why_it_matters: "\
'여러 곳에서 같은 데이터를 소유하고 싶다' → Rc<T> 또는 Arc<T>
Box<dyn Trait>: 런타임에 다양한 타입을 다룰 때
Rc<RefCell<T>>: 단일 스레드 공유 + 가변 (그래프 구조)
Arc<Mutex<T>>: 멀티스레드 공유 + 가변",
        diagram: "\
  스마트 포인터 메모리 구조
  ──────────────────────────────────

  Box<T>:
  스택              Heap
  ┌───────┐        ┌─────┐
  │ Box   │        │  5  │
  │ ptr───┼───────►│     │
  └───────┘        └─────┘
  단일 소유권

  Rc<T>:
  스택    스택    스택         Heap
  ┌────┐ ┌────┐ ┌────┐    ┌───────┐
  │ a  │ │ b  │ │ c  │    │ value │
  │ptr─┼─┤ptr─┼─┤ptr─┼───►│  5    │
  └────┘ └────┘ └────┘    │count:3│
                          └───────┘
       참조 카운트 공유

  Rc<RefCell<T>>:
  ┌────────┐    ┌──────────┐    ┌─────┐
  │ Rc     │    │ RefCell  │    │     │
  │ ptr───┼───►│ ptr──────┼───►│  5  │
  │        │    │ borrow:  │    │     │
  └────────┘    │ mutable  │    └─────┘
                └──────────┘
  런타임 빌림 검사!

  선택 가이드:
  ──────────────────────────────────
  단일 소유권 + Heap?  → Box<T>
  공유 소유권 (단스레드)? → Rc<T>
  공유 + 가변 (단스레드)? → Rc<RefCell<T>>
  공유 (멀티스레드)?  → Arc<T>
  공유 + 가변 (멀티스레드)? → Arc<Mutex<T>>",
        code: r#"use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    // Box<T>: Heap 할당
    let b = Box::new(5);
    println!("Box: {}", *b);

    // Box<dyn Trait>: 동적 디스패치
    trait Speak { fn speak(&self); }
    struct Dog;
    struct Cat;
    impl Speak for Dog { fn speak(&self) { println!("멍!"); } }
    impl Speak for Cat { fn speak(&self) { println!("야옹!"); } }

    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &animals { a.speak(); }

    // Rc<T>: 공유 소유권 (단일 스레드)
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    println!("Count: {}", Rc::strong_count(&a));  // 3
    drop(b);
    println!("After drop: {}", Rc::strong_count(&a));  // 2

    // Rc<RefCell<T>>: 공유 + 가변
    let shared = Rc::new(RefCell::new(0));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    *clone1.borrow_mut() += 10;
    *clone2.borrow_mut() += 20;
    println!("Shared: {}", shared.borrow());  // 30

    // Arc<T>: 멀티스레드 공유 소유권
    let counter = Arc::new(0);
    let counter_clone = Arc::clone(&counter);
    println!("Arc: {}", counter_clone);
}
"#,
        key_points: &[
            "Box<T>: Heap 할당, 재귀 타입, Box<dyn Trait> 동적 디스패치",
            "Rc<T>: 단일 스레드 공유 소유권 / Arc<T>: 멀티스레드",
            "RefCell<T>: 컴파일 타임 대신 런타임에 빌림 검사 (내부 가변성)",
            "Rc<RefCell<T>>: 단스레드 공유+가변 / Arc<Mutex<T>>: 멀티스레드",
        ],
        comparisons: &[
            "header|Box<T>|Rc<T>|Arc<T>",
            "diff|단일 소유권|공유 소유권|공유 소유권",
            "diff|스레드 안전|단일 스레드만|멀티스레드 OK",
            "equal|Heap 할당|Heap 할당|Heap 할당",
            "diff|카운트 없음|참조 카운트|원자적 카운트",
        ],
    }
}
