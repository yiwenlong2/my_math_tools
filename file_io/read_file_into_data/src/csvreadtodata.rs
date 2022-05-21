use data_struct::sheetdata::{SubGroubInOneColumn, SubGroubInRows};

pub fn csv_one_column_into_subgroubinonecolumn(path:String,column_n:usize,split_n:usize) ->SubGroubInOneColumn<f64> {
        let mut recoder=csv::Reader::from_path(path).unwrap();
        let mut vec_data:Vec<f64>=Vec::new();


        for i in recoder.records() {
            let temp=i.unwrap();
            let temp_string=temp.get(column_n-1).unwrap().to_string();
            let value_f64:f64=temp_string.trim().parse().unwrap();

            vec_data.push(value_f64);
        }
        //println!("vec_data:{:?}",vec_data);
        //println!("vec_data.len() {}",vec_data.len());


        let data_in_column=SubGroubInOneColumn {
            groub_split_n:split_n,
            data:vec_data,
        };

        return data_in_column;
}

pub fn  csv_rows_into_subgroubinrows(path:String)->SubGroubInRows<f64>{
    let mut recoder=csv::Reader::from_path(path).unwrap();
    let mut vec_data:Vec<Vec<f64>>=Vec::new();
   


    for i in recoder.records() {
        let temp=i.unwrap();
         let mut vec_data_row:Vec<f64>=Vec::new();
        for j in temp.iter(){
            let temp_string=j.to_string();
            let value_f64:f64=temp_string.trim().parse().unwrap();
            vec_data_row.push(value_f64);
        }
        vec_data.push(vec_data_row);
    }
    let data_in_rows=SubGroubInRows{
        data: vec_data,
    };
    return data_in_rows;


}