use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Threads & Mutex (멀티스레딩)",
        category: "동시성",
        explanation: "\
Rust 멀티스레딩은 '두려움 없는 동시성(fearless concurrency)'을 지향합니다.
Send, Sync 트레이트로 스레드 안전성을 컴파일 타임에 보장합니다.

  Send: 이 타입을 다른 스레드로 전달할 수 있음
  Sync: 이 타입의 참조를 여러 스레드에서 공유할 수 있음

thread::spawn: 새 스레드 생성 (move 클로저로 소유권 이전)
Mutex<T>: 한 번에 하나의 스레드만 접근 가능
Arc<Mutex<T>>: 멀티스레드 공유 가변 상태의 표준 패턴",
        why_it_matters: "\
Java/Node.js에서 공유 상태 버그(race condition)가 런타임에 발생합니다.
Rust는 Send/Sync 체크로 컴파일 타임에 잡습니다.
NestJS는 이벤트 루프 단일 스레드이지만, Rust는 진정한 병렬 실행이 가능합니다.",
        diagram: "\
  Arc<Mutex<T>> 패턴 시각화
  ──────────────────────────────────

  Thread 1        Thread 2        Thread 3
  ┌──────┐       ┌──────┐       ┌──────┐
  │ clone│       │ clone│       │ clone│
  │  Arc │       │  Arc │       │  Arc │
  └──┬───┘       └──┬───┘       └──┬───┘
     │              │              │
     └──────────────┼──────────────┘
                    │
             ┌──────▼──────┐
             │   Arc<T>    │
             │  ┌────────┐ │
             │  │ Mutex  │ │
             │  │ ┌────┐ │ │
             │  │ │ 0  │ │ │
             │  │ └────┘ │ │
             │  │ lock() │ │
             │  └────────┘ │
             └─────────────┘

  스레드 간 순차적 접근:
  ──────────────────────────────────

  T1: lock() ──► *num += 1 ──► unlock
                                  T2: lock() ──► *num += 1 ──► unlock
                                                                 T3: lock() ──► *num += 1 ──► unlock
  결과: 항상 3 (race condition 불가!)

  Send & Sync 자동 구현:
  ──────────────────────────────────
  대부분의 타입은 자동으로 Send + Sync
  Rc<T> → Send 아님! (컴파일 에러로 감지)
  Arc<T> → Send + Sync (원자적 연산)",
        code: r#"use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // 기본 스레드 생성
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("스레드: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("메인: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();

    // move 클로저: 변수 소유권을 스레드로 이전
    let data = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("데이터: {:?}", data);
    });
    handle2.join().unwrap();

    // Arc<Mutex<T>>: 여러 스레드에서 공유 카운터
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("최종 카운터: {}", *counter.lock().unwrap());  // 5
}
"#,
        key_points: &[
            "thread::spawn + move 클로저: 소유권을 스레드로 이전",
            "Arc<Mutex<T>>: 멀티스레드 공유 가변 상태의 표준 패턴",
            "Mutex::lock(): 잠금 획득, MutexGuard drop 시 자동 해제",
            "Send/Sync: 컴파일 타임에 스레드 안전성 보장",
        ],
        comparisons: &[
            "header|Rust|Node.js",
            "diff|진정한 병렬 (OS 스레드)|이벤트 루프 (단일 스레드)",
            "diff|Arc<Mutex<T>>로 공유|공유 상태 없음",
            "win|컴파일 타임 race condition 방지|런타임才 감지",
            "diff|move로 소유권 명확|비동기 콜백/async-await",
        ],
    }
}
