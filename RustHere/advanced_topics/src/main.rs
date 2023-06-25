fn main() {
    say_hello();
    say_a_number(square(-4).1);
    say_the_sum(3, 2);

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn say_hello()
{
    println!("Hello, world!");
}

fn say_a_number(number: i32)
{
    println!("The number is {}", number);
}

fn say_the_sum(first: u8, second: u8)
{
    println!("The sum is {}", first + second);
}

fn square(x: i32) -> (i32, i32)
{
    return (x, x * x);
}

fn celsius_to_fahrenheit(temperature: f64) -> f64
{
    return (temperature * 1.8) + 32.0;
}