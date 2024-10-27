enum Star {
    BrownDwarf = 10,
    RedDwarf = 120,
    YellowStar = 150,
    RedGiant = 99,
    DeadStar    // 99 다음인 100으로 할당됨
}

fn main() {
    use Star::*;

    let vec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

    for star in vec {
        println!("{}", star as u32);
    }
}