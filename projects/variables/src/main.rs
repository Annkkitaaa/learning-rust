fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn main() {
    let mut x=0;
    println!("the value of x is : {x}");
    x=6;
    println!("the value of x is : {x}");
    let tup: (i32, f64, u8)= (500,6.4,1);
    let (x,_y,_z)=tup; //it was earlier showing error because y and z were completely not being used so putting an _ is telling the program that it was intentional.

    println!("the vale of x is:{x} ");

    let x = plus_one(5);

    println!("The value of x is: {x}");

    //variable scope
    println!("we need to learn rust this week ")

       {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    } 

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                    

     
}
