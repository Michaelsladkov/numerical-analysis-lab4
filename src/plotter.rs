use crate::points::Clearness::*;
use crate::points::*;
use plotters::prelude::*;

const IMAGE_WIDTH: u32 = 1024;
const IMAGE_HEIGHT: u32 = 720;
const OUTPUT_FILENAME: &str = "out.png";
pub fn draw_chart(
    original: (fn(f64) -> f64, &str),
    interpolated: Box<dyn Fn(f64) -> f64>,
    points: PointSet,
    a: f32,
    b: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUTPUT_FILENAME, (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;
    let graph_left_border = if a < 0f32 { 1.5 * a } else { 0.5 * a };
    let graph_right_border = if b < 0f32 { 0.5 * b } else { 1.5 * b };

    let original_prepared_set_info =
        create_set_with_info(original.0, a as f64, b as f64, IMAGE_WIDTH / 2, CLEAR);
    let original_prepared_set_info = match original_prepared_set_info {
        Ok(info) => info,
        Err(msg) => {
            return Err(msg)?;
        }
    };
    let original_prepared_set = original_prepared_set_info.0;

    let min = original_prepared_set_info.1;
    let max = original_prepared_set_info.2;
    let graph_low_border = if min < 0f32 { 1.1 * min } else { 0.9 * min };
    let graph_top_border = if max < 0f32 { 0.9 * max } else { 1.1 * max };
    let mut chart = ChartBuilder::on(&root)
        .caption("graph", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            graph_left_border..graph_right_border,
            graph_low_border..graph_top_border,
        )?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            original_prepared_set
                .iter()
                .map(|(x, y)| (*x as f32, *y as f32)),
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

    chart.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x as f32, *y as f32), 3, RED.filled())),
    )?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
