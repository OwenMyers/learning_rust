fn main() {

    // This is just a simple example of two differnt ways
    // you can initilize some basic variables. Just different 
    // sytax. Both valid.
    let some_u32_v: u32 = 4;
    let some_u32_v2 = 5u32;
    println!("some_u32_v: {}", some_u32_v);
    println!("some_u32_v2: {}", some_u32_v2);

    let s: String = String::from(format!("try str mult in: {}, {}",1,2));
    println!("{}",s);


}
