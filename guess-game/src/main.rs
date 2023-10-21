use std::io;

fn main() {
    println!("guess game");

    let mut game = String::new();
    io::stdin()
        .read_line(&mut game)
        .expect("try again");
        
    println!("guess:{game}");
}