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

#[test]
fn fizzbuzz_test_divisores_tres () {
    assert_eq!(fizzbuzz (3), "Fizz");
    assert_eq!(fizzbuzz (6), "Fizz");
    assert_eq!(fizzbuzz (9), "Fizz");
    assert_eq!(fizzbuzz (12), "Fizz");
}

#[test]
fn fizzbuzz_test_divisores_cinco () {
    assert_eq!(fizzbuzz (5), "Buzz");
    assert_eq!(fizzbuzz (10), "Buzz");
    assert_eq!(fizzbuzz (20), "Buzz");
}

#[test]
fn fizzbuzz_test_divisores () {
    assert_eq!(fizzbuzz (90), "FizzBuzz");
    assert_eq!(fizzbuzz (75), "FizzBuzz");
    assert_eq!(fizzbuzz (60), "FizzBuzz");
}

fn main() {
    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}
