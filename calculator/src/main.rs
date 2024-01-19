// CLI калькулятор
use std::io;
use regex::Regex;

fn main(){

    println!("Добро пожаловать в Калькулятор");

    loop {
        println!("Введите Ваше выражение(напр. 2 + 2)");
        let mut input = String::new();
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
        let res = Regex::new(r"([\d.]+)\s*([+-/*])\s*
([\d.]+)").unwrap();
        let captures = res.captures(input).unwrap();
        let num1: f64 = captures[1].parse().unwrap();
        let num2: f64 = captures[3].parse().unwrap();
        let opt = &captures[2];

        match opt {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => panic!("Неизвестная операция!"),
        }
    }

}