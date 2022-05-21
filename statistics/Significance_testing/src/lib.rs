use std::io::Write;

use array_tools;
use combination::combine;
use data_struct::*;
mod my_array_tools;
mod my_math_tools;

pub fn mean(array: &Vec<f64>) -> f64 {
    let mut sum = 0f64;
    let n = array.len();
    for i in array.iter() {
        sum = i + sum;
    }
    sum / (n as f64)
}

//显著性验证, 输入A数据,和同时间内做了某项改善后的B数据,以及过往处于的A状态的历史数据;
pub fn significance_with_history(
    array_a: &Vec<f64>,
    array_b: &Vec<f64>,
    array_a_history: &Vec<f64>,
) -> f64 {
    if array_a.len() != array_b.len() {
        //A,B两数据长度不一致时报错
        panic!("Array A's length not equal to B's");
    }

    let mut significance: f64 = 0.0;
    let a_mean = mean(&array_a);
    let b_mean = mean(&array_b);
    let a_b_bay_dev = b_mean - a_mean;
    if a_b_bay_dev == 0.0 {
        //A,B两数据均值没有差异时,返回显著值100%,也就是该改善没有任何作用;
        significance = 1.0;
        return significance;
    }

    let n_dev = array_b.len();
    let mut a_history_dev_array: Vec<f64> = Vec::new();
    let mut sum_temp_1 = 0f64;
    let mut sum_temp_2 = 0f64;
    let mut a_history_dev;

    let mut i = 0usize;
    loop {
        if i + 1 + n_dev == array_a_history.len() + 1 {
            break;
        }

        for j in i..(i + n_dev) {
            sum_temp_1 = array_a_history[j] + sum_temp_1;
        }
        for j in (i + 1)..(i + 1 + n_dev) {
            sum_temp_2 = array_a_history[j] + sum_temp_2;
        }

        a_history_dev = (sum_temp_2 / (n_dev as f64)) - (sum_temp_1 / (n_dev as f64));
        a_history_dev_array.push(a_history_dev);
        i = i + 1;
    }
    a_history_dev_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", a_history_dev_array);

    let las_one_of_index = a_history_dev_array.len() - 1;
    let index;

    if a_b_bay_dev < a_history_dev_array[0] || a_b_bay_dev > a_history_dev_array[las_one_of_index] {
        significance = 0.0;
    } else if a_b_bay_dev > 0.0 {
        for i in 0..a_history_dev_array.len() {
            if a_b_bay_dev > a_history_dev_array[i] {
            } else if a_b_bay_dev == a_history_dev_array[i] {
                index = i;
                significance = ((a_history_dev_array.len() as f64) - ((index + 1) as f64))
                    / (a_history_dev_array.len() as f64);
                break;
            } else {
                index = i - 1;
                significance = ((a_history_dev_array.len() as f64) - ((index + 1) as f64))
                    / (a_history_dev_array.len() as f64);
                break;
            }
        }
    } else if a_b_bay_dev < 0.0 {
        for i in 0..a_history_dev_array.len() {
            if a_b_bay_dev > a_history_dev_array[i] {
            } else if a_b_bay_dev == a_history_dev_array[i] {
                index = i;
                significance = ((index + 1) as f64) / (a_history_dev_array.len() as f64);
                break;
            } else {
                index = i - 1;
                significance = ((index + 1) as f64) / (a_history_dev_array.len() as f64);
                break;
            }
        }
    }

    return significance;
}

//在没有历史样本的情况下, 对有限的样本随机分为A,B两组, 对B实行改善;
//将A,B试验的结果再进行混合,组合出与A,B同样数量的组合集合,求均差,
//假设B改善没有效果, 所有可能组合的均差正态分布; 如果原A,B的均差越包含在内,则说明B改善越没有效果
pub fn significance_with_sample(a_sample: &Vec<f64>, b_sample: &Vec<f64>) -> f64 {
    let all_sample_count = a_sample.len() + b_sample.len();
    let _a_b_mean_dev = mean(b_sample) - mean(a_sample);
    let mut all_sample = a_sample.clone();
    let mut _b_sample = b_sample.clone();
    let significance;
    let all_sample_combination_count =
        my_math_tools::Probability::combination(all_sample_count, a_sample.len());

    all_sample.append(&mut _b_sample);

    let combination_index = combine::index(all_sample_count, a_sample.len());
    let mut combination_index_others: Vec<Vec<usize>> = Vec::new();

    for combination_index_item in combination_index.iter() {
        let mut temp: Vec<usize> = Vec::new();
        for i in 0..all_sample_count {
            if !combination_index_item.contains(&i) {
                temp.push(i);
            }
        }
        combination_index_others.push(temp);
    }

    let mut sample_combination: Vec<Vec<f64>> = Vec::new();
    for i in combination_index.iter() {
        sample_combination.push(combination::select_index(&all_sample, &i));
    }

    let mut sample_combination_others: Vec<Vec<f64>> = Vec::new();
    for i in combination_index_others.iter() {
        sample_combination_others.push(combination::select_index(&all_sample, &i));
    }

    let mut _dev_mean_temp: f64;
    let mut _dev_mean_array: Vec<f64> = Vec::new();
    for i in 0..sample_combination.len() {
        _dev_mean_temp = mean(&sample_combination[i]) - mean(&sample_combination_others[i]);
        _dev_mean_array.push(_dev_mean_temp);
    }
    _dev_mean_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", _a_b_mean_dev);
    println!("{:?}", _dev_mean_array);

    let compare_index_result =
        my_array_tools::find_index_biger_less_tan_value(&_dev_mean_array, _a_b_mean_dev);
    match compare_index_result {
        my_array_tools::CompareResultIndex::BiggerThanMax => {
            significance = 0.0;
            return significance;
        }
        my_array_tools::CompareResultIndex::LessThanMin => {
            significance = 0.0;
            return significance;
        }
        my_array_tools::CompareResultIndex::Index(_index) => {
            println!("{}", _index);
            significance = ((_dev_mean_array.len() - _index - 1) as f64)
                / (all_sample_combination_count as f64);
            return significance;
        }
    }
}

