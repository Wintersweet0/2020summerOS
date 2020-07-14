fn main() {
    let distance: i32 = 100;
    let power: f32 = 2.345;
    let super_power: f64 = 56789.4532;
    let initial = 'A';
    let mut first_name = String::from("Zed");
    let last_name = String::from("Shaw");

    first_name.push('Z');

    println!("You are {} miles away.\n", distance);
    println!("You have {:.06} levels of power.\n", power);
    println!("You have {:.07} awesome super powers.\n", super_power);
    println!("I have an initial {}.\n", initial);
    println!("I have a first name {}.\n", first_name);
    println!("I have a last name {}.\n", last_name);
    println!("My whole name is {} {}. {}.\n", first_name, initial, last_name);

    let bugs = 100i64;
    let bug_rate = 1.2f64;

    println!("You have {} bugs at the imaginary rate of {}.\n", bugs, bug_rate);

    let universe_of_defects = 1i64 * 1024i64 * 1024i64 * 1024i64;
    println!("The entire universe has {} bugs.\n", universe_of_defects);

    let expected_bugs: f64 = bugs as f64 * bug_rate;
    println!("You are expected to have {} bugs.\n", expected_bugs);

    let part_of_universe :f64 = expected_bugs / universe_of_defects as f64;
    println!("That is only a {:e} portion of the universe.\n", part_of_universe);
}