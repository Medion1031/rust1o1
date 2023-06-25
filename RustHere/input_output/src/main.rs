use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    // command line arguments
    /*if env::args().len() <= 2 {
        println!("not enought argument");
        return;
    }

    for (index, argument) in env::args().enumerate()
    {
        println!("index: {index}, argument: {argument}");
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("{arg2}");

    //read from file
    let contents = fs::read_to_string("test.txt").unwrap();
    println!("contents: {contents}");

    for line in contents.lines()
    {
        println!("line: {line}");
    }

    let contents = fs::read("test.txt").unwrap();
    println!("contents: {:?}", contents);

    //write to file
    let mut example_string = String::new();
    example_string.push_str("This is the first line \n");
    example_string.push_str("This is the second line \n");
    example_string.push_str("This is the third line \n");

    fs::write("example_write.txt", example_string);

    let mut file = fs::OpenOptions::new().append(true).open("test.txt").unwrap();
    ile.write(b"\nPluto"); */

    //Challenge
    if env::args().len() <= 2 {
        println!("Not enough argument!");
        return;
    }
    let path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    let file_content = fs::read_to_string(path).unwrap();

    for (index, item) in file_content.lines().enumerate() {
        if item.trim() == name {
            println!("Name is found at {index}");
            break;
        }
    }
}
