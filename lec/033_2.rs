use std::mem::size_of;

// 기본 정렬
#[derive(Debug)]
struct DataDefault {
    a: u8,
    b: u32,
    c: u16
}

// C 스타일 정렬
// C의 자연 정렬과 유사
#[repr(C)]
#[derive(Debug)]
struct DataC {
    a: u8,
    b: u32,
    c: u16
}

// 패킹된 정렬 (빽빽하게)
// C의 #pragma pack(1)과 유사
#[repr(packed)]
#[derive(Debug)]
struct DataPacked {
    a: u8,
    b: u32,
    c: u16
}

// 8바이트 정렬
// C의 #pragma pack(8)과 유사
#[repr(align(8))]
#[derive(Debug)]
struct DataAlign8 {
    a: u8,
    b: u32,
    c: u16
}

fn main() {

    /*
    DataDefault: Rust의 기본 정렬 규칙 사용
    DataC: C 스타일의 정렬 사용
    DataPacked: 패딩 없이 최소 크기로 정렬
    DataAlign8: 8바이트 단위로 정렬
    */
    println!("기본 정렬: {} bytes", size_of::<DataDefault>());
    println!("C 정렬: {} bytes", size_of::<DataC>());
    println!("Packed 정렬: {} bytes", size_of::<DataPacked>());
    println!("8바이트 정렬: {} bytes", size_of::<DataAlign8>());
}