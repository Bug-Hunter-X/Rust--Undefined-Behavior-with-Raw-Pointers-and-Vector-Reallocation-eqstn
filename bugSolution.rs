fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, use safe methods
    v[0] = 10; 
    println!("The first element is: {}", v[0]);
    // Or if you need to mutate multiple elements, use iterators
    for i in v.iter_mut() {
        *i *= 2; 
    }
    println!("The vector is: {:?}", v);
} 