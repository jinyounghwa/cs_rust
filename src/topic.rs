/// CS 학습 토픽을 나타내는 구조체
pub struct CSTopic {
    pub title: &'static str,
    pub category: &'static str,
    pub explanation: &'static str,
    pub why_it_matters: &'static str,
    pub diagram: &'static str,       // 아스키 다이어그램 (비어있으면 미표시)
    pub code: &'static str,
    pub key_points: &'static [&'static str],
    pub comparisons: &'static [&'static str], // 비교 표 (header|left|right 형식)
}
