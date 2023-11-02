fn main() {
    let penguin_data = "\
    common name,lenght (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
    
    let fields: Vec<_> = record                                         //Вектор - динамически расширяемая коллекция элементов "_" - печать типа
        .split(',')
        .map(|field| field.trim())
        .collect();
        if cfg! (debug_assertions) {                                        //DEBUG - блок, полезное дополнение к основному коду
            eprintln!("debug: {:?} -> {:?}",                                //println! - печать в stdout; eprintln! - печать в stderr
                record, fields);
        }

        let name = fields[0];
        if let Ok(lenght) = fields[1].parse::<f32>() { //Указание компилятору - на какой тип парсить обьект
            //"Попытаться разобрать field[1] в виде 32 разрядного числа с точкой, если успех - присвоить lenght"
            println!("{}, {}cm", name, lenght);
        }
    } 
}

//cargo run --release - запуск кода без печати вывода DEBUG - блоков