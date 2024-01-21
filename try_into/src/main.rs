// Метод try_into() как замена ключегого слова as, когда это допустимо
use std::convert::TryInto;

fn main() {

    let x = 18_i32;
    let y = 122_u16;
    let y_ = y.try_into()
                .unwrap();

    if x < y_ {
        println!("18 меньше 122");
    }    
}
