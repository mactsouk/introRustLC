fn main() {
    let mut v = vec![2, 3, -2, 2];
    let some_val = 2;
    v.retain(|&val| val != some_val);
    println!("{:?}", v);
}
