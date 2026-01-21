#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    // Collections:
    // vector
    // strings
    // hashmap

    // Strings
    // String vs string slice
    // slice: referencia a una contigua secuencia de elemtos de un collection.

    // String y string son una coleccion de caracteres, especificamente u8's
    
    // String se guarda en el heap
    // string slide: stack, referencia al heap, string literal incrustados en el codigo binario

    let mut nombre_String =String::from("Abdiel");
    let nombre_str = "Abdiel"; // hardcode

    let mut nombre2 = nombre_str.to_string();
    nombre2.push('a');

    let nombre_string_a_str = &nombre_String[..nombre_String.len()];
    nombre_String.push('a');

    println!("nombre: {}", nombre_string_a_str);

}