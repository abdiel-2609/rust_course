#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    // Collections:
    // vector
    // strings
    // hashmap

    // Vector: permite guardar valores uno a uno al lado de otro en la memoria - tienen que ser todos del mismo tipo de datos
    let v: Vec<i32> = Vec::new(); // Forma 1
    algo();
}

fn algo() {
    let mut v = vec![1, 2, 3]; // Forma 2

    v.push(9);
    v.push(100);

    // let tercer = v.get(100);
    // if tercer.is_some(){

    //     println!("valor elemento: {}", tercer.unwrap());
    // }

    match v.get(2) {
        Some(valor ) => println!("valor elemento: {}", valor),
        None => ()
    }

    for i in &v {
        println!("Valor: {}", i);
    }
    
    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("Valor: {}", i);
    }

    enum Mensaje {
        TEXTO(String),
        ERROR(i32),
    }

    let mensajes = vec![Mensaje::TEXTO("Hola".to_string()), Mensaje::ERROR(404)];
    for m in &mensajes {
        match m {
            Mensaje::TEXTO(texto) => println!("valor: {}", texto),
            Mensaje::ERROR(codigo_error) => print!("valor: {}", codigo_error),
        }
    }

}
