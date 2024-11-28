
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

### format

```
: 포매팅지정자
포매팅지정자 다음 오는 문자 : 이걸로 채우겠다.
^ 가운데 정렬 < 왼쪽 정렬 > 오른쪽 정렬
width$ width 만큼 너비 지정
? debug format사용
```

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

- 소유권 목표 : 읽던기, 바꾸던지 둘 중 하나만 가능.
  - Data is never mutated while it's being read
  - Data is never read while it's being mutated
  - 이 원칙에 따라, `가변 참조가 존재하는 동안에는 원본 변수를 직접 수정할 수 없음.`
- 어떤 시점에서 하나의 가변 참조(&mut T) 또는 여러 개의 불변 참조(&T)를 가질 수 있지만, 둘을 동시에 가질 수는 없습니다.
  - 어떤 변수의 가변 참조는 한개만 가질 수 있음 (unique ref 같은 느낌)
  - 어떤 변수의 불변 참조는 여러개 가질 수 있음 (shared ref 같은 느낌)
  - 어떤 변수에 대해서 가변 참조와 불변 참조 둘 다 가질 순 없음. 둘 중 하나만 가능
    - `single writer or multiple writer`
  - &       immutable ref / shared ref
  - &mut    mutable ref / unique ref
- 어떤 변수 shadowing을 하더라도 해당 변수의 ref는 살아 있음.
- 참조의 종류와 관계없이 대여 중인 값의 원본을 수정할 수 없습니다.

- 메서드의 첫 인자로 self 를 넣을 때도 고려해야.
  - 인스턴스를 변환하거나 소비해야 하는 경우 -> self
  - 데이터를 읽기만 하는 경우 -> &self
  - 데이터를 수정해야 하는 경우 -> &mut self


### Interior Mutability Types

interior mutability(내부 가변성) == 불변 참조(&)를 통해서도 내용을 변경할 수 있게 해주는 패턴

`Cell`: 간단한 값 타입용
`RefCell`: 단일 스레드에서 복잡한 타입용. runtime checked borrowing rules
`Mutex/RwLock`: 멀티스레드 환경용
`Atomic`: 고성능 멀티스레드 연산용
`OnceCell/OnceLock`: 초기화 한 번만 필요한 경우


- Cell::get()은 내부 값의 복사본을 반환하는데, 이는 Copy 트레이트가 구현된 타입에서만 가능합니다.
- RefCell은 즉, single writer or multiple writer 규칙을 런타임에 검사하고 아니면 패닉



### ptr

- 레퍼런스(&) 도 포인터임.

- 소유권 관련
Box: 힙 메모리 할당
Rc: 참조 카운팅
Arc: 원자적 참조 카운팅 (스레드 안전)

- 내부 가변성
Cell, RefCell
Mutex, RwLock
Atomic 타입들

- rust에서도 원시 포인터가 존재하지만 unsafe 내에서만 역참조 가능.

### Rc


- fully qualified syntax
  - rc.clone() 하지 말고 Rc::clone(&rc) 로 확실하게 RC 타입을 활용하고 있음을 보여야 함.

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
- 슬라이스(slice) - 런타임에 길이가 정해지는 유사한 원소들의 collection
- str(문자열 slice) - 런타임에 길이가 정해지는 텍스트

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
- match 외에 if let, while let 을 사용하여 간단하게 원하는 것만 처리 가능.

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

- 특정 trait를 구현하면 몇가지 메서드가 자동구현되는 위계가 존재함.
  - automatic implementation
    - ToString을 직접 impl하기보다는 impl Display만 하면 .to_string()이 포함됨
    - FromStr → parse() 메서드 자동 구현
    - AsRef<T> → 여러 타입에 대한 참조 변환을 자동으로 처리 (특히 String, str, Path 관련 작업에서 자주 사용)
    - From<T> 구현 → 자동으로 대응되는 Into<T> 구현됨 (이건 많이 사용되는 패턴이라 특별히 기억해두면 좋습니다)


#### copy traits

