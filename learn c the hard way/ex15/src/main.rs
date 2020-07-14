fn main() {
    let person = vec![("Alan", 23), ("Frank", 43), ("Mary", 12), ("John", 89), ("Lisa", 2)];

    for (a, b) in person {
        println!("{} is {} years old", a, b);
    }
}
