
#[derive(Debug)]
struct Try {
    x: u8,
    y: u8,
}

impl Try {
    fn try_add_one(&mut self) {
        self.x += 1;
    }
}

fn main() {
    let mut try: Try = Try{
        x: 1,
        y: 2,
    };

    println!("try struct before: {:?}", try);
    try.try_add_one();

    println!("try struct after: {:?}", try);
}
