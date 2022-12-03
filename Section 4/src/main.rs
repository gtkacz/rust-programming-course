enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon
}

impl Shape {
    fn corners(&self) -> i8 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
            _ => 0
        }
    }
}

fn main() {
    println!("Hello, world!");
}
