fn print_size(size: usize) {
    println!("Size is {}", size);
}

fn take_str(s: &str) {
    // Q: println can only take a format string literal followed by args...?
    println!("{}", s);
}

fn main() {
    let str_vec: Vec<String> =
        ["one", "two", "three", "uno", "dos", "tres"].iter().map(|&s| s.into()).collect();
    if let Some(subvec) = str_vec.get(0..3) {
        for s in subvec {
            take_str(&s);
        }
    }
    let foo = Option::Some ( ["foo".to_string()] );
    for f in foo {
        take_str(&f);
    }
    // for s in str_vec.get(0..3) {
    //     print_size(s.len());
    //     take_str(&s); // Type mismatch: found type `&&[std::string::String]`
    // }
}
