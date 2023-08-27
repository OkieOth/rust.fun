
trait Callback {
    fn call(&mut self, msg: String);
}

struct A {
    name: String,
    callCount: usize,
}

impl A {
    fn new(name: String) -> A {
        A {
            name: name,
            callCount: 0,
        }
    }
}

impl Callback for A {
    fn call(&mut self, msg: String) {
        self.callCount += 1;
        println!("[{}] I was called: callCount={}\n message: {}", self.name, self.callCount, msg);
    }
}

fn main() {
    println!("Hello, world ...");
    let mut a: A = A::new("first".to_string());
    let mut b: A = A::new("second".to_string());
    let mut c: A = A::new("third".to_string());

    for _ in 0 .. 10 {
        a.call("a is called".to_string());
        b.call("  b is called".to_string());
        c.call("    c is called".to_string());
    }

    println!("Object '{}' was {} times called.", a.name, a.callCount);
}
