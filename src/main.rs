use std::io;

fn main() {
    println!("Tebak angkanya!");

    loop {
        println!("Silakan masukkan tebakanmu.");

        let guess = loop {
            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Gagal membaca baris input");

            match temp.trim().parse::<i32>() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Masukkan sebuah angka!");
                    continue;
                }
            }
        };

        if guess == -1 {
            println!("Permainan selesai..");
            break;
        }

        println!("Anda menebak {}", guess);
    }
}