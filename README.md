
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
4. static을 mut 변수로 선언하고 활용하는 건 unsafe함

## others

```text
[String]
https://doc.rust-lang.org/std/string/struct.String.html  
힙에 할당된 문자열로, 소유권을 가집니다. 변수가 스코프를 벗어나면 자동으로 메모리가 해제됩니다.
소유권 규칙에 따라 관리됩니다.

[&str]
문자열 슬라이스로, 다른 문자열 데이터(String이나 정적 문자열)를 참조합니다. 소유권이 없습니다.
&str은 포인터네? 실제 문자 배열은 힙에 있고, 포인터만 스택에 있는거지
명시적인 라이프타임 지정이 필요할수도 있음

[변환]
String에서 &str로: &를 사용하여 쉽게 변환 가능 (&String은 &str로 자동 변환)
&str에서 String으로: to_string() 또는 String::from()을 사용
```

## ref

```
& : immutable ref, shared ref, immutable borrow
&mut : mutable ref, unique ref, mutable borrow
```

- 가변 참조가 존재하는 동안에는 원본 변수를 직접 수정할 수 없음.
- 어떤 시점에서 하나의 가변 참조(&mut T) 또는 여러 개의 불변 참조(&T)를 가질 수 있지만, 둘을 동시에 가질 수는 없습니다.
  - 어떤 변수의 가변 참조는 한개만 가질 수 있음
  - 어떤 변수의 불변 참조는 여러개 가질 수 있음
  - 어떤 변수에 대해서 가변 참조와 불변 참조 둘 다 가질 순 없음. 둘 중 하나만 가능
- 어떤 변수 shadowing을 하더라도 해당 변수의 ref는 살아 있음.
