use std::io;

fn main() {
    println!("Tebak angkanya!");

    println!("Silakan tebak angkanya.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Gagal membaca baris input");

    println!("Kamu menebak: {}", guess);
}