
#[derive(Debug)]
enum ThingsInTheSky {
    Sun,
    Starts
}

fn create_sky(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Starts
    }
}

fn check_sky(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("sun"),
        ThingsInTheSky::Starts => println!("stars")
    }
}

fn main() {
    let sky = create_sky(10);
    check_sky(&sky);

    println!("sky valid in this scope : {:?}", sky);
}