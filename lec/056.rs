use std::collections::VecDeque;

fn main() {
    /*
        Vec vs VecDeque의 성능 차이 데모
        
        1. Vec (일반 벡터)의 경우:
           - remove(0)나 요소를 앞에서 제거할 때마다
           - 남은 모든 요소들을 한 칸씩 앞으로 이동시켜야 함
           - 이는 O(n) 시간 복잡도를 가짐
           - 예: [1,2,3,4,5] → remove(0) → [2,3,4,5] (2,3,4,5를 전부 한칸씩 앞으로 이동)

        2. VecDeque (Double-ended Queue)의 경우:
           - 내부적으로 ring buffer(환형 버퍼) 구조를 사용
           - 앞쪽에서 요소를 제거할 때 실제로 데이터를 이동시키지 않음
           - 단순히 시작 포인터만 변경
           - 이는 O(1) 시간 복잡도를 가짐
           - 양방향에서의 효율적인 삽입/제거가 필요할 때 적합    */
    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for _i in 0..600000 {
        my_vec.pop_front(); // pop_front is like .pop but for the front
    }
}