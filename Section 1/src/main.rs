fn concat_string(argument: &str) {
    let suffix_str: &str = "World";
    println!("{} {}", argument, suffix_str);
}

fn control_flow(argument: i32) {
    if argument == 1 {
        println!("The value is one");
    } else if argument < 25 {
        println!("The value is less than 25")
    } else if argument > 50 {
        println!("The value is greater than 50")
    } else {
        println!("The value is greater than 25 but less than 50")
    }
}

fn main() {
    // Question 1
    let val1: i32 = 5;
    let val2: i32 = 2;
    let ans: i32 = val1 % val2;
    println!("{}", ans);

    // Question 2
    let mut vec = vec![2, 4, 6, 8, 10];
    println!("{:?}", vec);
    vec.pop();
    vec.push(12);
    println!("{:?}", vec);

    // Question 3
    let init_string = "Hello";
    concat_string(init_string);
    control_flow(5);
}
