#[allow(dead_code)] //Esto ignorará el codigo no usado.

// Option

fn main() {
    let nuevo = User{
        name: "Abdiel".to_string(),
        // age: Some(22),
        age: None,
    };

    let age = nuevo.getAge();
    // match age {
    //     Some(age) =>println!("Edad: {}", age),
    //     _ => (),
    // }
    if age != -1 {
        println!("Edad: {}", age);
    }

}

struct User {
    name: String,
    age: Option<i32>,
}

impl User {
    // fn getAge(&self) -> Option<i32> {
    //     self.age
    // }
    fn getAge(&self) -> i32{
         if self.age.is_none() {
            -1
        } else {
            self.age.unwrap()
        }
        // self.age.unwrap_or_default()
    }
}