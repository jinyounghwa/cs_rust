use crate::topic::CSTopic;

pub mod variables;
pub mod types;
pub mod functions;
pub mod strings;
pub mod control_flow;
pub mod collections;
pub mod structs;
pub mod enums;
pub mod ownership;
pub mod borrowing;
pub mod option;
pub mod result;
pub mod traits;
pub mod generics;
pub mod closures;
pub mod lifetimes;
pub mod smartpointers;
pub mod threads;
pub mod channels;
pub mod modules;
pub mod testing;
pub mod error_handling;

/// 모든 토픽을 순서대로 반환
pub fn all_topics() -> Vec<CSTopic> {
    vec![
        // ────────── 기초: 언어의 첫걸음 ──────────
        variables::topic(),
        types::topic(),
        functions::topic(),
        strings::topic(),
        control_flow::topic(),
        collections::topic(),
        // ────────── 핵심: Rust다운 코드 ──────────
        structs::topic(),
        enums::topic(),
        ownership::topic(),
        borrowing::topic(),
        option::topic(),
        result::topic(),
        // ────────── 추상화: 다형성과 재사용 ──────────
        traits::topic(),
        generics::topic(),
        closures::topic(),
        lifetimes::topic(),
        // ────────── 동시성과 메모리 ──────────
        smartpointers::topic(),
        threads::topic(),
        channels::topic(),
        // ────────── 실전 패턴 ──────────
        modules::topic(),
        testing::topic(),
        error_handling::topic(),
    ]
}
