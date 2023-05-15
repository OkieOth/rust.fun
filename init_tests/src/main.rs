
struct Person {
    name: String,
    age: u32,
}

struct Person2 {
    name: Option<String>,
    age: Option<u32>,
}

fn main() {
    let i: i32 = 0;
    let b: bool = false;
    let p = Person {
        name: String::from("test"),
        age: 0,
    };

    let p2: Person2;

    // uninitialized scalares seem to create compile errors :-/
    println!("Fresh init int value: {}", i);
    println!("Fresh init bool value: {}", b);
    println!("Fresh init struct value: p.name={}, p.age={}", p.name, p.age);
    // doesn't work yet
    //println!("Fresh init optional struct value: p2.name={}, p2.age={}", p2.name, p2.age);
}
