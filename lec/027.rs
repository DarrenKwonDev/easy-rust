
// from, into traits
// 어떤 type은 특정한 traits를 implements 한다.
// OOP에서 어떤 클래스가 특정한 interface를 구현하는 것과 같은 꼴이다.

fn main() {
    let _my_name = String::from("David MacLead");
    let my_city:String = "Seoul".into();
    let _my_vec = Vec::from([1, 2, 3]);

    println!("{}", my_city);
    println!("{:?}", _my_vec);
}