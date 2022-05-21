#[cfg(test)]
pub mod tests_list {
    use crate::{n2_permulation, sheetdata::SubGroubInOneColumn, traverse_branch_sum};

    extern crate csv;
    #[test]
    fn traverse_branch_sum_works() {
        let array1=vec![1,2,3,4];
        let array2=vec![-1,-2,-3,-4];
        let head_value=0;
        let array_head=n2_permulation(&array1, &array2, head_value);
        let sum=0;
        let mut sum_vec:Vec<i32>=Vec::new();
        traverse_branch_sum(array_head, sum, &mut sum_vec);

        let rst=vec![10,2,4,-4,6,-2,0,-8,8,0,2,-6,4,-4,-2,-10];

        assert_eq!(sum_vec,rst);
    }
    
    #[test]
    fn sheetdata_works() {
        let path="./test_data/sheetdata_in_one_column.csv";
        let mut recoder=csv::Reader::from_path(path).unwrap();

        let mut vec_string:Vec<String>=Vec::new();
        let mut vec_data:Vec<f64>=Vec::new();


        for i in recoder.records() {
            let temp=i.unwrap();
            //println!("{:?}",temp);
            for i in 0..2 {
                let temp_inside=temp.get(i).unwrap();
                match i {
                    0=>vec_string.push(temp_inside.to_string()),
                    1=>{
                        let temp_f64:f64=temp_inside.trim().parse().unwrap();
                        //println!("{}",temp_f64);
                        vec_data.push(temp_f64);

                    },
                    _ =>() ,
                }
            }
        }
        //println!("{:?}",vec_data);
        //println!("vec_data.len() {}",vec_data.len());
        let mut groub_split_n=0usize;
        for i in 0..vec_string.len() {
            if vec_string[0]!=vec_string[i] {
                groub_split_n=i;
                //println!("the groub_split_n is: {}",groub_split_n);
                break;
            }
        }

        let data_in_column=SubGroubInOneColumn {
            groub_split_n:groub_split_n,
            data:vec_data,
        };

        //let data_in_columns=data_in_column.one_column_into_columns_vec();
    
        //let data_in_column_1=data_in_columns.columns_into_one_column();

        let data_in_rows=data_in_column.one_column_into_rows_vec();
        for i in data_in_rows.data.iter(){
            println!("{:?}",&i);
        } 
        
        let data_in_column_2=data_in_rows.rows_into_one_column();
        println!("{:?}",data_in_column_2.data);


    }
    
}


