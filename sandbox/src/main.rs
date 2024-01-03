use rand::Rng; // randライブラリからRngトレイトをインポート
use std::f64::consts::PI; // 円周率PIをインポート
use plotters::{prelude::*, coord::types::RangedCoordf64}; // plottersライブラリをインポート

fn sin_func(x: f64) -> f64 {
    let amp = 1.0;
    let omega = 1.0;
    let phase = 0.0;
    return amp * f64::sin(omega * x + phase);
}

fn draw_latent_func(
    chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    latent_f: fn(f64) -> f64,
    bounds: (f64, f64)) -> Result<(), Box<dyn std::error::Error>> {

    let step = 0.1;
    let range = (0..=((bounds.1 - bounds.0) / step).round() as usize)
        .map(|i| bounds.0 + i as f64 * step)
        .filter(|&x| x <= bounds.1);
    chart.draw_series(LineSeries::new(
        range.map(|x| (x, latent_f(x))),
        &BLUE
    ))?;

    return Ok(());
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng(); // 乱数ジェネレータを初期化

    let num_train = 10;

    let mut x_train = Vec::new();
    let mut y_train = Vec::new();

    for _ in 0..num_train {
        let x = rng.gen_range(-PI..PI);
        x_train.push(x);
        y_train.push(sin_func(x));
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

    // XY軸、グリッド線を描画
    chart.configure_mesh().draw()?;

    for (x, y) in x_train.iter().zip(y_train.iter()) {
        chart.draw_series(PointSeries::of_element(
            vec![(*x, *y)], // ベクトル (x, y) を作成
            5, // 点のサイズ
            ShapeStyle::from(&RED).filled(), // 点のスタイル
            &|coord, size, style| { // 無名関数（クロージャ）を作成。&で参照キャプチャを指定。
                EmptyElement::at(coord) // 点の座標を指定（空の要素を配置）
                    + Circle::new((0, 0), size, style) // 円を描画
                    + Text::new(format!("{:.2}", coord.1), (10, 0), ("sans-serif", 15).into_font()) // y の値を表示
            },
        ))?;
    }

    // sin関数を描画
    draw_latent_func(&mut chart, sin_func, (-PI, PI))?;

    // 描画が終わったら、画像ファイルを保存
    root.present()?;

    return Ok(());

}
