#[cfg(test)]
mod tests;

//使用外部crate::combination;
use combination::combine;

pub mod Probability {

    //求n的阶乘;
    pub fn factorial(n: usize) -> usize {
        let mut sum: usize = 1;
        for i in 1..=n {
            sum = i * sum;
        }
        return sum;
    }
    //求在n个中取m个组合数;
    pub fn combination(n: usize, m: usize) -> usize {
        let mut rst: usize = 1;
        if m > n {
            panic!("m is bigger than n");
        } else if m == n {
            return rst;
        } else {
            rst = (factorial(n)) / (factorial(m) * factorial(n - m));
        }
        return rst;
    }
    //求在n个中取m个排列的数;
    pub fn permutation(n: usize, m: usize) -> usize {
        let mut rst: usize = 1;
        if m > n {
            panic!("m is bigger than n");
        } else if m == n {
            return rst;
        } else {
            rst = (factorial(n)) / (factorial(n - m));
        }

        return rst;
    }
    //列出在n个中取m个组合的所有组合的编号和剩下的元素组成的编号;
    //直接列出排列或组合数使用combination::premutate或combine(&Vec<T>)
    pub fn combination_index(n: usize, m: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
        let combination_index = combination::combine::index(n, m);
        let mut combination_index_others: Vec<Vec<usize>> = Vec::new();

        for combination_index_item in combination_index.iter() {
            let mut temp: Vec<usize> = Vec::new();
            for i in 0..n {
                if !combination_index_item.contains(&i) {
                    temp.push(i);
                }
            }
            combination_index_others.push(temp);
        }
        return (combination_index, combination_index_others);
    }
}

pub mod Basic {
    use std::process::Output;


    pub fn sum<T: Copy + std::ops::Add<Output = T>>(array: &Vec<T>) -> T {
        let mut sum: T = array[0];
        let n = array.len();
        for i in 1..array.len() {
            sum = array[i] + sum;
        }
        return sum;
    }

    pub fn mean<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>>(
        array: &Vec<T>,
    ) -> f64 {
        let mut sum: T = array[0];
        let n = array.len();
        for i in 1..array.len() {
            sum = array[i] + sum;
        }

        let sumf64: f64 = sum.into();
        let rst = sumf64 / (n as f64);
        return rst;
    }

    pub  fn max<T:Copy+std::cmp::PartialOrd>(array: &Vec<T>) ->T {
        let mut max=array[0];
        for i in array.iter(){
            if *i > max {
                max=*i;
            }
        }

        return max;
    }

    pub fn min<T:Copy+std::cmp::PartialOrd>(array: &Vec<T>) ->T {
        let mut min=array[0];
        for i in array.iter() {
            if *i<min {
                min=*i;
            }
        }
        return min;
    }

    //求数组的极差
    pub fn range<T:Copy+std::cmp::PartialOrd+std::cmp::PartialEq+std::ops::Sub<Output=T>>(array: &Vec<T>)->T{
        let range=max(array)-min(array);
        return range;
    }

    //获取移动极差, 返回的是一个极差集合
    pub fn moveing_range<T:Copy+std::cmp::PartialOrd+std::cmp::PartialEq+std::ops::Sub<Output=T>>(array: &Vec<T>,step:usize,width:usize)->Vec<T>{
        let mut mv_rg:Vec<T>=Vec::new();
        let vec=array_tools::create_vec_move_step_with_width(array, step, width);
        for i in vec.iter(){
            let temp=range(i);
            mv_rg.push(temp);
        }
        return mv_rg;

    }

