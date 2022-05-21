#[cfg(test)]
mod tests {
    use crate::csvreadtodata::{csv_one_column_into_subgroubinonecolumn, csv_rows_into_subgroubinrows};

    #[test]
    fn csv_one_column_into_subgroubinonecolumn_works() {
        assert_eq!(2 + 2, 4);
        let path="./test_data/子组_R控制图_凸轮轴长度.csv".to_string();
        let data=csv_one_column_into_subgroubinonecolumn(path, 1, 5);
        println!("{:?}",data.data);
    
    }

    #[test]
    fn csv_rows_into_subgroubinrows_works() {
        assert_eq!(2 + 2, 4);
        let path="./test_data/子组_R控制图_凸轮轴长度.csv".to_string();
        let data=csv_rows_into_subgroubinrows(path);
        println!("{:?}",data.data);
    
    }


}
