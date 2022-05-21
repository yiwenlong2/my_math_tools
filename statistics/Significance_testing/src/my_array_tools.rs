

//目标值在数列中比较后的结果
pub enum CompareResultIndex {
    BiggerThanMax,
    LessThanMin,
    Index(usize),
}

//在数组array:&Vec<f64> 中 找到<value或>value 的index, 自该index 之后的数>=value或=<value;
//返回值是枚举体Compare_index, 如果value 在array之外, 返回Bigger/Less than max;
pub fn find_index_biger_less_tan_value(array:&Vec<f64>,value:f64)->CompareResultIndex {

    let las_one_of_index=array.len()-1;
    let mut index=0usize;

    if value<array[0]{
        return CompareResultIndex::LessThanMin;
    }
    else if value>array[las_one_of_index] {
        return CompareResultIndex::BiggerThanMax;
    } 
    else if value >0f64 {
        for i in 0..array.len() {
            if value > array[i] {
                
            }
            /* else if value==array[i] {
                index=i;
                return CompareResultIndex::Index(index);
                
            } */
            else {
                index=i-1;
                
                return CompareResultIndex::Index(index);
            }
        }
    }
    else if value <0f64 {
        for i in 0..array.len() {
            if value >array[i]{

            }
            /* else if value==array[i] {
                index=i;
                return CompareResultIndex::Index(index);
                
            } */
            else {
                index=i-1;
               
                return CompareResultIndex::Index(index);
            }
        }
        
    }

    return CompareResultIndex::Index(index);

}


