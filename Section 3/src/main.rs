struct Car {
    mpg: i16,
    color: String,
    top_speed: i16,
}

impl Car {
    fn set_mpg(&mut self, new_value: i16) {
        self.mpg = new_value;
    }
    
    fn set_color(&mut self, new_value: String) {
        self.color = new_value;
    }

    fn set_top_speed(&mut self, new_value: i16) {
        self.top_speed = new_value;
    }
}

fn main() {
    let mut bmw = Car{mpg: 10, color: String::from("black"), top_speed: 120};
    println!("{} {} {}", bmw.mpg, bmw.color, bmw.top_speed);
    
    bmw.set_mpg(20);
    bmw.set_color(String::from("white"));
    bmw.set_top_speed(110);
    println!("{} {} {}", bmw.mpg, bmw.color, bmw.top_speed);
}
