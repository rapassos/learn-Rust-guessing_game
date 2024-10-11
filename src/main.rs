use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é: {secret_number}");

    loop {
        println!("Por favor, insira seu palpite.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        if guess.trim() == "q" {
            println!("Saindo do programa!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite é: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} é menor que o número secreto", guess.to_string()),
            Ordering::Greater => println!("{} Maior que o numero secreto", guess.to_string()),
            Ordering::Equal => {
                println!("Voce acertou, parabéns!");
                break;
            }
        }
    }
}
