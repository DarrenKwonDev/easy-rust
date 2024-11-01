use std::collections::HashMap;


fn main() {
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("female", 0),
        ("male", 3),
        ("male", 4),
    ];

    let mut survey_hash = HashMap::new();
    
    for (gender, score) in data {
        // item.0 이 없으면 Vec::new 로 생성.
        // item.0이 있거나 없나 Vec의 존재는 확정적이므로 item.1을 push 가능
        survey_hash.entry(gender)
            .or_insert(Vec::new())
            .push(score);
    }

    for (male_or_female, num) in survey_hash {
        println!("{:?} {:?}", male_or_female, num);
    }


}