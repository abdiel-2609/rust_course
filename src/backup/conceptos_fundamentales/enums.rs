struct User {
    name: String,
    email: String,
    activo: bool, 
    user_role: UserRole,
    website: WebSite,
}

// Enums = enumaration
enum UserRole {
    BASIC,
    ADMIN,
}

enum WebSite {
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
        website: WebSite::INSTAGRAM(String::from("www.instagram.com)"))
    };

    let acces = has_access(user.user_role);
}

fn has_access(user_role: UserRole) -> bool {
    match user_role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false,
    }
}

fn go_to_website(website: WebSite) -> bool {
    match website {
        WebSite::INSTAGRAM => true,// algo..
        WebSite::FACEBOOK => false,// algo..
    }
}