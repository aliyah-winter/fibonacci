use std::io;

fn fib(n: u128) -> u128 {
  if n < 2 {
    n
  } else {
    fib(n - 1) + fib(n -2)
  }
}

fn main() {
  println!("Welcome to fibonacci, find the fibonacci number in its sequence");
  loop {
    println!("Enter a number:");

    let mut num: String = String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read line");

  let num: u128 = num.trim().parse().expect("Failed to read number.");

  for i in 0..=num {
    let result = fib(i);
    println!("{}", result)
  }
}

}
