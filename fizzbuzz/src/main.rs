fn fizzbuzz (i: u32) -> String {
    if i % 3 == 0 && i % 5 == 0{
        return String::from("FizzBuzz");
    } else if i % 3 == 0{
        return String::from("Fizz");
    } else if i % 5 == 0{
        return String::from("Buzz");
    } else {
        return format!("{}", i);
    }
}

fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}
