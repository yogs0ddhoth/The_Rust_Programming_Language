struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }
}

fn main() {
    implement_user();

    // tuple struct implementation
    let point = Point(0, 0, 0);
    let (x, y, z) = (point.0, point.1, point.2);

    println!("Coordinates: {x}, {y}, {z}\n");

    implement_rectangle();
}
// ------------------------ Chapter 5.1 ------------------------ //
fn implement_user() {
    // create instance of User using the factory function
    let mut user1 = build_user(String::from("asdf@mail.com"), String::from("_"));
    print_user(&user1);

    // update user1
    user1.username = String::from("asdf");
    print_user(&user1);

    // create new User instance
    let user2 = User {
        email: String::from("qwer@mail.com"),
        ..user1 // user1.username ownership moved to user2.username
                // user1.active & user1.sign_in_count copied to user2

                // username: user1.username.clone(), would deeply copy user1.username, if user1 is still needed
    };
    print_user(&user2);
    // user1 can no longer be used, as username was moved to user2
    // print_user(&user1); will cause panic
}
// User factory
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn print_user(user: &User) {
    println!(
        "User: {},\n at {},\n active: {},\n sign in count: {}!\n",
        user.username, user.email, user.active, user.sign_in_count
    );
}
// ------------------------ Chapter 5.2-3 ------------------------ //
fn implement_rectangle() {
    let scale = 10;

    let rect1 = Rectangle {
        width: dbg!(3 * scale), // dbg! helps log expressions
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 1 * scale,
    };
    let rect3 = Rectangle {
        width: 6 * scale,
        height: 45,
    };

    println!("rect1: {:#?}\n", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
