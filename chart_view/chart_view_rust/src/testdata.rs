

pub mod control_chart_data{
    use chart::{RControlChart, XbarControlChart};
    //使用函数来获取数据
    pub fn testdata_xbar_chart()->XbarControlChart{
        use data_struct::sheetdata::SubGroubInOneColumn;
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

        let data_in_rows=data_in_column.one_column_into_rows_vec();

        let xbc=XbarControlChart::from_sub_groub_in_rows_vec(&data_in_rows);
        //println!("Xbar_all {:?}",xbc.Xbar_all);
        //println!("ucl{:?}",xbc.ucl);
        //println!("lcl{:?}",xbc.lcl);
        //println!("Xbar_data {:?}",xbc.Xbar_data);
        return xbc;

    }
    pub fn testdata_r_chart()->RControlChart{
        let path="./test_data/子组_R控制图_凸轮轴长度.csv".to_string();
        let data_in_one_column=read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 5);
        let data_in_rows_vec=data_in_one_column.one_column_into_rows_vec();
        //println!("{:?}",data_in_rows_vec.data);
        let r_contr_chart=RControlChart::from_sub_groub_in_rows_vec(&data_in_rows_vec);
        //println!("{:?}",r_contr_chart);
        return r_contr_chart;
   
    }
    


}

