// Композитные типы. Кортеж и массив
fn main() {
    // Кортеж. Коллекция фиксированного размера элементов одного или разных типов.
    let my_info = ("Зарплата", 380_000);
    println!("{}, {}", my_info.0, my_info.1);

    // Деструкция (распаковывание) кортежа
    let (salary, slalary_value) = my_info;
    println!("{0} = {1}", &salary, slalary_value);

    // Кортеж в кортеже
    let tupl_value = (1, 8.0, (4, 8), "ПРивет!");
    let res_value = tupl_value.2 .1;
    println!(
        "Значение 2-го эелемента (вложенный кортеж, индекс 1) по индексу 2 = {}",
        res_value
    );

    // Пустой кортеж. Unit
    let unit_value = ();
    println!("Значение пустого кортежа: {:?}", unit_value);
}
