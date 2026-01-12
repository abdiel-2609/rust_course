fn main() {
    let mut nombre: Option<String> = None;

    nombre = Some("Abdiel".to_string());
    match nombre {
        None => println!("Nombre no indicado"),
        Some(nombre) => println!("{}", nombre),
    }
}