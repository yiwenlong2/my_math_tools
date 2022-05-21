use data_struct::sheetdata::SubGroubInRows;




pub fn from_qt_double_vec_to_rust_sbgb_in_rows(x_bartcontrol_ptr_from_c:*mut f64,len:u32,split_n: u32)->SubGroubInRows<f64>{
    let mut x_bartcontrol_data:Vec<Vec<f64>>=Vec::new();
    let vec_f64_from_ptr=unsafe {
        std::slice::from_raw_parts_mut(x_bartcontrol_ptr_from_c, len as usize)
    };
    println!("{:?}",vec_f64_from_ptr);
    let mut temp_vec_f64=Vec::new();
    let mut n:u32=1;
    for i in vec_f64_from_ptr.iter() {
        temp_vec_f64.push(*i);
        if n==split_n {
            x_bartcontrol_data.push(temp_vec_f64.clone());
            temp_vec_f64.clear();
            n=1;
        }
        else{
            n=n+1;
        }
    }
    //打印数据是否装入Vec<Vec<T>>；
    //println!("{:?}",x_bartcontrol_data);
    let x_bar_control=data_struct::sheetdata::SubGroubInRows{
        data: x_bartcontrol_data,
    };

    return x_bar_control;
}





#[repr(C)]
#[derive(Debug)]
pub struct CXbarControlChart{
    pub ucl:f64,
    pub lcl:f64,
    pub Xbar_all:f64,
    pub Xbar_data:&'static [f64],
    pub Xbar_data_len: u32,
}

pub fn struct_trans_from_xbarcontrolchart_to_c(x_bar_control:Box<chart::XbarControlChart>)->CXbarControlChart{

    let x_data=unsafe{(*(Box::into_raw(x_bar_control.clone()))).Xbar_data.as_slice()};
    let c_xbcc=CXbarControlChart {
        ucl:x_bar_control.ucl,
        lcl:x_bar_control.lcl,
        Xbar_all:x_bar_control.Xbar_all,
        Xbar_data:x_data,
        Xbar_data_len: x_bar_control.Xbar_data.len() as u32,
    };
  
    return c_xbcc;

}


