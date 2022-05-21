#[cfg(test)]
mod control_charts {
    use crate::{IControlChart, MRControlChart, RControlChart, SControlChart, XbarControlChart};
    use data_struct::sheetdata::ItemsInOneColumn;
    use read_file_into_data;

    #[test]
    fn xbar_chart_work(){
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
        println!("Xbar_all {:?}",xbc.Xbar_all);
        println!("ucl{:?}",xbc.ucl);
        println!("lcl{:?}",xbc.lcl);
        println!("Xbar_data {:?}",xbc.Xbar_data);

    }
    #[test]
    fn r_chart_work(){
        let path="./test_data/子组_R控制图_凸轮轴长度.csv".to_string();
        let data_in_one_column=read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 5);
        let data_in_rows_vec=data_in_one_column.one_column_into_rows_vec();
        //println!("{:?}",data_in_rows_vec.data);
        let r_contr_chart=RControlChart::from_sub_groub_in_rows_vec(&data_in_rows_vec);
        println!("{:?}",r_contr_chart);
        println!("{:?}",r_contr_chart.R_data.len());
   
    }

    #[test]
    fn s_chart_work(){
        let path="./test_data/sheetdata_in_one_column.csv".to_string();
        let data_in_one_column=read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 2, 10);
        let sbgb_in_rows=data_in_one_column.one_column_into_rows_vec();
        let scc=SControlChart::from_sub_groub_in_rows_vec(&sbgb_in_rows);
        println!("{:?}",&scc);
    }

    #[test]
    fn I_chart_work(){
        let path="./test_data/洗涤剂pH值.csv".to_string();
        let data_in_one_column_temp=read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 1);
        let data_in_one_column= ItemsInOneColumn {
            data: data_in_one_column_temp.data,
        };
        let i_control_chart=IControlChart::from_items_in_one_column(&data_in_one_column);

        print!("{:?}",i_control_chart);
    
    }

    #[test]
    fn MR_chart_work(){
        let path="./test_data/洗涤剂pH值.csv".to_string();
        let data_in_one_column_temp=read_file_into_data::csvreadtodata::csv_one_column_into_subgroubinonecolumn(path, 1, 1);
        let data_in_one_column= ItemsInOneColumn {
            data: data_in_one_column_temp.data,
        };
        let mr_control_chart=MRControlChart::from_items_in_one_column(&data_in_one_column,2,1);
        println!("{:?}",mr_control_chart);
    }



    
}

mod constantbase {
    
    #[test]
    fn dn_work() {
        use crate::constantbase::{d2,d3, d4};
        assert_eq!(3.931,d2(25));
        assert_eq!(0.7084,d3(25));
        assert_eq!(3.883,d4(25));
    }
    
}

mod evaluate_sigma_test{
    
    #[test]
    fn sigma_from_moving_range_average_work(){
        use crate::evaluate_sigma;
        let array=vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
        let mr_list=evaluate_sigma::sigma_from_moving_range_average(&array, 3);
        println!("sigma_from_moving_range_average: {}",mr_list);
    }
}