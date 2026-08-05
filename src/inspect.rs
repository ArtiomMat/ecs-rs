pub trait Inspect {
    fn inspect(&self) -> String;
}

impl Inspect for String {
    fn inspect(&self) -> String {
        self.clone()
    }
}

impl Inspect for u32 {
    fn inspect(&self) -> String {
        self.to_string()
    }
}

impl Inspect for f32 {
    fn inspect(&self) -> String {
        self.to_string()
    }
}

impl Inspect for bool {
    fn inspect(&self) -> String {
        self.to_string()
    }
}

impl<A, B> Inspect for (A, B) where A: Inspect, B: Inspect {
    fn inspect(&self) -> String {
        format!("({}, {})", self.0.inspect(), self.1.inspect())
    }
}

impl<A, B, C> Inspect for (A, B, C) where A: Inspect, B: Inspect, C: Inspect {
    fn inspect(&self) -> String {
        format!("({}, {}, {})", self.0.inspect(), self.1.inspect(), self.2.inspect())
    }
}

pub fn print_with_inspect<T: Inspect>(val: T) {
    println!("{}", val.inspect());
}