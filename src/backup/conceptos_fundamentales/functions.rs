fn main() {
    mostrar_bienvenida();
    let nro = seleccion_numero(8);
    println!("El número es: {}", nro);

    let nro2 = {
        10
    };
    println!("El nuevo numero es: {}", nro2);

    // saludar_con_nombre("Julio".to_string());
    saludar_con_nombre("Julio");
}
    
fn mostrar_bienvenida() {
    println!("Bienvenidos a Rust!");
}

fn seleccion_numero(nro: i32) -> i32{
    nro
}

fn saludar_con_nombre(nombre: &str){
    println!("Hola {}", nombre)
}
