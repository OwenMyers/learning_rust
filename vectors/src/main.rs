fn main() {
    let mut v = vec![10,20,30];
    for i in &mut v {
        *i += 3;
    }

    for i in &v {
        println!("{}", i);
    }
}
