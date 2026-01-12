#[allow(dead_code)] //Esto ignorará el codigo no usado.
#[derive(Default, Debug)]
struct User {
    name: String,
    email: String,
    activo: bool,
    user_role: UserRole,
    website: WebSite,
}

// Enums = enumaration
#[derive(Default, Debug)]
enum UserRole {
    #[default]
    BASIC,
    ADMIN,
}

#[derive(Default, Debug)]
enum WebSite {
    #[default]
    None,
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main() {
    let mut user = User {
        name: "Abdiel".to_string(),
        email: String::from("abdiel.alvarado@wabtec.com"),
        activo: true,
        user_role: UserRole::BASIC,
        website: WebSite::INSTAGRAM(String::from("www.instagram.com)")),
    };

    let user2 = User::default();
    println!("User 2: {:?}", user2);
}
