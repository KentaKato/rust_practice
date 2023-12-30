use rand::Rng; // randライブラリからRngトレイトをインポート
use std::f64::consts::PI; // 円周率PIをインポート
use plotters::prelude::*; // plottersライブラリをインポート

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng(); // 乱数ジェネレータを初期化

    let num_data = 10;

    let mut x_train = Vec::new();
    let mut y_train = Vec::new();

    for _ in 0..num_data {
        let x = rng.gen_range(-PI..PI);
        x_train.push(x);
        y_train.push(x.sin());
    }

    // グラフを描画する画像ファイルを作成
    let root = BitMapBackend::new("images/plot.png", (640, 480)).into_drawing_area();

    // グラフの背景を白に設定
    root.fill(&WHITE)?;

    // グラフの描画領域を設定
    let mut chart = ChartBuilder::on(&root)
        .caption("y = sin(x)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-PI..PI, -1.0..1.0)?;

    for (x, y) in x_train.iter().zip(y_train.iter()) {
        chart.draw_series(PointSeries::of_element(
            vec![(*x, *y)],
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:.2}", coord.1), (10, 0), ("sans-serif", 15).into_font())
            },
        ))?;

    }

    Ok(())

}
