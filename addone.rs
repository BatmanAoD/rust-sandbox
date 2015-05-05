fn main() {
    let x = Box::new(5);

    let x = add_one(x);

    println!("{}", x);
}

fn add_one(mut num: Box<i32>) -> Box<i32> {
    *num += 1;

    num
}
