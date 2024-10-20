
## cargo

```bash
cargo init
cargo build
cargo run

# watch style cargo run
cargo install cargo-watch
cargo watch -x run
```

## learn things

1. rust에서 char는 32비트 (4바이트) -> 유니코드 지원을 위함.
2. 블록 마지막 표현식이 세미콜론 없이 작성되면 암시적으로 반환값이 됨
3. Rust는 다중 레벨의 레퍼런스를 허용합니다.

## others

```text
[String]
힙에 할당된 문자열로, 소유권을 가집니다. 변수가 스코프를 벗어나면 자동으로 메모리가 해제됩니다.
소유권 규칙에 따라 관리됩니다.

[&str]
문자열 슬라이스로, 다른 문자열 데이터(String이나 정적 문자열)를 참조합니다. 소유권이 없습니다.
명시적인 라이프타임 지정이 필요할수도 있음

[변환]
String에서 &str로: &를 사용하여 쉽게 변환 가능 (&String은 &str로 자동 변환)
&str에서 String으로: to_string() 또는 String::from()을 사용
```