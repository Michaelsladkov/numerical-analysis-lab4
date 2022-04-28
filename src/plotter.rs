use plotters::prelude::*;

pub fn draw_chart(
    original: (fn(f64) -> f64, &str),
    interpolated: Box<dyn Fn(f64) -> f64>,
    points: Vec<(f64, f64)>,
    a: f32,
    b: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (1024, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let graph_left_border = if a < 0f32 {1.5 * a} else {0.5 * a};
    let graph_right_border = if b < 0f32 {0.5 * b} else {1.5 * b};
    let mut chart = ChartBuilder::on(&root)
        .caption("graph", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(graph_left_border..graph_right_border, -10f32..20f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            ((a * 100.0) as i32..=(b * 100.0) as i32)
                .map(|x| x as f32 / (5.0 * 50.0))
                .map(|x| (x, original.0(x as f64) as f32)),
            &BLUE,
        ))?
        .label(original.1)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart
        .draw_series(LineSeries::new(
            ((a * 100.0) as i32..=(b * 100.0) as i32)
                .map(|x| x as f32 / (100.0))
                .map(|x| (x, interpolated(x as f64) as f32)),
            &RED,
        ))?
        .label("interpolated")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
