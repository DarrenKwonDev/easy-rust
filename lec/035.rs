
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*; // enum variant를 접두사 없이 바로 사용 가능

    let happy_level = match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happy_level
}

fn main() {
    let my_mood = Mood::NotBad;
    let score = match_mood(&my_mood);
    println!("score: {}", score);
}