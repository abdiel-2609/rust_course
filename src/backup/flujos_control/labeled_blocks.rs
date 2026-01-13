#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    let algun_numero: Option<i32> = Some(200);

    // This block has been labeled, and has another scope
    let result:i32 = 'calcular_porcentajes: {
        'muchos_calculos: loop {
            let Some(num) = algun_numero else{
                break 'calcular_porcentajes 0;
            };
            if num > 100 {
                break 'muchos_calculos 100;
            } else {
                break 'calcular_porcentajes num;
            }
        }
    };
    println!("Resultado es: {}", result);
}
