use crate::topic::CSTopic;

pub fn topic() -> CSTopic {
    CSTopic {
        title: "Channels — 메시지 패싱으로 통신",
        category: "동시성",
        explanation: "\
'메모리를 공유해서 통신하지 말고, 통신해서 메모리를 공유하라'
Go 철학을 Rust도 채용합니다.

  mpsc: Multiple Producer, Single Consumer
  (tx, rx) = mpsc::channel()
  tx.send(val): val의 소유권을 채널로 이전
  rx.recv(): 블로킹 수신 (값이 올 때까지 대기)
  rx.try_recv(): 비블로킹 수신

tx는 clone() 가능 → 여러 스레드에서 전송 가능
rx는 clone() 불가 → 수신자는 하나

소유권을 이전하므로 채널을 통과한 데이터에는 데이터 경쟁이 없습니다.",
        why_it_matters: "\
Arc<Mutex<T>>보다 간단한 경우가 많습니다.
Producer-Consumer 패턴, 작업 큐, 결과 수집에 유용합니다.
Go의 goroutine + channel 패턴과 비슷한 아이디어입니다.",
        diagram: "\
  Channel 구조 시각화
  ──────────────────────────────────

  mpsc::channel()

  Producer(tx)                       Consumer(rx)
  ┌──────────┐                      ┌──────────┐
  │ tx       │    채널 (FIFO 큐)     │ rx       │
  │ send(val)├────►│ val1 │ val2 │──►│ recv()   │
  └──────────┘     └──────┴──────┘   └──────────┘
  소유권 이동 →    순서 보장          블로킹 대기
  val 사용 불가

  Multiple Producer:
  ──────────────────────────────────

  Thread 1: tx.send(\"A\") ──┐
                            ├──► Channel ──► rx.recv()
  Thread 2: tx_clone.send(\"B\")──┘

  tx.clone()으로 여러 생산자 가능!
  rx는 하나뿐 — 단일 소비자

  recv() 동작:
  ──────────────────────────────────
  rx.recv()      → 블로킹 (값 올 때까지 대기)
  rx.try_recv()  → 즉시 반환 (Ok/Err)
  for msg in rx  → 모든 tx drop될 때까지 반복",
        code: r#"use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 기본 채널 통신
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();  // val의 소유권 이전
    });

    let received = rx.recv().unwrap();  // 블로킹 대기
    println!("Got: {}", received);

    // 여러 값 전송
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "thread"];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    for received in rx2 {  // rx를 반복자처럼 사용
        println!("Received: {}", received);
    }

    // 여러 Producer (tx 복제)
    let (tx3, rx3) = mpsc::channel();
    let tx3_clone = tx3.clone();

    thread::spawn(move || {
        tx3.send(String::from("Producer 1")).unwrap();
    });
    thread::spawn(move || {
        tx3_clone.send(String::from("Producer 2")).unwrap();
    });

    for _ in 0..2 {
        println!("{}", rx3.recv().unwrap());
    }
}
"#,
        key_points: &[
            "mpsc: 다중 생산자(tx.clone()), 단일 소비자(rx)",
            "send(val): 소유권 이전 → 전송 후 원본 접근 불가 = 데이터 경쟁 없음",
            "rx를 for in으로 사용: 모든 tx가 drop될 때까지 반복",
            "Mutex 공유보다 채널이 더 안전하고 구조가 명확한 경우가 많음",
        ],
        comparisons: &[
            "header|Channel|Arc<Mutex<T>>",
            "diff|메시지 패싱|공유 메모리",
            "diff|소유권 이전|공유 참조",
            "left|Producer-Consumer 패턴|공유 상태 패턴",
            "win|데드락 위험 적음|데드락 주의 필요",
            "left|구조가 명확|더 유연한 접근",
        ],
    }
}
