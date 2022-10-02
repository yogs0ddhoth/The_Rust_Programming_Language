use generic_types::{
    largest,
    traits::{impl_article, impl_tweet, summarize_vec},
    lifetimes::{main as lifetime, structs::main as struct_lifetime},
    Point,
};
// Generic Type Parameters can be used without affecting performance
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest_i32 = largest::<i32>(&number_list);
    println!("The largest number is {}\n", largest_i32);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest_char = largest::<char>(&char_list);
    println!("The highest letter is {}\n", largest_char);

    let p1 = Point::new(1.5, 1.0, 2.0, 0.0);
    let p2 = Point::new("x", "y", "z", "Hiya Chuck, it's John.");
    let p3 = p1.mixup(p2);
    println!(
        "p: {{ x: {}, y: {}, z: {}, t: {}}}. distance from the vertex: {}\n",
        p3.x(),
        p3.y(),
        p3.z(),
        p3.t(),
        p3.distance_from_origin()
    );
    impl_tweet();
    summarize_vec();
    impl_article();

    lifetime();
    struct_lifetime();
}
