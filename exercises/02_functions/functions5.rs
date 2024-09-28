// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    let calc = num * num;
    let result: i32 = calc;
    
    return result
}

fn main() {
    let input = 9;
    let answer = square(input);
    println!("The square of {input} is {answer}");
}
