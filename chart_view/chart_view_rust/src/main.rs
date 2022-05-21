use plotters::prelude::*;

mod testdata;


fn main() {
    println!("Hello, world!");
    let xbar_chart=testdata::control_chart_data::testdata_xbar_chart();
    let r_chart=testdata::control_chart_data::testdata_r_chart();
    let root_area = BitMapBackend::new("images/2.5.png", (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Line Plot Demo", ("DejaVuSerif", 40))
    .build_cartesian_2d(-10..10, 0..100)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  ctx.draw_series(
    LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)
  ).unwrap();


    
}
 