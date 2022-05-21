use crate::constantbase::d2;

//uc: Unbiased constants 无偏常量

//移动极差平均值法
//mr_width 为移动产生的子组的宽度, 默认为2;
pub fn sigma_from_moving_range_average(array:&Vec<f64>,mr_width:usize) ->f64{

    let mr=math_array_tools::Basic::moveing_range(array, 1, mr_width);
    //println!("{:?}",&mr);
    let mr_bar=math_array_tools::Basic::sum(&mr)/((array.len()-mr_width+1) as f64);
    println!("MR_bar: {}",mr_bar);
    let s_mr=mr_bar/d2(mr_width);
    //println!("{:?}",s_mr);
    return s_mr;
}

