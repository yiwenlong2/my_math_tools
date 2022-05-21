mod constantbase;
mod evaluate_sigma;
mod tests;

use data_struct::sheetdata::{ItemsInOneColumn, SubGroubInRows};
use integral_square_root::SquareRoot64;
use math_array_tools;

use crate::constantbase::{c4, c5, d2, d3};

pub struct SubGroubControlChart {
    pub xbarcontrolchart: XbarControlChart,
    pub rcontrolchart: RControlChart,
}

#[derive(Debug, Clone)]
pub struct XbarControlChart {
    pub ucl: f64,
    pub lcl: f64,
    pub Xbar_all: f64,
    pub Xbar_data: Vec<f64>,
}

impl XbarControlChart {
    //要求同一个子组在同一行，一行作为一个Vec<T>矩阵；
    pub fn from_sub_groub_in_rows_vec(sbgb_in_rows: &SubGroubInRows<f64>) -> XbarControlChart {
        let mut xbar_vec: Vec<f64> = Vec::new();
        let n_sqrt = sbgb_in_rows.data[0].len().sqrt();

        //println!("n's sqrt:{}",n_sqrt);

        //println!("n_sqrt: {}",n_sqrt);
        for i in sbgb_in_rows.data.iter() {
            let xbar_sub_groub = math_array_tools::Basic::mean(i);
            xbar_vec.push(xbar_sub_groub);
        }
        let all_in_one_column = sbgb_in_rows.rows_into_one_column();
        let xbar_all = math_array_tools::Basic::mean(&all_in_one_column.data);

        let mut R_data: Vec<f64> = Vec::new();
        for row in sbgb_in_rows.data.iter() {
            let r = math_array_tools::Basic::max(row) - math_array_tools::Basic::min(row);
            R_data.push(r);
        }
        //println!("{:?}",R_data);
        let r_bar = math_array_tools::Basic::mean(&R_data);
        //println!("r_bar: {:?}",r_bar);
        let d2 = d2(sbgb_in_rows.data[0].len());
        let sigma = r_bar / d2;
        //println!("d2: {:?}",d2);
        //println!("sigma: {:?}",sigma);

        let ucl = xbar_all + 3.0 * sigma / n_sqrt;
        let lcl = xbar_all - 3.0 * sigma / n_sqrt;

        let xbar_control_chart = XbarControlChart {
            ucl: ucl,
            lcl: lcl,
            Xbar_all: xbar_all,
            Xbar_data: xbar_vec,
        };
        return xbar_control_chart;
    }
}

#[derive(Debug, Clone)]
pub struct RControlChart {
    pub ucl: f64,
    pub lcl: f64,
    pub r_bar: f64,
    pub R_data: Vec<f64>,
}

impl RControlChart {
    pub fn from_sub_groub_in_rows_vec(sbgb_in_rows: &SubGroubInRows<f64>) -> RControlChart {
        let mut R_data: Vec<f64> = Vec::new();
        for row in sbgb_in_rows.data.iter() {
            let r = math_array_tools::Basic::max(row) - math_array_tools::Basic::min(row);
            R_data.push(r);
        }
        //println!("{:?}",R_data);
        let r_bar = math_array_tools::Basic::mean(&R_data);
        //println!("r_bar: {:?}",r_bar);
        let d2 = d2(sbgb_in_rows.data[0].len());
        //let sigma=r_bar/d2;
        //println!("d2: {:?}",d2);
        let d3 = d3(sbgb_in_rows.data[0].len());
        //println!("d3: {:?}",d3);
        //println!("sigma: {:?}",sigma);
        let ucl = r_bar + 3.0 * d3 * (r_bar / d2);
        //println!("ucl: {:?}",ucl);
        let r_control = RControlChart {
            ucl: ucl,
            lcl: 0.0,
            r_bar: r_bar,
            R_data: R_data,
        };
        return r_control;
    }
}

#[derive(Debug, Clone)]
pub struct SControlChart {
    pub ucl: f64,
    pub lcl: f64,
    pub s_bar: f64,
    pub S_data: Vec<f64>,
}

