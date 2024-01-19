// CLI калькулятор
use std::io;

fn main(){

    println!("Добро пожаловать в Калькулятор");

    loop {
        println!("Введите Ваше выражение(напр. 2 + 2)");
        let input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения");
        let input = input.trim();

        if input == "exit" {
            println!("До свидания!)");
            break;
        }
        let result = calculate(input);
        println!("Результат вычисления: {}", result);

    }

    fn calculate(input: &str) -> f64 {

        0.0
    }

}