
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
5. Copy 트레이트를 구현하지 않은 타입 (예: String, Vec)은 인자로 넘겨도 값 복사는 일어나지 않고, 소유권만 이동합니다.
6. empty tuple = () = unit type
7. match는 타입이 가능한 모든 경우의 수를 exhaustive하게 처리해야 함.
8. 얕은 복사와 move는 다르다. 얕은 복사는 객체의 포인터만 복사해가는가고 무브는 얕은 복사와 같은데 다만 이전 객체를 무효화하는 것이다



## concepts

### String, &str

String
- https://doc.rust-lang.org/std/string/struct.String.html  
- 힙에 할당된 문자열로, 소유권을 가집니다. 변수가 스코프를 벗어나면 자동으로 메모리가 해제됩니다.
소유권 규칙에 따라 관리됩니다.
- String은 항상 24바이트임. 실세 문자열은 heap 저장되고, heap을 가리키는 포인트(8) + len(8) + cap(8) 다 합쳐서 24바이트.

&str
문자열 슬라이스로, 다른 문자열 데이터(String이나 정적 문자열)를 참조합니다. 소유권이 없습니다.
&str은 포인터네? 실제 문자 배열은 힙에 있고, 포인터만 스택에 있는거지
명시적인 라이프타임 지정이 필요할수도 있음

String에서 &str로: &를 사용하여 쉽게 변환 가능 (&String은 &str로 자동 변환)
&str에서 String으로: to_string() 또는 String::from()을 사용

### reference
```
& : immutable ref, shared ref, immutable borrow
&mut : mutable ref, unique ref, mutable borrow
```

- Ref를 만드는 행위는 borrow라 함. 
- Ref도 스코프 벗어나면 없어짐
- 가변 참조가 존재하는 동안에는 원본 변수를 직접 수정할 수 없음.
- 어떤 시점에서 하나의 가변 참조(&mut T) 또는 여러 개의 불변 참조(&T)를 가질 수 있지만, 둘을 동시에 가질 수는 없습니다.
  - 어떤 변수의 가변 참조는 한개만 가질 수 있음
  - 어떤 변수의 불변 참조는 여러개 가질 수 있음
  - 어떤 변수에 대해서 가변 참조와 불변 참조 둘 다 가질 순 없음. 둘 중 하나만 가능
- 어떤 변수 shadowing을 하더라도 해당 변수의 ref는 살아 있음.

### Sized vs DST
- 러스트에서는 크게 두 타입 존재
  - 정적 크기 타입 (Sized types): 컴파일 시점에 크기를 알 수 있는 타입
    - String 
    - array
    - vecs : 실제 내용은 힙에 저장되고, 타입은 해당 힙을 가리키는 포인터를 가지고 있음
  - 동적 크기 타입 (Dynamically Sized Types, DST): 런타임에 크기가 결정되는 타입
    - &str : string slice 라고도 불림
    - slice : array나 vecs 자르면 slice
    - DST는 항상 참조나 포인터를 통해 다루어집니다 (예: &str, &[T], Box<dyn Trait>).

- Sized 타입은 스택에 직접 저장될 수 있지만, DST는 항상 간접적으로 (참조나 포인터를 통해) 다루어집니다.

### slice

- 슬라이스는 컨테이너의 참조이자 원본 컨테이너에 대한 소유권은 없음

### struct

```rs
struct FileDir;           // empty struct
struct Color(u8, u8, u8); // tuple struct
// named struct
struct Country {
    pop: u32,
    capital: String,
    leader_name: String
}
```

```rust
struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    
    // --------------------
    // stack
    // heap
    // data
    // code
    // --------------------

    // SeaCreature의 데이터는 stack에 있음
    let ferris = SeaCreature {
        // String struct도 stack에 있지만,
        // heap에 있는 데이터에 대한 참조를 갖고 있음
        animal_type: String::from("게"),
        name: String::from("Ferris"), // data 영역에 저장
        arms: 2,
        legs: 4,
        weapon: String::from("집게"), // data 영연ㄱ에 저장
    };
}
```

### enum, variant

- Enum은 variant소유
  - 예를 들어 Result enum에는 OK, Err variant 존재
  - 각 variant에 대한 match 키워드로 핸들링 가능.

```rust
// 데이터가 없는 기본 형태 (전통적인 enum과 비슷)
enum Basic {
    RED,
    GREEN,
    BLUE
}

// 데이터를 가질 수 있는 형태
enum Message {
    Quit,                          // 데이터 없음
    Move { x: i32, y: i32 },      // 구조체 스타일
    Write(String),                 // 단일 값
    ChangeColor(i32, i32, i32)    // 여러 값
}
```

```rust

// rust의 enum 메모리 모델은 "tagged union"
// 메모리 모델 중 0은 미사용, 1은 사용이라 가정할 때 다음과 같음
// 가장 긴 String 을 기반으로 다음과 같이 구성됨.
// 즉, 어느 enum 데이터를 사용하던지 (가장 긴 자료형의 크기 + 1[tag]) 만큼의 비트를 사용함.
enum Message {
    Quit,                       // 데이터 없음
                                // [00000000] | [미사용 24바이트          ]
                                // 태그:0   [        미사용 데이터        ]
  
    Move { x: i32, y: i32 },    // 8바이트 (i32 두개) 
                                // [00000001] | [8바이트][미사용 16바이트 ]
                                // 태그:1   [  x,y 데이터  ][   미사용   ]
                                // 태그 포함 총 25비트

    Write(String),              // 24바이트 (String)
                                // [00000010] | [String의 24바이트 메모리]
                                // 태그:2   [       String 데이터        ]
                                // 태그 포함 25바이트

    ChangeColor(i32, i32, i32)  // 12바이트 (i32 세개)
                                // [00000011] | [12 바이트][미사용 12바이트]
                                // 태그:3   [   r,g,b 데이터  ][미사용  ]
}

fn main() {
    let a = Message::Quit; // 25바이트를 사용함.
}
```

### traits
- 어떤 type은 특정한 traits를 implements 한다.
  - OOP에서 어떤 클래스가 특정한 interface를 구현하는 것과 같은 꼴이다.

### range

```
6..=10    // 6,7,8,9,10 (포함적 범위: 시작과 끝 모두 포함)
6...10    // 6..=10과 동일하지만 deprecated됨
6..10     // 6,7,8,9 (배제적 범위: 끝 숫자 제외)
..10      // 0,1,2,...,9 (시작점 생략, 0부터 시작)
6..       // 6부터 타입의 최대값까지
..        // 전체 범위
```