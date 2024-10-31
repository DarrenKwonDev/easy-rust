use std::collections::{BTreeMap, HashMap};

struct City {
    name: String,
    population: HashMap<u32, u32>, // year + pop
    pop_btreemap: BTreeMap<u32, u32>
}

fn main() {
    let mut tallin = City {
        name: "Tallin".to_string(),
        population: HashMap::new(),
        pop_btreemap: BTreeMap::new()
    };

    tallin.population.insert(1327, 3_250);
    tallin.population.insert(1851, 87_250);
    tallin.population.insert(2020, 427_250);
    
    tallin.pop_btreemap.insert(1327, 3_250);
    tallin.pop_btreemap.insert(1851, 87_250);
    tallin.pop_btreemap.insert(2020, 427_250);

    // hash map은 순서 보장이 없음. 순서를 원하면 BTreeMap을 사용할 것
    for (year, pop) in tallin.population {
        println!("{year} year, pop: {pop}");
    }

    println!("------------------");

    for (year, pop) in tallin.pop_btreemap {
        println!("{year} year, pop: {pop}");
    }
}