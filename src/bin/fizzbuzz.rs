fn main() {
    let fizz_buzz = |x| {
        if x % 15 == 0{
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x)
        }
    };
    // For loop play
    for i in 1..16 {
        fizz_buzz(i);
    }
    println!("------");
    // Iterator play
    (1..16).into_iter().for_each(fizz_buzz);
}