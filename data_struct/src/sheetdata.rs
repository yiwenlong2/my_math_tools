pub struct SheetBase {
    pub row_title: Vec<String>,
    pub column_title: Vec<String>,
    pub data: Vec<Vec<f64>>,
}
//每次都在短时间内测量多个样本，这几个样本构成一个子组。这种测量方式与对同一个样本测量多次不同；
//所有子组在同一列
pub struct SubGroubInOneColumn<T: Copy> {
    pub groub_split_n: usize,
    pub data: Vec<T>,
}
impl<T:Copy> SubGroubInOneColumn<T> {
    //将一列数据，转换为行，但是同一个标号在同一个Vec<T>内，此时一个Vec<T> 相当于二维表的一个列；
    pub fn one_column_into_columns_vec(&self) -> SubGroubInRows<T> {
        let mut data_vec: Vec<Vec<T>> = Vec::new();
        println!("{}",self.groub_split_n);
        for i in 0..self.groub_split_n {
            
            let temp: Vec<T> = Vec::new();
            data_vec.push(temp);
            //println!("{} data_vec.len: {}",i,data_vec.len());
        }
        //println!("data_vec.len: {}",data_vec.len());
        let mut j=0usize;
        for i in self.data.iter() {
            data_vec[j].push(*i);
            if j==9 {
                j=0;
            }
            else {
                j=j+1;
            }
        }
        let vec_in_rows = SubGroubInRows { data: data_vec };
        return vec_in_rows;
    }
    
    //将一列数据，转换为行，同一个子组在同一个Vec<T>内；此时一个Vec<T> 相当于二维表的一个行；
    pub fn one_column_into_rows_vec(&self)->SubGroubInRows<T>{
        let mut vec_temp:Vec<T>=Vec::new();
        let mut vec_rows:Vec<Vec<T>>=Vec::new();
        let mut n=0usize;
        for i in self.data.iter(){
            vec_temp.push(*i);
            n=n+1;
            if n==self.groub_split_n {
                //println!("vect_temp.len():{}",vec_temp.len());
                vec_rows.push(vec_temp.clone());
                vec_temp.clear();
                n=0;
                
                //println!("vec_rows.len():{}",vec_rows.len());
            }

        }
        let sbrws=SubGroubInRows{data:vec_rows};
        return sbrws;
    }
}

//一个子组在二维表的同一行；
pub struct SubGroubInRows<T: Copy> {
    pub data: Vec<Vec<T>>,
}

impl<T: Copy> SubGroubInRows<T> {

    //一个Vec<T>为二维表的一列，将多个Vec<T>列转换为一列，原同一个index的Vector<T>被认为是一个子组，同子组的数据在转换后是挨着的，groub_split_n为子组数据的分割线；
    pub fn columns_into_one_column(&self) -> SubGroubInOneColumn<T> {
        let mut vec_in_column: SubGroubInOneColumn<T> = SubGroubInOneColumn {
            groub_split_n: self.data.len(),
            data: Vec::new(),
        };
        //println!("the rows count is: {}",self.data[0].len());
        //println!("the column count is: {}",self.data.len());
        for i in 0..self.data[0].len() {
            for j in 0..self.data.len() {
                vec_in_column.data.push(self.data[j][i]);
            }
        }
        return vec_in_column;
    }
    //一个Vec<T> 为二维表的一行，多个Vec<T>拼接为一列，首尾相接；
    pub fn rows_into_one_column(&self)->SubGroubInOneColumn<T> {
        let mut vec_in_one_column:Vec<T>=Vec::new();
        for i in self.data.iter(){
           
            vec_in_one_column.append(&mut i.clone());
        }

        let sbgbincolumn=SubGroubInOneColumn{
            groub_split_n:self.data.len(),
            data:vec_in_one_column,
        };

        return sbgbincolumn;

    }
}

pub struct SubGroubInColumns<T: Copy> {
    pub data: Vec<Vec<T>>,
}

impl<T:Copy> SubGroubInColumns<T> {
    pub fn columns_into_one_column(&self) -> SubGroubInOneColumn<T> {
        // //一个Vec<T>为二维表的一列，将多个Vec<T>列转换为一列，原同一个index的Vector<T>被认为是一个子组，同子组的数据在转换后是挨着的，groub_split_n为子组数据的分割线；
        let mut vec_in_column: SubGroubInOneColumn<T> = SubGroubInOneColumn {
            groub_split_n: self.data.len(),
            data: Vec::new(),
        };
        //println!("the rows count is: {}",self.data[0].len());
        //println!("the column count is: {}",self.data.len());
        for i in 0..self.data[0].len() {
            for j in 0..self.data.len() {
                vec_in_column.data.push(self.data[j][i]);
            }
        }
        return vec_in_column;
    }
}

pub struct ItemsInOneColumn<T:Copy>{
    pub data:Vec<T>,
}
