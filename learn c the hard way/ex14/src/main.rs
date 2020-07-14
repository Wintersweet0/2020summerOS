fn print_letters(arg: String) -> () {
    for ch in arg.chars() {
            println!("'{}' == {} ", ch, ch as i32);
    }
}

fn print_arguments(argv: Vec<String>) -> () {
    for arg in argv {
        print_letters(arg);
    }
}

fn main() {
    let mut argv = Vec::<String>::new();
    for arg in std::env::args() {
        argv.push(arg);
    }
    print_arguments(argv);
}