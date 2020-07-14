#[derive(Clone)]
struct Person {
    name: String,
    age: i32,
    height: i32,
    weight: i32,
}

fn person_create(name: String, age: i32, height: i32, weight: i32) -> Box<Person> {
    let who = Box::new(Person{name, age, height, weight});
    return who;
}

fn person_print(who: Box<Person>) -> () {
    println!("Name: {}\n", who.name);
    println!("\tAge: {}\n", who.age);
    println!("\tHeight: {}\n", who.height);
    println!("\tWeight: {}\n", who.weight);
}

fn main() {
    let mut joe: Box<Person> = person_create(String::from("Joe Alex"), 32, 64, 140);
    let mut frank: Box<Person> = person_create(String::from("Frank Blank"), 20, 72, 180);
    
    println!("Joe is at memory location {:p}:\n", &joe);
    person_print(joe.clone());

    println!("Frank is at memory location {:p}:\n", &frank);
    person_print(frank.clone());

    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    person_print(joe);

    frank.age += 20;
    frank.weight +=20;
    person_print(frank);
}