copy traits의 조건(https://rust-exercises.com/100-exercises/04_traits/12_copy.html)  

- it must implement Clone (Clone : Copy)
- The type doesn't manage any additional resources (e.g. heap memory, file handles, etc.) beyond the std::mem::size_of bytes that it occupies in memory.
  - 이 점 때문에, Drop을 구현한 타입은 Copy를 구현할 수 없음. Drop은 메모리 정리를 하므로 Drop이 구현된 구조체는 스택에 지정된 것 외에 힙에 뭔가 저장해서 관리하고 있다는 점을 암시함.
- The type is not a mutable reference (&mut T)

### blanket traits impl

특정 트레이트를 구현한 모든 타입에 대한 impl

```rust
// T가 특정 트레이트를 구현한 모든 타입에 대해
impl<T: SomeTrait> OtherTrait for T {
    // OtherTrait 구현
}
```

### trait object

- dyn키워드는 트레이트 객체를 나타내는 키워드
- 
```rust
// Printable 트레이트를 구현하되, 연관 타입 Age 타입이 u32인 것만 받겠다.
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}

// Trait1과 Trait2 두 개 다 구현한 것만 인자로 받겠다.
fn some_function(param: &(dyn Trait1 + Trait2)) {
    // Function body
}
```

### impl

- 러스트에서 impl은 struct와 enum 둘 다에다 사용할 수 있음

- 앞에 self를 받으면 메서드 아니면 연관함수
  - Rust에서 self 매개변수는 반드시 메서드의 첫 번째 매개변수여야 합니다. 두 번째나 다른 위치에 넣으면 컴파일 에러가 발생합니다
  - 일반적인 메서드는 인스턴스에서 점(.) 표기법으로 호출하는 반면, 연관 함수는 타입 이름과 함께 이중 콜론(::)을 사용해 호출합니다.
    - Type::Something은 연관함수, Type.method() 는 메서드.
- dot operator는 알아서 필요한 만큼 dereferencing 한다.


### impl method

- 메서드의 첫 인자로 self 를 넣을 때도 고려해야.
  - 인스턴스를 변환하거나 소비해야 하는 경우 -> self
  - 데이터를 읽기만 하는 경우 -> &self
  - 데이터를 수정해야 하는 경우 -> &mut self

- setter를 구현할 때 mut 혹은 &mut 둘 다 이점이 있음. 
```rust
// mut을 넘기면, self를 재반환하여 메서드 체이닝이 가능함
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        self.title = new_title;
        self
    }
}

// 단순히 바꾸고 말거면 &mut
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }
}
```

### generic

typescript와 매우 유사한 편임... 


### range

```
6..=10    // 6,7,8,9,10 (포함적 범위: 시작과 끝 모두 포함)
6...10    // 6..=10과 동일하지만 deprecated됨
6..10     // 6,7,8,9 (배제적 범위: 끝 숫자 제외)
..10      // 0,1,2,...,9 (시작점 생략, 0부터 시작)
6..       // 6부터 타입의 최대값까지
..        // 전체 범위
```

### package, creates, modules

- Cargo.toml 하나당 package 1개
- package는 하나 이상의 creates로 구성됨. (bin|lib)creates
- 코드 내 응집력을 위해 module이 존재함. (js module system과 비슷함)

- 현재 프로젝트 내의 모듈은 crate 이하에 존재함.
- 외부 create는 자체적으로 use tokio::runtime 꼴이고
- std 는 use std::collections::HashMap 꼴임.
- super:: 는 상위 모듈을 가리키는 상대 경로임
- 일반적으로 선호하는 경로는 절대 경로입니다. 아이템을 정의하는 코드와 호출하는 코드는 분리되어 있을 가능성이 높기 때문입니다.

- 구조체 정의에 pub를 쓰면 구조체는 공개되지만, 구조체의 필드는 비공개로 유지됩니다.  공개 여부는 각 필드마다 정할 수 있습니다.
- 열거형은 공개로 지정할 경우 모든 배리언트가 공개됩니다

```rust

use std::io::Result as IoResult; // alias 가능
use std::{cmp::Ordering, io}; // 중첩 use
use std::collections::*; // glob 연산 가능
```

### error

- Recoverable error: 발생해도 프로그램은 정상 실행 가능한 에러. Result 타입으로 처리하며 match, ? 연산자 등으로 에러 핸들링
- Unrecoverable error: 발생하면 바로 뻗는 에러. panic! 매크로를 통해 처리
- RUST_BACKTRACE=1 cargo run panic! 발생시 backtrace 하기 위해 환경변수를 일시적으로 바꿔보자.

### [collections](https://doc.rust-lang.org/std/collections/index.html)

Sequences: Vec, VecDeque(ring buffer), LinkedList
Maps: HashMap, BTreeMap
Sets: HashSet, BTreeSet
Misc: BinaryHeap(최대 힙)


### lifetime

- 라이프타임의 주목적은 댕글링 참조 (dangling reference) 방지입니다
- 라이프타임을 명시한다고 해서 참조자의 수명이 바뀌진 않습니다. 그보다는 여러 참조자에 대한 수명에 영향을 주지 않으면서 서로 간 수명의 관계가 어떻게 되는지에 대해 기술하는 것입니다

- 라이프타임 생략 규칙 (lifetime elision rules)
  - 첫 번째 규칙은, 컴파일러가 참조자인 매개변수 각각에게 라이프타임 매개변수를 할당한다는 것입니다. fn foo<'a, 'b>(x: &'a i32, y: &'b i32)처럼 매개변수가 두 개인 함수는 두 개의 개별 라이프타임 매개변수
  - 두 번째 규칙은, 만약 입력 라이프타임 매개변수가 딱 하나라면, 해당 라이프타임이 모든 출력 라이프타임에 대입된다는 것입니다: fn foo<'a>(x: &'a i32) -> &'a i32처럼 말이지요.
  - 세 번째 규칙은, 입력 라이프타임 매개변수가 여러 개인데, 그중 하나가 &self나 &mut self라면, 즉 메서드라면 self의 라이프타임이 모든 출력 라이프타임 매개변수에 대입됩니다. 

