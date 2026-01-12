#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    // Closures: funcion que es definidia en linea (inline), como si fuera una lambda (arrow function javascript)
    //  | = pipe

    // let numero = sumar_uno(2);
    let sum = |nro: i32, nro2: i32| -> i32 { nro + nro2 };
    println!("El numero es: {}", sum(4, 5));

    let mut counter = 1;
    let mut incrementar = move || {
        counter += 1;
    };

    let variable = &counter; // Borrowing, pedir prestada
    incrementar();
    println!("{}", counter);
}
