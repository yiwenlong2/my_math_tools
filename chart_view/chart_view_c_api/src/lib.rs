use std::{io::Write, ops::Deref, rc::Rc};

use chart;
use data_struct::sheetdata::{self, ItemsInOneColumn};
use libc::{self, c_double, c_uint, c_ulong};
use mem_data_trans::CXbarControlChart;

use crate::{chart_type_define::{type_IControlChart, type_MRControlChart, type_RControlChart, type_SControlChart, type_XbarControlChart}, mem_data_trans::from_qt_double_vec_to_rust_sbgb_in_rows};

mod mem_data_trans;
mod chart_type_define;

#[cfg(test)]
mod tests {
    use crate::r_control_chart;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rcc_works() {
        r_control_chart(true);
    }
}

#[no_mangle]
pub extern "C" fn func(buf_ptr_f64: *mut c_double, len: c_uint) {
    println!("From test func");
    let vec_f64_from_ptr = unsafe { std::slice::from_raw_parts_mut(buf_ptr_f64, len as usize) };
    for i in vec_f64_from_ptr {
        println!("{}", &i);
    }
}

#[no_mangle]
pub extern "C" fn control_map_from_sub_groub_in_rows_vec(
    sbgb_in_rows_ptr_from_c: *mut f64,
    len: u32,
    split_n: u32,
) -> CXbarControlChart {
    let sbgb_in_rows =
        from_qt_double_vec_to_rust_sbgb_in_rows(sbgb_in_rows_ptr_from_c, len, split_n);
    let r_control = chart::RControlChart::from_sub_groub_in_rows_vec(&sbgb_in_rows);
    let x_bar_control = Box::new(chart::XbarControlChart::from_sub_groub_in_rows_vec(
        &sbgb_in_rows,
    ));

    let c_xcc = mem_data_trans::struct_trans_from_xbarcontrolchart_to_c(x_bar_control);

    return c_xcc;
}

#[no_mangle]
pub extern "C" fn get_vec_from_rust(mut len: *mut i32) -> *mut i32 {
    println!("From func:get_vec_from_rust");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    //print!("{:?}",&v);
    let _ptr = v.as_mut_ptr();
    let _len = Box::new(v.len() as i32);
    unsafe {
        //len=Box::into_raw(_len);
        *len = *_len;
    }
    //std::mem::forget(_len);
    //std::mem::forget(v);
    _ptr
}


#[no_mangle]
pub extern  "C" fn get_vec_from_rust_v2(array:*mut libc::c_double, mut len: *mut i32){
    unsafe{
        for i in 0..*len {
            
            array.add(i as usize).write(i as f64);
        }
    }
}

#[no_mangle]
pub extern "C" fn r_control_chart(inrows: bool) {
    use csv;
    let path = "./testdata/子组_R控制图_凸轮轴长度.csv".to_string();
    let data_in_column =
        read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 5);
    let data_in_rows = data_in_column.one_column_into_rows_vec();
    let mut rcc = chart::ControlChart::r_from_sub_groub_in_rows_vec(&data_in_rows);
    //println!("{:?}", &rcc);
    //println!("{:?}", &rcc.data.len());
    let path_writing_to = "./output/control_chart.csv".to_string();
    chart::ControlChart::control_chart_write_to_csv(&rcc,&path_writing_to);

}

#[no_mangle]
pub extern  "C" fn xbar_control_chart(inrows:bool){
    use csv;
    let path = "./testdata/子组_R控制图_凸轮轴长度.csv".to_string();
    let data_in_column =
        read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 5);
    let data_in_rows = data_in_column.one_column_into_rows_vec();
    let mut rcc = chart::ControlChart::xbar_from_sub_groub_in_rows_vec(&data_in_rows);
    //println!("{:?}", &rcc);
    let path_writing_to = "./output/control_chart.csv".to_string();
    chart::ControlChart::control_chart_write_to_csv(&rcc,&path_writing_to);

}

#[no_mangle]
pub extern  "C" fn calculate_control_chart_subgroup(inrows:bool,chart_type:u32,data_in_column_number:u32,split_n: u32){
    use csv;
    let path = "./database/tableInput.csv".to_string();
    let data_in_rows;
    if(inrows){
        let data_in_column =
            read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, data_in_column_number as usize, split_n as usize);
        data_in_rows = data_in_column.one_column_into_rows_vec();
    }
    else {
        panic!("inrows=false!");
    }
    let mut cc=chart::ControlChart{
        ucl:0.0,
        lcl:0.0,
        ml:0.0,
        data:Vec::new(),
    };

    if(chart_type==type_RControlChart){
        cc = chart::ControlChart::r_from_sub_groub_in_rows_vec(&data_in_rows);
    }
    else if(chart_type==type_XbarControlChart) {
        cc=chart::ControlChart::xbar_from_sub_groub_in_rows_vec(&data_in_rows);
    }
    else if chart_type==type_SControlChart {
        cc=chart::ControlChart::s_from_sub_groub_in_rows_vec(&data_in_rows);
    }
    else {
        panic!("Error chart_type!");
    }

    let path_writing_to = "./output/control_chart.csv".to_string();
    chart::ControlChart::control_chart_write_to_csv(&cc,&path_writing_to);
}


#[no_mangle]
pub extern  "C" fn calculate_control_chart_individual(inrows:bool,chart_type:u32,data_in_column_number:u32,split_n: u32,mr_width:u32,mr_step:u32){
    use csv;
    let path = "./database/tableInput.csv".to_string();
    let mut data_in_column=ItemsInOneColumn{
        data:Vec::new(),
    };
    if(inrows){
        data_in_column.data=
            read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, data_in_column_number as usize, split_n as usize).data;
       
    }
    else {
        panic!("inrows=false!");
    }
    let mut cc=chart::ControlChart{
        ucl:0.0,
        lcl:0.0,
        ml:0.0,
        data:Vec::new(),
    };

    if chart_type==type_MRControlChart {
        cc = chart::ControlChart::mr_from_items_in_one_column(&data_in_column,mr_width as usize,mr_step as usize);
    }
    if chart_type==type_IControlChart {
        cc=chart::ControlChart::i_from_items_in_one_column(&data_in_column);
    }



}