### closure

Rust의 클로저(closure) 트레잇에는 3가지가 있습니다:

Fn: 여러 번 호출 가능, 캡처된 값을 공유 참조로 빌림
FnMut: 여러 번 호출 가능, 캡처된 값을 가변 참조로 빌림
FnOnce: 한 번만 호출 가능, 캡처된 값을 소유권 이전(move)

```rust
// Fn - 불변 참조로 캡처
let closure = || println!("{a}"); // 참조로 캡처

// FnOnce - 소유권 이전 
let closure = move || println!("{s}"); // s의 소유권이 클로저로 이동

// FnMut - 가변 참조
let mut closure = || {
    x += 1; // 가변 참조로 캡처
    println!("{x}");
};
```


### 소유권 이전 주의

(1) for 루프의 기본 동작

```rust
for item in collection  // collection의 소유권을 가져감
```
- 컬렉션의 소유권을 가져가고
- 각 요소의 소유권도 item으로 이전됨

(2) 참조로 순회할 때

```rust
for item in &collection  // &collection을 순회
```
- item은 각 요소의 참조 타입이 됨 (&T)
- 컬렉션을 계속 사용할 수 있음

(3) 참조 패턴 매칭

```rust
for &item in &collection
```
- &item은 "이 참조를 해제해서 item에 값을 바인딩하겠다"는 의미
- item은 참조가 아닌 값 타입(T)이 됨


여러 예시들을 보자.

```rust
let vector = vec![String::from("hello"), String::from("world")];

for x in vector {  // vector의 소유권이 이전되고
    println!("{}", x);  // 각 String의 소유권도 x로 이전됩니다
}  // x가 스코프를 벗어나면서 String도 drop됩니다

// vector도 이미 소비되어 사용할 수 없고
// vector 안의 String들도 이미 move되어 접근할 수 없습니다
```


```rust
let vector = vec![1, 2, 3];

// &elem은 참조 패턴이며, elem은 i32 타입
for &elem in &vector {
    println!("{}", elem); // elem만 썼으므로 &를 떼어낸 것이라고 이해하면 편함
}

// item은 &i32 타입의 참조
for item in &vector {
    println!("{}", item);
}
```



### etc

- wrapping: 범위를 넘어가면 처음/끝으로 돌아감
- saturating: 범위를 넘어가면 최대/최소값에 고정됨
- as casting은 Use it exclusively for going from a smaller type to a larger type (업 캐스팅)
큰 것을 작은것으로 casting할 땐 TryFrom and TryInto 을 사용하라. (다운 캐스팅)
- derive 매크로는 재귀적으로 작동합니다. 즉, derive 매크로를 구조체에 적용하면 그 구조체의 모든 필드들도 해당 기능을 구현하고 있어야 합니다
- 제네릭은 암시적으로 Sized 트레이트 바운드가 설정됨. T : Sized  
- DST 타입의 레퍼런스는 FAT pointer 라서, usize * 2 의 크기를 가짐. x64기준 16바이트. 예를 들어 &str, &[T], trait object(dyn Trait) 는 DST.  
```rust
&[T]       // 슬라이스 - (ptr, len)
&str       // 문자열 슬라이스 - (ptr, len)
&dyn Trait // 트레이트 객체 - (ptr, vtable_ptr)
```
- Sized trait는 auto trait 이고, 별도 필수 메서드가 없는 Marker trait임.  
- Deref trait를 통해서 deref coercion 가능  
- match는 기본적으로 값을 소유(move)하려고 합니다
- bool은 1바이트, char는 4바이트. 
- rust에서 let a = b 의 경우 => Copy trait를 구현하지 않은 경우 move 혹은 Copy traits 구현에 의한 copy

- 일반적으로 많이 쓰이는 crates
  - reqwest/tokio/serde/thiserror/anyhow/clap/ratatui

- 문서 읽을 때...
  ```
    associated contants : i32::... 꼴
    methods : a.method(...) 꼴
    Trait Implementations : 구현된 트레이트
    Auto Trait Implementations : 컴파일러가 자동으로 구현해주는 트레이트들입니다
    Blanket Implementations : 특정 트레이드를 구현한 타입에 자동적으로 구현되는 트레이트
  ```