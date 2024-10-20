const _CONST_SHOULD_UPPER_CASE: i32 = 20; // 'const' binding은 타입을 직접 명시해야 함. 추론이 안 됨

// static mut 변수는 전역적으로 접근 가능하고 변경 가능하기 때문에, 
// 여러 스레드에서 동시에 접근하거나 변경할 수 있어 
// 데이터 레이스(data race)와 같은 문제를 일으킬 수 있습니다.
static mut SCORE: i32 = 11; // unsafe

fn main() {
    let _let_binding = 8; // 'let' binding
    
    unsafe {
        SCORE = 0;
    }
}
