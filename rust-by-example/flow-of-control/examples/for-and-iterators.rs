fn main() {
    use_iter();
    use_into_iter();
    use_iter_mut();
}

// iter -> borrows each element of the collection
fn use_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a restacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
}

// into_iter -> move each element of the collection
fn use_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a restacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // error[E0382]: borrow of moved value: `names`
    // println!("names: {:?}", names);
}

// iter_mut -> This mutably borrows each element of the collection
fn use_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a restacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
