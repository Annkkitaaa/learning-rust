fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    another_function(5);
}

fn another_function(x: i32){
    println!("the value of x is {x}!");
}



fn plus_one(x: i32) -> i32 {
    x + 1
}