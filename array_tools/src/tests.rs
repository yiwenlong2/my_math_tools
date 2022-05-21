
#[cfg(test)]
mod tests {
    use crate::{CompareResultIndex, create_vec_with_no_repeat, find_index_in_ranked_vec, find_value_norepeat_out_index, find_value_repeat_out_indexs, remove_repeat_value, remove_repeat_value_out_ranked};

    #[test]
    fn create_works() {
        let amount=1000usize;
        let array=create_vec_with_no_repeat(amount, 0.0, 10000.0);
        assert_eq!(amount, array.len());
    }
    #[test]
    fn find_no_repeat_works(){
        let a1=vec![0,2,4,5,7,8];
        let (index1,result1)=find_value_norepeat_out_index(&a1, 0);
        let (index2,result2)=find_value_norepeat_out_index(&a1, 8);
        let (index3,result3)=find_value_norepeat_out_index(&a1, 12);

        assert_eq!((0,true),(index1,result1));
        assert_eq!((5,true),(index2,result2));
        assert_eq!((0,false),(index3,result3));

    }
    #[test]
    fn find_value_repeat_out_indexs_works(){
        let a1=vec![0,0,3,4,6,8,8];
        let (indexs1,result1)=find_value_repeat_out_indexs(&a1, 0);
        let (indexs2,result2)=find_value_repeat_out_indexs(&a1, 8);
        let (indexs3,result3)=find_value_repeat_out_indexs(&a1, 10);
        
        assert_eq!((vec![0,1],true), (indexs1,result1));
        assert_eq!((vec![5,6],true), (indexs2,result2));
        assert_eq!((vec![],false), (indexs3,result3));

    }
    #[test]
    fn remove_repeat_value_out_ranked_works(){
        let mut a1=vec![0,0,1,2,3,3,3,4,5,6,6];
        remove_repeat_value_out_ranked(&mut a1);

        assert_eq!(vec![0,1,2,3,4,5,6],a1);
    }
    #[test]
    fn remove_repeat_value_works(){
        let mut a1=vec![0,4,3,4,6,8,5,5,5];
        remove_repeat_value(&mut a1);
        //println!("{:?}",a1);
        assert_eq!(vec![0,4,3,6,8,5],a1);
    }
    #[test]
    fn find_index_in_ranked_vec_works(){
        let a1=vec![-4.0,-3.0,0.0,1.0,4.0,6.0];
        let rst=find_index_in_ranked_vec(&a1, -3.5);
        match rst {
            CompareResultIndex::EqualityIndex(i)=>println!("EqualityIndex:{}",i),
            CompareResultIndex::MiddleLeftIndex(i)=>println!("MiddleIndex:{}",i),
            CompareResultIndex::NearLeftIndex(i)=>println!("NearLeftIndex:{}",i),
            CompareResultIndex::NearRightIndex(i)=>println!("NearRightIndex:{}",i),
            CompareResultIndex::BiggerThanMax=>println!("Bigger than Max"),
            CompareResultIndex::LessThanMin=>println!("Less than Min"),
        }
    }

}
