fn plus_one(x: i32) -> i32 {
    x + 1
}
fn main() {
    let mut x=0;
    println!("the value of x is : {x}");
    x=6;
    println!("the value of x is : {x}");
    let tup: (i32, f64, u8)= (500,6.4,1);
    let (x,_y,_z)=tup;
    println!("the vale of x is:{x} ");

    let x = plus_one(5);

    println!("The value of x is: {x}");
     
}
