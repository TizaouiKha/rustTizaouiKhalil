use std::{io};

fn main() {
    let mut input_x = String::new();
    let mut input_y = String::new();

    println!("Veuillez entrer la valeur de x");
    println!("Veuillez entrer la valeur de y");

    io::stdin().read_line(&mut input_x).expect("error");
    io::stdin().read_line(&mut input_y).expect("error");

    let result = meme_longueur(&input_x, &input_y);
    println!("resultat: {}", result);
}

fn meme_longueur(x: &String, y: &String)-> bool{
    x.len() == y.len() 
}
