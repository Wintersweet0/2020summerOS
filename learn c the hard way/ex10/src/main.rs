fn main() {
    let mut i = 0;
    let mut strings = Vec::<String>::new();
    for arg in std::env::args() {
        println!("{}", arg);
        strings.push(arg);
        i += 1;
    }
    if i != 2 {
        println!("ERROR: You need one argument.\n");
    }
    i = 0;
    for argv in strings[1].chars() {
        let letter = argv;
        match letter {
            'a'|'A' => println!("{}:'A'\n", i),
            'e'|'E' => println!("{}:'E'\n", i),
            'i'|'I' => println!("{}:'I'\n", i),
            'o'|'O' => println!("{}:'O'\n", i),
            'u'|'U' => println!("{}:'U'\n", i),
            'y'|'Y' => if i > 2 {println!("{}:'Y'\n", i);},
            _ => println!("{}: {} is not a vowel\n", i, letter),
        }
        i += 1;
    }
}