impl SControlChart {
    pub fn from_sub_groub_in_rows_vec(sbgb_in_rows: &SubGroubInRows<f64>) -> SControlChart {
        let mut S_data: Vec<f64> = Vec::new();
        for row in sbgb_in_rows.data.iter() {
            let s_i = math_array_tools::Statistic::standard_variance_sample(row);
            //println!("{}",s_i);
            S_data.push(s_i);
        }
        let s_bar = math_array_tools::Basic::mean(&S_data);
        //println!("The mean of Si: {}",&s_bar);

        let n = sbgb_in_rows.data[0].len();
        let c4 = c4(n);
        let sigma = s_bar / c4;
        //println!("{}",sigma);
        let c5 = c5(n);
        let ucl = c4 * sigma + 3.0 * sigma * c5;
        let lcl = c4 * sigma - 3.0 * sigma * c5;

        let s_control_chart = SControlChart {
            ucl: ucl,
            lcl: lcl,
            s_bar: s_bar,
            S_data: S_data,
        };
        return s_control_chart;
    }
}

#[derive(Debug, Clone)]
pub struct IControlChart {
    pub ucl: f64,
    pub lcl: f64,
    pub I_bar: f64,
    pub I_data: Vec<f64>,
}

impl IControlChart {
    pub fn from_items_in_one_column(items_in_one_column: &ItemsInOneColumn<f64>) -> IControlChart {
        let I_bar = math_array_tools::Basic::mean(&items_in_one_column.data);
        let sigma = evaluate_sigma::sigma_from_moving_range_average(&items_in_one_column.data, 2);
        let ucl = I_bar + 3.0 * sigma;
        let lcl = I_bar - 3.0 * sigma;
        let i_control_chart = IControlChart {
            ucl,
            lcl,
            I_bar,
            I_data: items_in_one_column.data.clone(),
        };
        return i_control_chart;
    }
}

#[derive(Debug, Clone)]
pub struct MRControlChart {
    pub ucl: f64,
    pub lcl: f64,
    pub r_bar: f64,
    pub R_data: Vec<f64>,
}

impl MRControlChart {
    pub fn from_items_in_one_column(items_in_one_column: &ItemsInOneColumn<f64>,mr_width:usize,mr_step:usize) -> MRControlChart {
        
        let mr = math_array_tools::Basic::moveing_range(&items_in_one_column.data, mr_step, mr_width);
        //println!("{:?}",&mr);
        let mr_bar = math_array_tools::Basic::sum(&mr)
            / ((items_in_one_column.data.len() - mr_width + 1) as f64);
        let sigma = mr_bar / d2(mr_width);
        let ucl = mr_bar + 3.0 * d3(mr_width) * sigma;
        //print!("ucl: {}",ucl);
        let lcl = mr_bar - 3.0 * d3(mr_width) * sigma;
        let mr_control_chart = MRControlChart {
            ucl,
            lcl: 0.0,
            r_bar: mr_bar,
            R_data: mr,
        };
        return mr_control_chart;
    }
}

#[derive(Debug, Clone)]
pub struct ControlChart {
    pub ucl: f64,
    pub lcl: f64,
    pub ml: f64,
    pub data: Vec<f64>,
}

use std::io::Write;
impl ControlChart {
    pub fn r_from_sub_groub_in_rows_vec(sbgb_in_rows: &SubGroubInRows<f64>) -> ControlChart {
        let mut R_data: Vec<f64> = Vec::new();
        for row in sbgb_in_rows.data.iter() {
            let r = math_array_tools::Basic::max(row) - math_array_tools::Basic::min(row);
            R_data.push(r);
        }
        //println!("{:?}",R_data);
        let r_bar = math_array_tools::Basic::mean(&R_data);
        //println!("r_bar: {:?}",r_bar);
        let d2 = d2(sbgb_in_rows.data[0].len());
        //let sigma=r_bar/d2;
        //println!("d2: {:?}",d2);
        let d3 = d3(sbgb_in_rows.data[0].len());
        //println!("d3: {:?}",d3);
        //println!("sigma: {:?}",sigma);
        let ucl = r_bar + 3.0 * d3 * (r_bar / d2);
        //println!("ucl: {:?}",ucl);
        let r_control = ControlChart {
            ucl: ucl,
            lcl: 0.0,
            ml: r_bar,
            data: R_data,
        };
        return r_control;
    }

