#[allow(dead_code)] //Esto ignorará el codigo no usado.
use std::fmt;

//Trait Display
struct User {
    nombre: String,
    edad: i32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} ({})", self.nombre, self.edad)
    }
}
fn main() {
    let usuario = User {
        nombre: "Abdiel Alvarado".to_string(),
        edad: 20,
    };

    println!("{}", usuario);
}