//虽然做试验时,A,B两措施应该在同一个个体随机试验,但是在实验结果统计时:输入的数据b_sample为B措施改善组, b-a的差异值平均值为目标值;
//在计算假设后随机试验中比B试验相对A试验差异的个数时, 小数位数会影响统计个数, 所以要指定小数位数; 
//等于目标差异的个数,只取一半;比如5个数等于目标值,3个数大于目标值, 统计计数为5/2 +3 =5.5; 
pub fn significance_with_twoactions_on_eatch<
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::convert::Into<f64>,
>(
    a_sample: &Vec<T>,
    b_sample: &Vec<T>,behind_point_in_process:u8
) -> f64 {
    let behind_point:u8=behind_point_in_process;
    if a_sample.len() != b_sample.len() {
        panic!("Two arrays' length is not equality!");
    }
    let mut array1: Vec<f64> = Vec::new();
    for i in 0..a_sample.len() {
        let temp: f64;
        temp = b_sample[i].into() - a_sample[i].into();
        array1.push(temp);
    }
    let mut array2: Vec<f64> = Vec::new();
    for i in array1.iter() {
        let temp: f64;
        temp = -i;
        array2.push(temp);
    }

    //println!("{:?}",array1);
    //println!("{:?}",array2);

    let head_value: f64 = 0.0;
    let binary_tree_head = n2_permulation(&array1, &array2, head_value);
    let mut sum: f64 = 0.0;
    let mut sum_vec: Vec<f64> = Vec::new();
    traverse_branch_sum(binary_tree_head, sum, &mut sum_vec);
    let mut sum_mean_vec: Vec<f64> = Vec::new();
    for i in sum_vec.iter() {
        let temp_orgin = i / (a_sample.len() as f64);
        let temp=math_array_tools::Basic::float_output_format(temp_orgin, behind_point);
        sum_mean_vec.push(temp);
    }

    sum_mean_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    //println!("{:?}",sum_mean_vec.len());
    //println!("{:?}",sum_mean_vec);
    let target_value_orign = math_array_tools::Basic::mean(&array1);

    let target_value=math_array_tools::Basic::float_output_format(target_value_orign, behind_point);

    println!("{}",target_value);
    let compare_rst = array_tools::find_index_in_ranked_vec(&sum_mean_vec, target_value);
    let mut significiance_value = 0.0f64;
    match compare_rst {
        array_tools::CompareResultIndex::BiggerThanMax => significiance_value = 0.0,
        array_tools::CompareResultIndex::EqualityIndex(x) => {
            //println!("{}",x);
            let mut equality_count:usize=0;
            for i in x..sum_mean_vec.len(){
                if target_value==sum_mean_vec[i] {
                    equality_count +=1;
                }
            }
            //println!("{}",equality_count);
            let half_equality_count=equality_count/2;
            significiance_value = ((sum_mean_vec.len() - x-half_equality_count) as f64) / (sum_mean_vec.len() as f64)
        }
        array_tools::CompareResultIndex::MiddleLeftIndex(x) => {
           // println!("{}",x);
            significiance_value =
                ((sum_mean_vec.len() - x + 1) as f64) / (sum_mean_vec.len() as f64)
        }
        array_tools::CompareResultIndex::NearLeftIndex(x) => {
            //println!("{}",x);
            significiance_value =
                ((sum_mean_vec.len() - x + 1) as f64) / (sum_mean_vec.len() as f64)
        }
        array_tools::CompareResultIndex::NearRightIndex(x) => {
            //println!("{}",x);
            significiance_value = ((sum_mean_vec.len() - x) as f64) / (sum_mean_vec.len() as f64)
        }
        array_tools::CompareResultIndex::LessThanMin => significiance_value = 1.0,
    }
    let mut f=std::fs::File::create("data_significance_with_twoactions_on_eatch.CSV").unwrap();
    for i in sum_mean_vec {
       
        f.write(i.to_string().as_bytes());
        f.write("\n".to_string().as_bytes());
    }

    return significiance_value;
}
