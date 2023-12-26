use std::vec;

fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());

    println!("{}", s);
    println!("{}", n);

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let items = vec![1,2,3,4,5,6,7,8,9,10];
    //let plus_one: Vec<_> = items.iter().map(|x| x + 1).collect();
    let plus_one: Vec<i32> = items.iter().map(|x: &i32| x + 1).collect();
    let sum_all: i32 = items.iter().map(|x: &i32| x + 1).sum();
    let avg_all: f32 = items.iter().map(|x: &i32| x + 1).sum::<i32>() as f32 / items.len() as f32;

    println!("{:?} {} {}", plus_one, sum_all, avg_all);


}