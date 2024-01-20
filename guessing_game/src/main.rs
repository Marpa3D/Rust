// Guessing game

#![allow(unused)] // отключение предупреждений на время обучения

use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {

    println!("Игра \"Угадай число\"");

    let secret: i32 = rand::thread_rng().gen_range(1..101);

    println!("Введите Ваше число от 1 до 100:");

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка чтения");
    
        let guess: i32 = guess.trim().parse().expect("Ошибка. Пожалуйста, ввдите число!");
        println!("Вы загадали число: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Слишком маленькое число"),
            Ordering::Greater => println!("Слишком большое число"),
            Ordering::Equal => {
                println!("Вы выиграли!!!");
                break;
            }
        }
    }

    println!("\nДо свидания!\n");

}