    //获取指定小数的f64 输出, 末尾为四舍五入;
    pub fn float_output_format(float:f64,behind_point:u8) ->f64{
        let float_int_part=(float as i32) as f64;
        //println!("float_int_part={}",float_int_part);
        let float_float_part=float-float_int_part;
        //println!("float_float_part={}",float_float_part);
        let mut exp=10.0;
        for i in 1..behind_point {
            exp=exp*10.0;
        }
        //println!("exp={}",exp);
        let mut float_exp=float_float_part* exp;
        if (float_exp-((float_exp as i32) as f64))>=0.5 {
            float_exp=float_exp+1.0;
        }
        //println!("float_exp={}",float_exp);
        let float_refloat=((float_exp as i32) as f64)/exp;
        //println!("float_refloat{}",float_refloat);
        let final_float=float_int_part+float_refloat;
        //println!("final_float={}",final_float);
        return final_float;
    }
}
mod Distribution;
pub mod Statistic {
    use std::ptr::read_volatile;

    use crate::Distribution::*;

    
    //均值, 沿用basic 模块mean
    pub fn mean<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>>(array: &Vec<T>) -> f64 {
        use crate::Basic::mean;
        let mean=mean(&array);
        return mean;
    }
    //总体的方差
    pub fn variance_totality<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>>(array:&Vec<T>)->f64{
        let mean=mean(&array);
        let mut _sum:f64=0.0;
        for i in 0..array.len() {
            _sum=(array[i].into()-mean)*(array[i].into()-mean)+_sum;
        }
        let variance=_sum/(array.len() as f64);

        return variance;
    }
    //总体的标准差
    pub fn standard_variance_totality<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>>(array:&Vec<T>)->f64{
        extern crate integral_square_root;
        let variance=variance_totality(&array);  
        let standard_variance=variance.sqrt();
        return standard_variance;
    }
    //样本方差
    pub fn variance_sample<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>>(array:&Vec<T>)->f64{
        let mean=mean(&array);
        let mut _sum:f64=0.0;
        for i in 0..array.len() {
            _sum=(array[i].into()-mean)*(array[i].into()-mean)+_sum;
        }
        let variance=_sum/((array.len()-1 )as f64);

        return variance;
    }
    //样本标准差
    pub fn standard_variance_sample<T: Copy + std::ops::Add<Output = T>+ std::convert::Into<f64>>(array:&Vec<T>)->f64{
        extern crate integral_square_root;
        let variance=variance_sample(&array);  
        let standard_variance=variance.sqrt();
        return standard_variance;
    }
    //中位数
    pub fn median<T:Copy+std::cmp::PartialOrd+std::ops::Add<Output = T>+std::convert::Into<f64>> (array:&Vec<T>) ->f64{
        let mut array_new=array.clone();
        array_new.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median_f64:f64;
        let x=array_new.len()%2;
        match x {
            0usize=>{
                median_f64=(array_new[array_new.len()/2].into() + array_new[array_new.len()/2-1].into())/2.0;
                
                return median_f64;},
            _=>{median_f64=(array_new[array_new.len()/2]).into();
            return median_f64;} 
        }
    }
    //标准正太分布下,小于value的概率;
    pub fn get_less_than_value_standard_normal_probability(less_than_target:f64)->f64 {
        extern crate array_tools;
        use array_tools::CompareResultIndex;
        use std::fs;
        let f=fs::read("./data.txt").unwrap();
        let s=String::from_utf8(f).unwrap();
        let sv:Vec<&str>=s.split('\n').collect();
        let mut find_vec:Vec<f64>=Vec::new();
        let mut value_vec:Vec<f64>=Vec::new();
        for i in sv {
            let line:Vec<&str>=i.split('\t').collect();
            let find:f64=line[1].trim().parse().unwrap();
            let value:f64=line[2].trim().parse().unwrap();
            find_vec.push(find);
            value_vec.push(value);
        }
    
        let positive_target:f64;
        let index_rst:CompareResultIndex;
        let index_find:usize;
        let mut probability:f64;
        if less_than_target<0.0 {
            positive_target=-less_than_target;
        }
        else {
            positive_target=less_than_target;
        }
        index_rst=array_tools::find_index_in_ranked_vec(&find_vec,positive_target);
        match index_rst {
            CompareResultIndex::BiggerThanMax=>probability=1.0,
                
            CompareResultIndex::LessThanMin=>probability=0.0,
                
            CompareResultIndex::EqualityIndex(f)=>{index_find=f;
                probability=value_vec[index_find];
                println!("target:{},index in finde_vec[{}]={}",less_than_target,index_find,find_vec[index_find])},
        
            CompareResultIndex::NearLeftIndex(f)=>{index_find=f;
                probability=value_vec[index_find];
                println!("target:{},index in finde_vec[{}]={}",less_than_target,index_find,find_vec[index_find])
            },
            CompareResultIndex::NearRightIndex(f)=>{index_find=f;
                probability=value_vec[index_find];
                println!("target:{},index in finde_vec[{}]={}",less_than_target,index_find,find_vec[index_find])
            },
            CompareResultIndex::MiddleLeftIndex(f)=>{index_find=f;
                probability=(value_vec[index_find]+value_vec[index_find+1])/2.0;
                println!("target:{},index in finde_vec[{}]={}",less_than_target,index_find,find_vec[index_find])
            },
        }
        if less_than_target<0.0 {
            probability=1.0-probability;
        }
        else {
        }
        return probability;
    }
    //给定正太分布N(mean,Sigma),小于value的概率;
    pub fn get_less_than_value_normal_probability(normal:Normal,less_than_target:f64) ->f64{
        let y=(less_than_target-normal.mean)/normal.standard_variance;
        let probability=get_less_than_value_standard_normal_probability(y);
        return probability;
    }
    //总体的协方差;输入的两个矩阵,同一个index 代表同一个对象的两个特性;
    pub fn cov_variance_totality<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>> (array1: &Vec<T>,array2: &Vec<T>) ->f64{
        if array1.len() != array2.len() {
            panic!("Tow vectors' len() is not equality!")
        }
        let mean_1=mean(&array1);
        let mean_2=mean(&array2);
        let mut sum:f64=0.0;
        let cov_variance_value:f64;
        for i in 0..array1.len() {
            sum= (array1[i].into()-mean_1)*(array2[i].into()-mean_2)+sum;
        }
        cov_variance_value=sum/(array1.len() as f64);
        return cov_variance_value;
    }
    //与量纲无关的相关系数;
    pub fn correlation_coefficient_totality<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>> (array1: &Vec<T>,array2: &Vec<T>) ->f64 {
        let cov_variance=cov_variance_totality(&array1, &array2);
        let standard_variance_1=standard_variance_totality(&array1);
        let standard_variance_2=standard_variance_totality(&array2);
        let correlation_cofficient_value=cov_variance/(standard_variance_1*standard_variance_2);
        return correlation_cofficient_value;
    }
    //k=1表示相邻,k=2表示相隔1个的样本之间的相关系数叫做自相关系数,自相关系数的数量N被约分掉了,所有跟是不是整体/样本无关;
    //公式中i从0..array.len(),在i-k越界时,(y_i-y_mean)*(y_i_k-y_mean) 只计算(y_i-y_mean);
    pub fn autocorrelation_cofficient<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>> (array: &Vec<T>,k:usize) ->f64 {
        let mut sum_2=0f64;
        let mean=mean(array);
        for i in 0..array.len() {
            sum_2=(array[i].into()-mean)*(array[i].into()-mean)+sum_2;
        }
        let mut sum_1=0f64;
        for i in 0..array.len() {
            if (array.len() as i32)-1-(i as i32)-(k as i32)<0i32 {
                sum_1=(array[array.len()-1-i].into()-mean)+sum_1;
            }
            else {
                sum_1=(array[array.len()-1-i].into()-mean)*(array[array.len()-1-i-k].into()-mean)+sum_1;
            }
        }
        let rst=sum_1/sum_2;
        println!("{}\n{}",sum_1,sum_2);
        return rst;
    }
}
