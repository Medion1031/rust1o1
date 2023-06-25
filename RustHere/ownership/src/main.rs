mod chalange;

fn main() {
    //Shadowing with it we van override locked data
    let planet: &str = "Earth";
    {
        let planet: &str = "Mars"; // <- this is stored on the same memory as the base thing
        println!("{}", planet);
    }
    println!("{}", planet);

    //String
    let mut message = String::from("hello");  
    println!("{message}");
    message.push_str(" is home");
    println!("{message}");

    //Moving
    let mut outer_planet : String;
    {
        let inner_planet = String::from("Mars");
        println!("{inner_planet}");
        outer_planet = inner_planet.clone(); // this is copy
        outer_planet = inner_planet; // this is moving because string is stored on the heap
    }
    println!("{outer_planet}");

    let outer_planet : i32;
    {
        let inner_planet = 1;
        println!("{inner_planet}");
        outer_planet = inner_planet; // this is copy
    }
    println!("{outer_planet}");

    let mut rocket_fuel = String::from("Rocket fuel");
    process_fuel(&mut rocket_fuel);
    println!("{rocket_fuel}");

    let fuel_length = get_fuel_length(&rocket_fuel);
    println!("{fuel_length}");

    let new_fuel = produce_fuel();
    println!("{new_fuel}");

    //Slices
    let new_message = String::from("Greetings from Earth!");
    println!("{new_message}");

    let last_word = &new_message[15..];
    println!("{last_word}");

    let first_word = get_first_word(&new_message);
    println!("{first_word}");

    //testing chalange
    let challenge = chalange::trim_spaces("   valami    ");
    println!("{challenge}");

}

fn process_fuel(propellant : &mut String)
{
    propellant.push_str(" fuel");
    println!("{propellant}");
}

fn get_fuel_length(propellant: &String) -> usize
{
    let length = propellant.len();
    return length;
}

fn produce_fuel() -> String {
    let new_fuel = String::from("NewFuel");
    return new_fuel;
}

fn get_first_word(message: &str) -> &str {
    let bytes = message.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &message[..i];
        }
    }

    return message;
}