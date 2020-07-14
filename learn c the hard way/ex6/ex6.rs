fn main() {
    let distance: i32 = 100;
    let power: f32 = 2.345;
    let super_power: f64 = 56789.4532;
    let initial = 'A';
    let mut first_name = String::from("Zed");
    let last_name = String::from("Shaw");

    first_name.push('Z');

    println!("You are {} miles away.\n", distance);
    println!("You have {} levels of power.\n", power);
    println!("You have {} awesome super powers.\n", super_power);
    println!("I have an initial {}.\n", initial);
    println!("I have a first name {}.\n", first_name);
    println!("I have a last name {}.\n", last_name);
    println!("My whole name is {} {}. {}.\n", first_name, initial, last_name);
}