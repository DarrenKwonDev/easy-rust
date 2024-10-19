fn main() {
    let a:u8 = 100;
    let b = 124; // i32로 추론되어야 하지만 코드 전체를 훑어보면 u8과 더해야하므로 u8로 추론됨
    let _res = a + b;
    /*
        혹은, 직접적으로 캐스팅을 할수도 있다.
     */
    let _res2 = a + b as u8;
}
