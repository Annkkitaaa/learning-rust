fn main() {
    let mut x=0;
    println!("the value of x is : {x}");
    x=6;
    println!("the value of x is : {x}");
    let tup: (i32, f64, u8)= (500,6.4,1);
    let (x,y,z) =tup;
    println!("the vale of x is:{x} ");
}