    pub fn xbar_from_sub_groub_in_rows_vec(sbgb_in_rows: &SubGroubInRows<f64>) -> ControlChart {
        let mut xbar_vec: Vec<f64> = Vec::new();
        let n_sqrt = sbgb_in_rows.data[0].len().sqrt();

        //println!("n's sqrt:{}",n_sqrt);

        //println!("n_sqrt: {}",n_sqrt);
        for i in sbgb_in_rows.data.iter() {
            let xbar_sub_groub = math_array_tools::Basic::mean(i);
            xbar_vec.push(xbar_sub_groub);
        }
        let all_in_one_column = sbgb_in_rows.rows_into_one_column();
        let xbar_all = math_array_tools::Basic::mean(&all_in_one_column.data);

        let mut R_data: Vec<f64> = Vec::new();
        for row in sbgb_in_rows.data.iter() {
            let r = math_array_tools::Basic::max(row) - math_array_tools::Basic::min(row);
            R_data.push(r);
        }
        //println!("{:?}",R_data);
        let r_bar = math_array_tools::Basic::mean(&R_data);
        //println!("r_bar: {:?}",r_bar);
        let d2 = d2(sbgb_in_rows.data[0].len());
        let sigma = r_bar / d2;
        //println!("d2: {:?}",d2);
        //println!("sigma: {:?}",sigma);

        let ucl = xbar_all + 3.0 * sigma / n_sqrt;
        let lcl = xbar_all - 3.0 * sigma / n_sqrt;

        let xbar_control_chart = ControlChart {
            ucl: ucl,
            lcl: lcl,
            ml: xbar_all,
            data: xbar_vec,
        };
        return xbar_control_chart;
    }

    pub fn s_from_sub_groub_in_rows_vec(sbgb_in_rows: &SubGroubInRows<f64>) -> ControlChart {
        let mut S_data: Vec<f64> = Vec::new();
        for row in sbgb_in_rows.data.iter() {
            let s_i = math_array_tools::Statistic::standard_variance_sample(row);
            //println!("{}",s_i);
            S_data.push(s_i);
        }
        let s_bar = math_array_tools::Basic::mean(&S_data);
        //println!("The mean of Si: {}",&s_bar);

        let n = sbgb_in_rows.data[0].len();
        let c4 = c4(n);
        let sigma = s_bar / c4;
        //println!("{}",sigma);
        let c5 = c5(n);
        let ucl = c4 * sigma + 3.0 * sigma * c5;
        let lcl = c4 * sigma - 3.0 * sigma * c5;

        let s_control_chart = ControlChart {
            ucl: ucl,
            lcl: lcl,
            ml: s_bar,
            data: S_data,
        };
        return s_control_chart;
    }

    pub fn i_from_items_in_one_column(items_in_one_column: &ItemsInOneColumn<f64>) -> ControlChart {
        let I_bar = math_array_tools::Basic::mean(&items_in_one_column.data);
        let sigma = evaluate_sigma::sigma_from_moving_range_average(&items_in_one_column.data, 2);
        let ucl = I_bar + 3.0 * sigma;
        let lcl = I_bar - 3.0 * sigma;
        let i_control_chart = ControlChart {
            ucl,
            lcl,
            ml: I_bar,
            data: items_in_one_column.data.clone(),
        };
        return i_control_chart;
    }

    pub fn mr_from_items_in_one_column(items_in_one_column: &ItemsInOneColumn<f64>,mr_width:usize,mr_step:usize) -> ControlChart {
        
        let mr = math_array_tools::Basic::moveing_range(&items_in_one_column.data, mr_step, mr_width);
        //println!("{:?}",&mr);
        let mr_bar = math_array_tools::Basic::sum(&mr)
            / ((items_in_one_column.data.len() - mr_width + 1) as f64);
        let sigma = mr_bar / d2(mr_width);
        let ucl = mr_bar + 3.0 * d3(mr_width) * sigma;
        //print!("ucl: {}",ucl);
        //let lcl = mr_bar - 3.0 * d3(mr_width) * sigma;
        let mr_control_chart = ControlChart {
            ucl,
            lcl: 0.0,
            ml: mr_bar,
            data: mr,
        };
        return mr_control_chart;
    }

    pub fn control_chart_write_to_csv(control_chart: &ControlChart,path:&String) {
        
        //let path_writing_to = "./output/control_chart.csv".to_string();
        let mut f = std::fs::File::create(path).unwrap();
        f.write(&control_chart.ucl.to_string().as_bytes()).unwrap();
        f.write(b",").unwrap();
        f.write(&control_chart.lcl.to_string().as_bytes()).unwrap();
        f.write(b",").unwrap();
        f.write(&control_chart.ml.to_string().as_bytes()).unwrap();
        f.write(b"\n").unwrap();
        let mut j=0;
        for i in control_chart.data.iter() {
            j +=1;
            let temp_string = i.to_string();
            let temp_u8 = temp_string.as_bytes();
            f.write(temp_u8).unwrap();
            if j != control_chart.data.len() {
                f.write(b",").unwrap();
            }
            //println!("{}:{}",&j,&i);
        }
        
    }


}
