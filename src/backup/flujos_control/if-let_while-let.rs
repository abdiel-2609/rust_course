#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    //if-let
    let edad: Option<i32> = Some(20);
    match edad {
        Some(value) => println!("edad: {}", value),
        _ => (),
    }

    if let Some(value) = edad {
        print!("edad: {}", value);
    }

    // while-let 
    let mut mensajes_no_leidos = Some(2);

    loop {
        match mensajes_no_leidos {
            Some(value) => {
                if value > 0 {
                    println!("Tienes mensajes no leídos");
                    mensajes_no_leidos = Some(value - 1);
                } else {
                    println!("No hay mensajes nuevos");
                    mensajes_no_leidos = None;
                }
            }
            None => {
                break;
            }
        }
    }

    while let Some(value) = mensajes_no_leidos {
        if value > 0 {
            println!("Tienes mensajes no leídos");
            mensajes_no_leidos = Some(value - 1);
        } else {
            println!("No hay mensajes nuevos");
            mensajes_no_leidos = None;
        }
    }
}
