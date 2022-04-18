fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        interests: Vec<String>,
    }

    let mut marie = Person {
        name: String::from("Marie"),
        age: 31,
        interests: vec![
            String::from("Rust"),
            String::from("Python"),
            String::from("History"),
        ],
    };
    marie.interests.push(String::from("Astronomy"));
    println!("{:?}", marie);
}
