
struct Monster {
    health: i32
}

struct Wizard {}

struct Ranger {}

// trait에 함수 시그니처만 존재. impl 하는 구조체에서 알아서 정의해야 함.
trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster);
}

// trait에 구현한 함수. impl 하는 구조체에서 덮어쓸 수도 있음.
trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster) {
        opponent.health -= 10;
    }
}

// trait를 구현한 구조체
impl FightClose for Ranger {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 15;
    }
}

// override 안하고 그냥 기본을 쓰기로 함
impl FightFromDistance for Wizard {}

fn main() {
    let a = Wizard{};
    let b = Ranger{};
    let mut monster = Monster{health: 100};

    b.attack_with_sword(&mut monster);
    a.attack_with_bow(&mut monster);
    
    println!("{}", monster.health);
}