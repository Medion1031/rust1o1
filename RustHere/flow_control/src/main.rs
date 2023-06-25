fn main() {
    // if statements
    let a = 32;
    let a_is_even = if a % 2 == 0 {true} else {false};

    if a < 100 && a_is_even {
        println!("a is less than 100 and is even");
    }

    //loops
    let mut count = 0;

    let res = loop {
        count += 1;
        println!("count is {}", count);

        if count == 10 {
            break count * 10;
        }
    };

    println!("The result is {}", res);
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    count = 0;

    while count < letters.len() {
        println!("current letter is {}", letters[count]);
        count += 1;
    }

    for (index, item) in letters.iter().enumerate() {
        println!("current letter is {} and it is the {} index", item, index);
    }

    for(index, item) in letters.iter().enumerate() {
        for _i in 0..index+1 {
            print!("{}", item);
        }
        println!();
    }

    //Challenge
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max : i32 = 0;
    let mut min : i32 = 0;
    let mean : f64;

    let mut sum : i32 = 0;

    for &num in numbers.iter() {
        sum += num;

        if num > max {
            max = num;
        }

        if num < min {
            min = num;
        }
    }
    mean = sum as f64 / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Success!");

}
