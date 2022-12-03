fn question1(val: Vec) {
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(param: &i8) {
    param + 2
}

fn main() {
    let mut vec = vec![1, 3, 5, 7];
    question1(vec);
    vec.push(15);
}
