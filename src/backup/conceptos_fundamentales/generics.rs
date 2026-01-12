#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main(){
    // Generics
    let algo: Option<i32> = Option::Some(32);

    let pointA = Point{
        x:2.0,
        y: 12.0,
    };
    let pointB = Point{
        x: 5.0,
        y: 12.0,
    };

    calcular_area(pointA, pointB);
}

struct Point<T> {
    x: T,
    y: T,
}

fn calcular_area<T>(pointA: Point<T>, pointB: Point<T>) {
// todo
}