//add two numbers together

fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

//add two and two together
fn main() {
    let a = 2;
    let b = 2;
    let c = add_two_numbers(a, b);
    println!("{} + {} = {}", a, b, c);
}
