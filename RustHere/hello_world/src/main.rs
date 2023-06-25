fn main() {
    // normal variable operations
    let mut variable: f32 = 10.0;
    let divider = 3;
    println!("variable value is {}", variable);
    variable = variable / divider as f32;
    println!("variable value is {:.3}", variable);

    //bitwise operations
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);

    value = !value;
    println!("value is {:08b}", value); // NOT

    value = value & 0b1111_0111;
    println!("value is {:08b}", value); // AND
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("value is {:08b}", value); // OR

    value = value ^ 0b0101_0101;
    println!("value is {:08b}", value); //XOR

    value = value << 4;
    println!("value is {:08b}", value); // shift left

    value = value >> 4;
    println!("value is {:08b}", value); // shift right

    // test exercises
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("CORRECT!");

    // Arrays
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    println!("{}", letters[0]);

    let numbers: [i32; 5];
    numbers = [0; 5]; // all numbers in the array are 0

    let parking_lot =   [[1, 2, 3],
                                    [4, 5, 6]];
    let first_lot = parking_lot[0][0];
    println!("{}", first_lot);

    let garage = [[[0; 100]; 20]; 5];

    let stuff: (u8, f32, char) = (100, 3.2, 'c');
    let first_item = stuff.0;
    println!("{}", first_item);

    let (a, b, c) = stuff;
}
