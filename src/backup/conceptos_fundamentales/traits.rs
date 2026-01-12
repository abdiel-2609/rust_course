#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main(){
    // Traits = rasgo
    let abdiel = Humano;
    println!("{}", abdiel.di_hola());

    let pelusa = Gato;
    println!("{}", pelusa.di_hola());
    println!("{}", Gato::idioma());
}

struct Humano;
struct Gato;

trait Hablar {
    fn di_hola(&self) -> String;
    fn idioma() -> String{
        "No tengo idioma".to_string()
    }
}

impl Hablar for Humano {
    fn di_hola(&self) -> String {
        "Hola amigos".to_string()
    }
    fn idioma() -> String {
        "Hablo humano".to_string()
    }
}

impl Hablar for Gato {
    fn di_hola(&self) -> String {
        "Miau!".to_string()
    }
    fn idioma() -> String {
        "Hablo gatuno ".to_string()
    }
}