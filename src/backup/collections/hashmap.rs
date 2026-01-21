#[allow(dead_code)] //Esto ignorará el codigo no usado.
use std::collections::HashMap;

fn main() {
    // Collections:
    // vector
    // strings
    // hashmap

    // Hashmap: Estructura de datos que permite guardar valores y asociarlos a una key.
    let mut puntajes: HashMap<String, i32> = HashMap::new();
    puntajes.insert(String::from("Azul"), 20);
    puntajes.insert(String::from("Rojo"), 100);

    let pnts_azul = puntajes.get("Azul"); // Owmership and borrowing

    puntajes.entry(String::from("Azul")).or_insert(40);
    for (key, value) in &puntajes {
        println!("{} {}", key, value);
    }


}