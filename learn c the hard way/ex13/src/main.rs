fn main() {
    let mut i=0;
    for arg in std::env::args() {
        println!("arg {}: {}\n", i, arg);
        i += 1;
    }

    i = 0;

    let s = vec![
        "California", "Oregon", 
        "Washington", "Texas"];
    for ss in s {
        println!("state {}: {}\n", i, ss);
        i += 1;
    }
}
