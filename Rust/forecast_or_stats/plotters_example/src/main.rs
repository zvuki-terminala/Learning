use plotters::prelude::*;

//Майн особый
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Обьект диаграммы
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    //Белая заливка
    root.fill(&WHITE)?;
    //Создаем обьект chart который вносит изменения в обьект диаграммы

    //СЕТКА

    let mut chart = ChartBuilder::on(&root)
        //Шрифт
        .caption("y=x^2", ("sans-serif, 50").into_font())
        //Поля
        .margin(5)
        //Отступ по x и y
        .x_label_area_size(30)
        .y_label_area_size(30)
        //Задаем оси
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    //Завершение конфигурации сетки с последующей отрисовкой
    chart.configure_mesh().draw()?;

    //ГРАФИК ТОЧКИ И ЛЕГЕНДА

    chart
    //Создаем серию точек, с заданным стилем и функцией
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        //Описание функции графика (для легенды)
        .label("y = x ^ 2")
        //Линия функции графика (для легенды)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    //ОТРИСОВКА ЛЕГЕНДЫ

    chart
        //Рисует легенду
        .configure_series_labels()
        //Фон легенды
        .background_style(&WHITE.mix(0.8))
        //Граница легенды
        .border_style(&BLACK)
        //Нарисовать
        .draw()?;

    //ОТРИСОВКА

    root.present()?;

    Ok(())
}