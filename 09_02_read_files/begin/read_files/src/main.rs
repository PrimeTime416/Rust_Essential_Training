use std::fs;

fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("The contents are:  {:}", contents);

    for line in contents.lines(){
        println!("Line is {}", line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("The contents are:  {:?}", contents)
}