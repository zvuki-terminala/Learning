fn greet_world() {
    println!("Hello, world!");                             // ! - символ макроса автоопределения типа
    let southern_germany = "Hallo Welt";             // let - операция присваивания (привязки переменной к значению)
    let japan = "こんにちは世界";                       // Поддержка Unicode вшита в язык
    let regions = [southern_germany, japan];    // Массив
    for region in regions.iter() {                  // .iter() - функция итерации массива - итератор
        println!("{}", &region);                           // & - читаем значение переменной (атрибут чтения)
    }
}

fn main() {
    greet_world();                                         // Вызов функции
}

// Cargo не переваривает и ругается на верблюжий кейс, юзать snake