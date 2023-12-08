// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer1 = square1(3);
    println!("The square of 3 is {}", answer1);
    let answer2 = square2(5);
    println!("the square of 5 is {}", answer2);
}

fn square1(num: i32) -> i32 {
    let sol = num * num;
    return sol; 
}

// Or

fn square2(num: i32) -> i32 {
    num * num
}
