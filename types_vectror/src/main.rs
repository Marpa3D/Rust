// Композитные типы данных. Вектор

fn main() {

    // Вектор - набор однотипных данных с изменяемым размером
    let mut numbers_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Содержимое вектора: {:?}", numbers_vec);

    println!("Первый элемент вектора: {}", numbers_vec[0]);

    numbers_vec[0] = 8;
    println!("Первый элемент вектора изменен на число 8: {}", numbers_vec[0]);
    println!("Содержимое вектора: {:?}", numbers_vec);
}
