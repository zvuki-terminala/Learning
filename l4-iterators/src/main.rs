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
    }//Переход к следующему шагу внутри итератора при помиощи continue

    let mut count = 0;
    let time_limit = Duration::new(5, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
