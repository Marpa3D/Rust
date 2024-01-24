// Композитные типы. Строки 2-х видов и работа с ними
fn main() {
    // 1. Статический тип(неизменяемый, длинна постоянна) &str (область памяти - стек)
    let string_static: &str = "Это неизменяемая строка";
    println!("Пример неизменяемойй строки &str = \"{}\"", string_static);

    // 2. Изменяемая строка String (область памяти - куча)
    let mut change_string: String = String::from("Это изменяемая строка");
    println!(
        "Пример изменяемой строки типа String = \"{}\"",
        change_string
    );

    // Методы работы со строками

    // Добавление символа к строке
    change_string.push('!');
    println!(
        "С помощью метода push() добавлен символ '!': \"{}\"",
        change_string
    );

    // Удаление последнего символа из строки
    change_string.pop();
    println!(
        "С помощью метода pop() удален символ '!': \"{}\"",
        change_string
    );

    // Добавить строку к строке
    change_string.push_str("! И это прекрасно!!!");
    println!(
        "С помощью метода push_str() добавлена строка: \"{}\"",
        change_string
    );

    // Распространенные операции со строками
    println!(
        "
    Пустая строка?: {},
    Длина строки: {} байта,
    Емкость строки в байтах: {},
    Есть ли подстрока 'прекрасно'в строке: {}",
        change_string.is_empty(),
        change_string.len(),
        change_string.capacity(),
        change_string.contains("прекрасно")
    );

    // Обрезка ненужных символов
    change_string.push_str("    ");
    println!(
        "
    Длина строки перед обрезкой: {} байт.
    Длина строки после обрезки: {} байт.
    ",
        change_string.len(),
        change_string.trim().len()
    );

    // Преобразование числа в строку
    let num = 8.14_f64;
    println!("Значение числа в виде строки: {}", num.to_string());
    println!("Является ли число строкой? {}", num.to_string() == "8.14");

    // Символ (тип char) в строку
    let some_char = 'S';
    println!("Значение символа в виде строки: {}", some_char.to_string());

    // Создание строки из текста (строкового литерала)
    let my_name = "Mark".to_string();
    println!("Из типа &str в тип String: {}", my_name);

    // Создание пустой строки
    let empty_str = String::new();
    println!("Длина пустой строки: {}", empty_str.len());

    // Форматирование строк

    // &str
    let s1 = "Mark";
    let s2 = "Evangelist Rust.";
    let res = format!("Имя: {}. Призвание: {}", s1, s2);
    println!(
        "Строка после применения макроса format!() для типа &str: \"{}\"",
        res
    );

    // String
    let s1 = String::from("Mark");
    let s2 = String::from("Evangelist Rust.");
    let res_str = format!("Имя: {}. Призвание: {}", s1, s2);
    println!(
        "Строка после применения макроса format!() для типа String: \"{}\"",
        res_str
    );
}
