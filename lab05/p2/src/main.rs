#[derive(Debug)]


struct Point{
    x:usize,
    y:usize,
    character:u8
}

fn main(){
    let a = Point{x:0,y:0,character:65};
    println!("{:?}",a);
}