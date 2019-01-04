
fn main() {
    let mut i: i32 = 0;
    loop {
        println!("i: {}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }
    println!("Outside i: {}", i);
    
    // Iterator
    // The .. operator is _Range_
    // Ranges are iretable types because they
    // implement the std::iter::IntoIterator _trait_.
    for xx in 0..10 {
        println!("xx: {}", xx);
    }
    
    i = 0;
    while i < 10 {
        println!("i: {}", i);
        i += 1;
    }
    println!("Outside i: {}", i);

    // Using a Vector with various values
    let numbers = vec![0, 4, -10];
    for i in 1..4 {
        for j in numbers.iter() {
            let _: &i32 = j;
            println!("i:{} j:{}", i, j);
        }
    }

    // Using into_iter() that converts the existing
    // value into an iterator by consuming the value
    // This means that the following will not work:
    // let n2 = vec![0, 4, -10];
    for i in 1..4 {
        let n2 = vec![0, 4, -10];
        for jj in n2.into_iter() {
            println!("i:{} j:{}", i, jj);
        }
    }
}
