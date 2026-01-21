#[allow(dead_code)] //Esto ignorará el codigo no usado.
use std::collections::HashSet;

fn main() {
    // Collections:
    // vector
    // strings
    // hashmap

    // Hashset
    // let user_ids: HashSet<i32> = vec![11, 123, 443];
    let mut  user_ids: HashSet<i32> = HashSet::new();

    user_ids.insert(100);
    user_ids.insert(900);
    user_ids.insert(2);

    
    
    // Union: obtener los elementos unicos entre 2 sets
    // difference: obtener los elementos que estan en el primer set, y no en el otro
    // intersection: obtener solo los elementos que estan en ambos sets.
    // symetric_difference: obtener todos los elementos que estan en un set, o en el otro, pero no en ambos.
    
    let mut backup_users = HashSet::new();
    backup_users.insert(100);
    backup_users.insert(9);
    backup_users.insert(23);

    for id in user_ids.difference(&backup_users){
        println!("{}",id);
    }

}