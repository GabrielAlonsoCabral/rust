fn main() {
    let s: String = String::from("hello");
    takes_ownership(s);
    // String s was ownershpped to takes_ownership function;
    // println!("{}", s); -> // it means that this code is not allowed to compile or run

    //Instead the above code do this;

    // In this case we just do a reference to s
    // It means just borrow the s String
    let x: String = String::from("hello");
    not_takes_ownership(&x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)

    //After execute this scope the some_string will be dropped
}

fn not_takes_ownership(some_string: &String) {
    println!("{}", some_string)
}
