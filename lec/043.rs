
struct Item {
    number: u8
}

impl Item {
    fn compare_number(&self, other_num: u8) {
        println!("is same? {}", self.number == other_num);
    }
}


fn main() {
    let item = Item{number: 10};
    let refer_item = &item;
    let double_refer = &refer_item; // &&Item

    // dot operator는 알아서 필요한 만큼 dereferencing 한다.
    refer_item.compare_number(10);

    // Rust의 점 연산자(.)는 자동으로 필요한 만큼의 참조 해제를 수행합니다.
    // double_refer는 &&Item 타입이지만 자동으로 한 번 더 참조 해제되어 &Item으로 변환됨
    double_refer.compare_number(10);

    /*
    만약 자동 참조 해제가 없었다면, 아래와 같이 작성해야 했을 것입니다:
    (*refer_item).compare_number(10);
    (**double_refer).compare_number(10);
    */
}