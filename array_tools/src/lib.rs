use std::usize;
use rand::{Rng, distributions::uniform:: SampleUniform};

mod tests;

//创造一个没有重复数据的数组;
pub fn create_vec_with_no_repeat<T: Clone+std::cmp::PartialOrd + std::cmp::PartialEq +SampleUniform> (amount:usize, range_low:T,range_high:T)->Vec<T> {
    let mut rng=rand::thread_rng();
    let mut array:Vec<T>=Vec::new();
    let mut value:T;
 
    
    loop {
        value=rng.gen_range(range_low.clone()..range_high.clone());
        if !array.contains(&value) {
            array.push(value);
        }
        if array.len() ==amount {
            break;
        }
    }
    return array;
}
//从一个只有一列数据的csv文件中创建vec;
pub fn create_vec_from_CSV_onerow(path:String)->Vec<f64>{
    use std::fs;
    let f=fs::read_to_string(path).unwrap();
    let normal_test_vec_string:Vec<&str>=f.split('\n').collect();
    let mut normal_test_vec:Vec<f64>=Vec::new();
    for i in normal_test_vec_string {

        if i !="" {
        let temp:f64=i.to_string().trim().parse().unwrap();
        //println!("{}",temp);
        normal_test_vec.push(temp)
        }
    }
    return normal_test_vec;

}

//在没有重复的数据的数组中找到目标,并返回其索引和布尔值来标记是否找到数据;
pub fn find_value_norepeat_out_index<T: std::cmp::PartialEq>(array:& Vec<T>, target:T) ->(usize,bool) {
    let mut index=0usize;
    if !array.contains(&target) {
        return (0,false);
    }
    for i in array.iter() {
        if *i==target {
            break;
        }
        else {
            index +=1;
        }
    }
    return (index,true);
}

//创造一个可能有重复数据的数组;
pub fn create_vec_with_repeat<T: Clone+std::cmp::PartialOrd + std::cmp::PartialEq +SampleUniform> (amount:usize, range_low:T,range_high:T)->Vec<T> {
    let mut rng=rand::thread_rng();
    let mut array:Vec<T>=Vec::new();
    let mut value:T;
 
   
    loop {
        value=rng.gen_range(range_low.clone()..range_high.clone());
        
        array.push(value);
        
        if array.len() ==amount {
            break;
        }
    }
    return array;
}

//在可能有重复的数组中, 找到目标值所在的索引和一个布尔值用来标记是否找到该数据;
pub fn find_value_repeat_out_indexs<T: std::cmp::PartialEq>(array:& Vec<T>, target:T) ->(Vec<usize>,bool) {

  let mut indexs:Vec<usize>=Vec::new();
  let mut index=0usize;
    if !array.contains(&target) {
        return (indexs,false);
    }
    for i in array.iter() {
        if *i==target {
            indexs.push(index);
        }
        
            index +=1;
        
    }
    return (indexs,true);
}

//删除数组中重复的数据,并返回一个排序后的数组;
pub fn remove_repeat_value_out_ranked <T: Ord+std::cmp::PartialEq>(array:&mut Vec<T>) {
    array.sort();
    
    let mut now_index=0usize;
    loop {
        let now_len=array.len();
        if now_index==(now_len-1) {
            break;
        }
        else {
            if array[now_index]==array[now_index+1] {
                array.remove(now_index+1);
            }
            else {
                now_index +=1;
            }
        }
        
    }
}

//删除数组中的重复数据;
pub fn remove_repeat_value<T: Copy+Ord+std::cmp::PartialEq>(array:&mut Vec<T>) {

    let mut temp;
    let mut temp_index=0usize;
    
    'a:loop {
        let now_len=array.len();
        if temp_index>=(now_len-1) {
            break 'a;
        }
        else {
            temp=array[temp_index];
            let mut i=temp_index+1;
        'b: loop {
                if i>(array.len()-1){
                    break 'b;
                }
                if temp==array[i] {
                    array.remove(i);
                }
                else {
                    i +=1;
                }
            }
            temp_index +=1;
        }
    }
}

//目标值在数列中比较后的结果
pub enum CompareResultIndex {
    BiggerThanMax,
    LessThanMin,
    EqualityIndex(usize),
    NearLeftIndex(usize),
    NearRightIndex(usize),
    MiddleLeftIndex(usize),
}

//在数组array:&Vec<f64> 中 找到<value或>value 的index
pub fn find_index_in_ranked_vec<T: Copy+std::cmp::PartialEq+std::cmp::PartialOrd+std::ops::Sub<Output = T>>(array:&Vec<T>,value:T)->CompareResultIndex {

    let las_one_of_index=array.len()-1;
    let index;

    if value<array[0]{
        return CompareResultIndex::LessThanMin;
    }
    else if value>array[las_one_of_index] {
        return CompareResultIndex::BiggerThanMax;
    } 
    else {
        for i in 0..array.len() {
            if value > array[i] { 
            }
            else if value==array[i] {
                index=i;
                return CompareResultIndex::EqualityIndex(index);              
            }
            else {
                if value-array[i-1] < array[i]-value{
                    index=i-1;
                    return  CompareResultIndex::NearLeftIndex(index);
                }
                else if value-array[i-1] == array[i]-value{
                    index=i-1;
                    return CompareResultIndex::MiddleLeftIndex(index);
                }
                else {
                    index=i;
                    return CompareResultIndex::NearRightIndex(index);
                }
            }
        }
    }
    return CompareResultIndex::BiggerThanMax;
}

pub fn create_vec_move_step_with_width<T:Clone>(array:& Vec<T>,step:usize,width:usize)->Vec<Vec<T>>{
    let mut range_vec:Vec<Vec<T>>=Vec::new();
    
    if width>array.len() {
        panic!("width is biger than array's length");
    }
    else {
        let mut temp:Vec<T>=Vec::new();
        let turn =array.len()/step;
        for i in 0..turn {
            temp.clear();
            if(i*step+width)<=array.len(){
                for j in i*step..(i*step+width) {
                    temp.push(array.get(j).unwrap().clone());
                }
                range_vec.push(temp.clone());
                   
            } else 
            {
                return range_vec;
            } 
        }
        return range_vec;
    }
}



