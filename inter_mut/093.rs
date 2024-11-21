use std::cell::Cell;
// interior mutability(내부 가변성) == 불변 참조(&)를 통해서도 내용을 변경할 수 있게 해주는 패턴


#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    on_sale: Cell<bool>
}

fn main() {

    // 불변임 (mut이 아님)
    let my_phone = PhoneModel {
        company_name: "YYY".to_string(),
        model_name: "awesome phone".to_string(),
        on_sale: Cell::new(true),
    };

    // 그러나 변경 가능
    my_phone.on_sale.set(false);

    println!("{:?}", my_phone);
}