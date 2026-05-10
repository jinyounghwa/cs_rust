# cs-bite — CLI 대기 시간 CS 공부 도구

Claude Code 실행 후 대기하는 2~5분 동안 Rust 코드 예제 기반 CS 개념을 터미널에 출력하는 학습 도구입니다.

## 설치

```bash
cargo install --path .
```

또는 프로젝트 디렉토리에서:

```bash
cargo build --release
./target/release/cs-bite
```

## 사용법

### 기본: 랜덤 예제 출력
```bash
cs-bite
```
무작위로 선택된 CS 개념을 설명, 코드 예제, 핵심 포인트와 함께 출력합니다.

### 예제 목록 보기
```bash
cs-bite list
```
모든 CS 토픽 제목을 나열합니다.

### 예제 코드 실행
```bash
cs-bite run
```
랜덤 예제를 출력한 후, 해당 코드 예제를 컴파일하고 실행합니다.

## 다루는 주제

1. **Ownership & Borrowing** — 메모리 관리와 소유권 시스템
2. **Result<T, E> & Error Handling** — 명시적 에러 처리
3. **Traits (인터페이스)** — 추상화와 다형성
4. **Pattern Matching** — 구조 분해와 패턴 매칭
5. **Closures & Function Traits** — 함수형 프로그래밍
6. **Generics (제네릭)** — 타입 안정성과 단형화
7. **Lifetime ('a)** — 참조의 유효 범위
8. **Channels (메시지 패싱)** — 스레드 간 안전한 통신

## 특징

- ✨ 2~5분 안에 읽을 수 있는 분량
- 🎯 실행 가능한 Rust 코드 예제
- 📝 핵심 포인트 3줄 정리
- 🌈 컬러 터미널 출력
- 📚 NestJS 백엔드 경험자를 위한 비교 설명

## 개발

```bash
cargo build        # 디버그 빌드
cargo build --release  # 릴리스 빌드
cargo run          # 실행
cargo run -- list  # 주제 목록 보기
cargo run -- run   # 코드 예제 실행
```

## 라이센스

MIT
