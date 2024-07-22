use std::io;

fn main() {
    println!("Enter number:");

    let mut num_choice = String::new();
    io::stdin().read_line(&mut num_choice).expect("Failed to read line");
    let num_choice: u32 = num_choice.trim().parse().expect("Please enter a number");

    if num_choice == 0 {
        println!("0");
    } else if num_choice == 1 {
        println!("1");
    }

    let mut prev1 = 0;
    let mut prev2 = 1;
    let mut current = 0;

    for _ in 2..=num_choice {
        current = prev1 + prev2;
        prev1 = prev2;
        prev2 = current;
    }
    println!("{current}");
}
