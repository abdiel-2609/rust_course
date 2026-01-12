#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    // Traits = resgo

    let edad: Option<i32> = Some(20);
    edad.es_mayor_de_edad();
}

trait LicenciaConducir {
    fn es_mayor_de_edad(&self) -> bool;
}

impl LicenciaConducir for Option<i32> {
    fn es_mayor_de_edad(&self) -> bool {
        match self {
            Some(edad) => edad > &18,
            None => false
        }
    }
}