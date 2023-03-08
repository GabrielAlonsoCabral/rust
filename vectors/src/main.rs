use std::mem::size_of;

fn main() {
    let mut vector_i32: Vec<i32> = Vec::new();

    println!("{}", vector_i32.capacity());
    vector_i32.push(1);
    println!("{}", vector_i32.capacity());

    check_size_of();
}

fn check_size_of() {
    // Some primitives
    assert_eq!(4, size_of::<i32>());
    assert_eq!(8, size_of::<f64>());
    assert_eq!(0, size_of::<()>());

    // Some arrays
    assert_eq!(8, size_of::<[i32; 2]>());
    assert_eq!(12, size_of::<[i32; 3]>());
    assert_eq!(0, size_of::<[i32; 0]>());

    // Pointer size equality
    assert_eq!(size_of::<&i32>(), size_of::<*const i32>());
    assert_eq!(size_of::<&i32>(), size_of::<Box<i32>>());
    assert_eq!(size_of::<&i32>(), size_of::<Option<&i32>>());
    assert_eq!(size_of::<Box<i32>>(), size_of::<Option<Box<i32>>>());
}
