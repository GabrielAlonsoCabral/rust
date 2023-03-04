mod takes_ownership_1;
fn main() {
    let x: i32 = 5;
    let y: i32 = x;

    println!("{}", x);

    let s1: String = String::from("hello");
    let s2: String = s1; // Borrowing s1 to s2 ( not shallow copy )
    let s3: String = s2.clone(); // s3 is clone of s2 and s2 keep having his ownership;

    //error-> s1 was borrowed to s2;
    //println!("{} world!", s1);

    println!("{} world!", s2);
    println!("{} world!", s3);

    let mut string: String = String::from("Hellow world");
    let hellow = &string[0..6];
    let world = &string[7..12];

    println!("{} {}", hellow, world)
}
