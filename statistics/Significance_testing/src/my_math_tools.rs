
//使用外部crate::combination;
use combination::combine;

pub mod Probability {
    //求n的阶乘;
    pub fn factorial(n:usize) ->usize{
        let mut sum:usize =1;
        for i in 1..=n {
            sum=i*sum;
        }
        return sum;
    }
    //求在n个中取m个组合数;
    pub fn combination(n:usize,m:usize) ->usize{
        let mut rst:usize=1;
        if m>n {
            panic!("m is bigger than n");
        }
        else if m==n {
            return rst;
        }
        else {
            rst= (factorial(n))/(factorial(m)*factorial(n-m));
        }
        return rst;
    }
    //求在n个中取m个排列的数;
    pub fn permutation(n:usize,m:usize) ->usize{
        let mut rst:usize=1;
        if m>n {
            panic!("m is bigger than n");
        }
        else if m==n {
             return rst;
        }
        else {
            rst=(factorial(n))/(factorial(n-m));
        }

        return rst;


    }
    //列出在n个中取m个组合的所有组合的编号和剩下的元素组成的编号;
    //直接列出排列或组合数使用combination::premutate或combine(&Vec<T>)
    pub fn combination_index(n:usize,m:usize) ->(Vec<Vec<usize>>,Vec<Vec<usize>>){
        let combination_index=combination::combine::index(n,m);
        let mut combination_index_others:Vec<Vec<usize>>=Vec::new();

        for combination_index_item in combination_index.iter() {
            let mut temp:Vec<usize>=Vec::new();
            for i in 0..n {
                if !combination_index_item.contains(&i) {
                    temp.push(i);
                }
            }
            combination_index_others.push(temp);
    
        }
        return (combination_index,combination_index_others);
    }

}