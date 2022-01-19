#[derive(Debug)]
struct Test {
  x: i64
}


fn main() {
    let test = Test { x: 5 };
        println!("{:?} months in a year.", test);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
}
