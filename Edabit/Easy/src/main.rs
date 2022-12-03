// Take an array of integers (positive or negative or both) and return the sum of the absolute value of each element.
fn get_abs_sum(input: Vec<i32>) {
    let mut abs_sum: i32 = 0;

    for num in input.into_iter() {
        abs_sum += num.abs();
    }

    println!("{}", abs_sum);
}

// Create a function that counts the number of syllables a word has. Each syllable is separated with a dash -.
fn number_syllables(input: &str) {
    let split = input.split("-");

    let vec: Vec<&str> = split.collect();

    println!("{:?}", vec.len());
}

// Create a function that returns the string "Burp" with the amount of "r's" determined by the input parameters of the function.
fn long_burp(input: usize) {
    let burp: String = "Bu".to_owned();
    let rs = "r".repeat(input);
    let rs_str = Box::leak(rs.into_boxed_str());

    let output = burp + rs_str + "p";

    println!("{}", output)
}

// Create a function which validates whether a bridge is safe to walk on (i.e. has no gaps in it to fall through).
fn is_safe_bridge(input: String) {
    for c in input.chars() {
        if c.is_whitespace() {
            println!("false");
            return;
        }
    }

    println!("true")
}

fn main() {
    get_abs_sum(vec![2, -1, 4, 8, 10]);
    number_syllables("mon-u-men-tal");
    long_burp(5);
    is_safe_bridge(String::from("#### ##"));
}
