#[allow(dead_code)] //Esto ignorará el codigo no usado.

fn main() {
    // Iterators
    // let s = [1, 2, 3];
    // for i in s.iter() {
    //     println!("{}", i + 1);
    // }

    let mut c = Counter::new();
    c.next();
    c.next();
    c.next();
    c.next();
    c.next();
    let c2 = c.next();
    match c2 {
        Some(i) => println!("{}", i),
        None => println!("Llegó al final"),
    }
}

struct Counter {
    count: i32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 5 -> 0

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
