#[cfg(test)]
use significance_testing::significance_with_twoactions_on_eatch;
#[test]
pub fn significance_with_compare_eatch_work() {
    

    let a_sample=vec![13.2,8.2,10.9,14.3,10.7,6.6,9.5,10.8,8.8,13.3];
    let b_sample=vec![14.0,8.8,11.2,14.2,11.8,6.4,9.8,11.3,9.3,13.6];
    let rst= significance_with_twoactions_on_eatch(&a_sample,&b_sample,2);
    println!("{}==0.00488",rst);
}