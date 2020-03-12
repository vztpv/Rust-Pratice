use std::io;

fn main() {
    println!("guess.. ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("cannot read");

    println!("input is {}", guess);
}
