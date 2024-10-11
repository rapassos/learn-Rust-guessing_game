use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Criação e preenchimento numérico aleatório da variável secret_number

    //println!("O número secreto é: {secret_number}");

    loop {
        println!("Por favor, insira seu palpite ou 'q' para sair.");

        let mut guess = String::new();

        io::stdin() // Entrada do usuário e gravação na variável guess
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        if guess.trim() == "q" { // Criação da opção de saída com o caractere 'q'
            println!("Saindo do programa!");
            break;
        }

        let guess: u32 = match guess.trim().parse() { // Validação de valor numérico na variável
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite é: {}", guess);

        match guess.cmp(&secret_number) { // Comparando o palpite com o número secreto
            Ordering::Less => println!("{} é menor que o número secreto", guess.to_string()),
            Ordering::Greater => println!("{} Maior que o numero secreto", guess.to_string()),
            Ordering::Equal => {
                println!("Voce acertou, parabéns!");
                break;
            }
        }
    }
}
