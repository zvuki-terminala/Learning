use std::time::{Duration, Instant};

fn main() {
    for _ in 0..10 {
        println!("Test for iter");
    }//Простой итератор с заданным количеством итераций

    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        println!("{}", item);
    }//Непроизводительный небезопасный итератор

    for n in 0..10 {
        if n % 2 == 0 {
            continue;
        }
        println!("Iterator skipping on num - {}", n);
    }//Пропуск оставшейся части цикла при помощи continue

    let mut count = 0;
    let time_limit = Duration::new(2, 0);
    let start = Instant::now();

    //Выполнение цикла пока не изменится состояние условия
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

    /* 
    while true {
        println!("Fuck loop");
    }
    //НЕ ИСПОЛЬЗОВАТЬ т.к. существует loop для бесконечных циклов
    */
    let mut iter2 = 0;
    loop {
        println!("Test loop");
        iter2 += 1;
        println!("{}", iter2);
        
        if iter2 == 100 {
            break;
        }
    }//Стандартный бесконечный цикл с прерыванием по выполнении условия

    'outer_heaven: for x in 0..10 {
        println!("Work outer iterator");
        println!("{}", x);
        for y in 0..10 {
            if y + x > 10 {
                break 'outer_heaven;
            }
        }
    }//Прерывание внешнего цикла при помощи метки и выполнения условия - метка замена goto, работает аналогично для continue 
}
