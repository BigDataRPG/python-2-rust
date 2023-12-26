fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);
    println!("{}", v.len());
    println!("{}", v.capacity());
    println!("{}", v.is_empty());
    println!("{}", v.pop().unwrap());
    println!("{}", v.len());
    println!("{}", v.capacity());
    println!("{}", v.is_empty());
    println!("{:?}", v);
    println!("=====================");

    let mut v = vec![1, 2, 3];
    println!("{}", v.len());
    println!("{}", v.capacity());
    v[0] = 4;
    println!("{:?}", v);
    v.extend([5, 6, 7].iter().cloned());
    println!("{:?}", v);

    for i in &v {
        println!("{}", i);
    }

    println!("=====================");

    // Iterating over mutable references to elements
    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }
    println!("{:?}", v);
    println!("=====================");

    for i in v.iter_mut() {
        *i += 2;
        println!("{}", i);
    }
    println!("{:?}", v);
    println!("=====================");

    // Iterating with float over mutable references to elements
    // Convert i32 values to f32 with a new calculation
    let v_f32: Vec<f32> = v.iter().map(|&x| x as f32 * 1.5).collect();

    // Print the original and new vectors
    println!("Original Vector (i32): {:?}", v);
    println!("New Vector (f32): {:?}", v_f32);
    println!("=====================");
}