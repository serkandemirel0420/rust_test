fn main() {
    println!("Hello world!");

    #[derive(Debug)]
    enum Thing {
        String(String),
        Number(i32),
    }

    let mut u: Vec<Thing> = Vec::new(); // create empty vector of unsigned 8 bit int, can grow
    u.push(Thing::Number(2)); // append item to vector
    u.push(Thing::String(String::from("Serkan"))); // append item to vector
    // println!("{:?}", u);

    let vecter_iterator = u.iter();
    for elem in vecter_iterator {
        println!("{:?}", elem);
    }
}
