// Тестирование скорости увеличения счетчика Вашим компьютером
use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0); // Создание продолжительности - 1 секунда
    let start = Instant::now(); // Считывание времени по сиситемным часам

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("Время: {}", count);
}
