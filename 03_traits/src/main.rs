use std::fmt;

trait TestTrait {
    fn inc(&mut self, value: isize);
    fn set_str(&self, value: &str);
    fn print(&self);
}

struct TestImpl {
    s: String,
    i: isize,
    os: Option<String>,
    oi: Option<isize>,
}

impl TestImpl {
    fn new(i_value: isize, s_value: &str) -> TestImpl {
        TestImpl {
            s: String::from(s_value),
            i: i_value,
            os: None,
            oi: None
        }
    }

    fn get_str(&self) -> String {
        let mut s: &str = "None";
        if self.os.is_some() {
            s = self.os.as_ref().unwrap();
        }
        let mut si: String = String::from("None");
        // how it looks like when 'match' is used ...
        match self.oi {
            Some(value) => {
                si = value.to_string();
            },
            _ => {}
        }
        // the if-variant
        // if self.oi.is_some() {
        //     si = self.oi.as_ref().unwrap().to_string();
        // }
        format!("TestImpl -> i: {}, s: {}, oi: {}, os: {}", self.i, self.s, si, s)

    }
}

impl TestTrait for TestImpl {
    fn inc(&mut self, value: isize) {
        self.i = self.i + value;
    }

    fn set_str(&self, value: &str) {

    }

    fn print(&self) {
        let s = self.get_str();
        println!("Trait.print: {}", s)
    }
}

impl fmt::Debug for TestImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.get_str();
        write!(f, "Debug: {}", s)
    }
}

fn print_trait<T: TestTrait>(trait_impl: &T) {
    trait_impl.print();
}

fn inc_trait<T: TestTrait>(trait_impl:&mut T, value: isize) {
    trait_impl.inc(value);
}


fn main() {

    let mut t1 = TestImpl::new(42, "Das ist ein Test - 1");
    let mut t2 = TestImpl::new(-42, "Das ist ein Test - 2");

    println!("t1: {:#?}", t1);
    println!("t2: {:#?}", t2);

    print_trait(&t1);
    print_trait(&t2);

    inc_trait(&mut t1, 10);
    inc_trait(&mut t2, -10);
    print_trait(&t1);
    print_trait(&t2);

}
