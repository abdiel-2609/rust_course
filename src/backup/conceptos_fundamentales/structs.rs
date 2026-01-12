// Struct
struct Usuario {
    nombre: String,
    email: String,
    nacimiento: i32,
    activo: bool,
}

impl Usuario {
    fn edad(&self) -> i32 {
        let actual = 2025;
        actual - self.nacimiento
    }
}

fn main() {
    let user = Usuario {
        nombre: "Abdiel".to_string(),
        email: "abdiel.alvarado@wabtec.com".to_string(),
        nacimiento: 2003,
        activo: true,
    };
    
    let edad = user.edad();

    println!("Usuario {}, edad {}, ", user.nombre, user.edad());

    let user1 = nuevo_usuario(String::from("Abdiel"), String::from("abdiel.alvarado@wabtec.com"));

    let user2 = Usuario{
        nombre: "Saul".to_string(),
        email: "saam.0926@gmail.com".to_string(),
        ..user1
    };

    // Tuple structs

    struct Point(i32, i32, i32);
    
    let point_a = Point(1,2,3);

}

fn nuevo_usuario(nombre: String, email: String) -> Usuario {
    return Usuario {
        nombre: nombre,
        email: email,
        nacimiento: 100,
        activo: true,
    };
}
