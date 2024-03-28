#[derive(Debug, Clone)] // Clone is added to allow cloning User instances
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32) -> Rectangle {
        Rectangle {
            width :size,
            height :size
        }
    }
}
fn main() {
    let mut user = User {
        email: String::from("rabeeb@gmail.com"),
        username: String::from("rabeeb"),
        sign_in_count: 1,
        active: true,
    };
    
    // Clone username to avoid moving it
    let name = user.username.clone();
    println!("Username = {}", name);
    
    user.username = String::from("Umer");

    let user2 = form(String::from("rabeeb@gmail.com"), String::from("rabeeb"));
    let user3 = User {
        email: String::from("umer@gmail.com"),
        username: String::from("UMER"),
        ..user2
    };
    println!("User = {:#?}", user3);
    
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
        
    let rectangle2 = Rectangle {
        width: 20,
        height: 40,
    };
        
    let rectangle3 = Rectangle {
        width: 40,
        height: 50,
    };

    let rectangle4 = Rectangle::square(25);

    // Uncomment and use the area function if needed
    println!("Area of rectangle = {}", area_rectangle(&rectangle));
    println!("Area of rectangle using method = {}", rectangle.area());
    println!("rectangle can hold = {}", rectangle.can_hold(&rectangle2));
    println!("rectangle can hold = {}", rectangle.can_hold(&rectangle3));
    println!("rectangle can hold = {:#?}", rectangle4);



}

// Corrected the parameter order and moved outside main
fn form(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

// Uncomment and define this function if you want to use it
fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
