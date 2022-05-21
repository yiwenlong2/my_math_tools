use chart::ControlChart;
use plotters::{element::Drawable, prelude::*};

const OUT_FILE_NAME: &'static str = "plotters-doc-data/sample.svg";

pub fn control_chart_drawing_svg(cc:&ControlChart,chart_title:&String){

    let root_area = SVGBackend::new(OUT_FILE_NAME, (800, 600)).into_drawing_area();
    
    root_area.fill(&WHITE).unwrap();
    //root_area.titled("Control Chart", ("Hack", 40)).unwrap();

    let mut chart=ChartBuilder::on(&root_area)
        .caption(chart_title, ("Hack",30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..(cc.data.len()+3), cc.lcl*1.2..cc.ucl*1.2)
        .unwrap();
    
    chart.configure_mesh().draw().unwrap();
    

    chart.draw_series(LineSeries::new((0..cc.data.len()).map(|x| (x, *cc.data.get(x).unwrap())), &BLUE)).unwrap();
    chart.draw_series(LineSeries::new((0..cc.data.len()).map(|x| (x, cc.ucl)), &RED)).unwrap();
    chart.draw_series(LineSeries::new((0..cc.data.len()).map(|x| (x, cc.lcl)), &RED)).unwrap();
    chart.draw_series(LineSeries::new((0..cc.data.len()).map(|x| (x, cc.ml)), &GREEN)).unwrap();

    let ucl_string=format!("{}:{}","UCL".to_string(),&cc.ucl.to_string());
    let lcl_string=format!("{}:{}","LCL".to_string(),&cc.lcl.to_string());
    let ml_string=format!("{}:{}","Mean".to_string(),&cc.ml.to_string());

    let text_ucl=Text::new(ucl_string, (680,110), ("Hack",20));
    let text_lcl=Text::new(lcl_string, (680,550), ("Hack",20));
    let text_ml=Text::new(ml_string, (680,350), ("Hack",20));
    
    root_area.draw(&text_ucl).unwrap();
    root_area.draw(&text_lcl).unwrap();
    root_area.draw(&text_ml).unwrap();
    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

   


}

#[test]
fn control_chart_drawing_svg_work(){
     use csv;
    let path = "./testdata/子组_R控制图_凸轮轴长度.csv".to_string();
    let data_in_column =
        read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 5);
    let data_in_rows = data_in_column.one_column_into_rows_vec();
    let mut rcc = chart::ControlChart::r_from_sub_groub_in_rows_vec(&data_in_rows);
    println!("{:?}", &rcc);
    //let path_writing_to = "./output/control_chart.csv".to_string();
    //chart::ControlChart::control_chart_write_to_csv(&rcc,&path_writing_to);
    let chart_title="R Chart Control".to_string();
    control_chart_drawing_svg(&rcc,&chart_title);

}