struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// create structs without names for the fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// allows for nice printing, like python!
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    println!("Hello, world!");

    let mut user1 = User{
        username: String::from("Winslow"),
        email: String::from("winslow@user.com"),
        sign_in_count: 300,
        active: true,
    };

    // can change with dot notation after instantiation
    user1.email = String::from("winslow22@user.com");

    let mut user2 = build_user(String::from("123@gmail.com"), String::from("myname"));

    let user3 = User {
        email : String::from("anotheremail@g.com"),
        username : String::from("name2"),
        ..user1
    };

    let black = Color(255, 255, 255);
    let white = Color(0, 0, 0);

    let rec1 = Rectangle{
        width: 32,
        height: 32,
    };



    
    println!("rect is {:?}", rec1);

    println!("rect area is {}", rec1.area());

    let rect2 = Rectangle{
        width: 40,
        height: 50,
    };

    println!("Can rect1 fit in rect2? {}", rect2.can_hold(&rec1));
    


}

fn build_user(email : String, username : String) -> User {
    User {
        email : email,
        username : username,
        sign_in_count : 1,
        active : true,
    
    }
}
