

fn main() {
    
    let v = vec![1, 2, 3];

    let _fourth = v.get(3).unwrap_or_else(|| {
        if v.get(0).is_some() {
            &v[0]
        } else {
            &0
        }
    });
}
