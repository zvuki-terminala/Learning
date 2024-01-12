extern crate rand;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use arima::{estimate, sim};

fn main() {
    let mut rng: StdRng = SeedableRng::from_seed([100; 32]);//Инициализация диапазона значений
    let normal = Normal::new(10.0, 2.0).unwrap();//Нормализация распределения шума
    //Симулирование временного ряда 
    let ts = sim::arima_sim(
        1000,//Число сэмплов
        Some(&[0.7, 0.2]),//Параметры авторегрессии
        Some(&[0.4]),//Параметр скользящего среднего
        0,//Разность
        &|mut rng| { normal.sample(&mut rng) },//Функция шума(?)
        &mut rng//Диапазон (?)
    ).unwrap();

    let coef = estimate::fit(&ts, 2, 0, 1).unwrap();//Параметры оценки авторегрессии

    println!("Параметр оценки: {:?}", coef);
}