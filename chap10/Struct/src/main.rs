#[derive(Debug)]
struct Point <T> {
    x:T,
    y:T,
}

fn main() {
let integer = Point  {x:8 , y:6};
let float = Point  {x:7.5 , y:6.5};



    println!("{:#?}", integer);
    println!("{:#?}",float);

    
}
