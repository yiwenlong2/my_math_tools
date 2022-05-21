

#[cfg(test)]
mod basic {
    use crate::{Basic::{float_output_format, max, mean, min}, Statistic::median};
    #[test]
    fn func_basic_works() {
        let array_i32 = vec![1, 2, 3, 4, 5];
        let array_float = vec![1.0, 2.0, 3.0, 4.0, 5.5];
        let mean_i32 = mean(&array_i32);
        let mean_float = mean(&array_float);
        //println!("{}",mean_i32);
        //println!("{}",mean_float);
        assert_eq!(mean_i32, 3.0);
        assert_eq!(mean_float, 3.1);
        assert_eq!(max(&array_i32), 5);
        assert_eq!(min(&array_i32), 1);
        assert_eq!(max(&array_float), 5.5);
        assert_eq!(min(&array_float), 1.0);
    }
    #[test]
    fn func_float_output_format_work(){
        let float:f64=3.141592689;
        let n=2;
        let rst=float_output_format(float, n);
        assert_eq!(rst,3.14)
    }
}

mod statistic {
    use crate::{Basic::float_output_format, Statistic::{autocorrelation_cofficient, correlation_coefficient_totality, get_less_than_value_standard_normal_probability, mean, median, standard_variance_sample, variance_sample}};
    #[test]
    fn func_median_work() {
        let array_i32 = vec![1, 2, 3, 4, 5];
        let array_float = vec![1.0, 2.0, 3.0, 4.0, 5.5];
        let median_i32 = median(&array_i32);
        let median_float = median(&array_float);
        assert_eq!(median_i32, 3.0);
        assert_eq!(median_float, 3.0);
        //println!("{}",median_i32);
        //println!("{}",median_float);

        let array_i32 = vec![1, 2, 3, 4, 5, 6];
        let array_float = vec![1.0, 2.0, 3.2, 4.2, 5.0, 6.0];
        let median_i32 = median(&array_i32);
        let median_float = median(&array_float);
        //println!("{}",median_i32);
        //println!("{}",median_float);
        assert_eq!(median_i32, 3.5);
        assert_eq!(median_float, 3.7);
    }

    #[test]
    fn func_variance_work() {
        let array = vec![24, 37, 38, 43, 33, 35, 48, 29, 30, 38];

        let mean = mean(&array);
        let variance_sample_value = variance_sample(&array);
        let standard_variance_sample_value=standard_variance_sample(&array);
        println!("mean: {}==35.5", mean);
        println!("variance_sample: {}==48.72", variance_sample_value);
        //assert_eq!(mean,35.5);
        //assert_eq!(variance_sample_value,48.72);
        println!("standard_variance_sample: {}==6.9799",standard_variance_sample_value);

    }

    #[test]
    fn func_get_standard_normal_probability_work() {
        let y12=get_less_than_value_standard_normal_probability(4.2);
        println!("{}",y12);
    }

    #[test]
    fn func_cov_variance_correlation_coefficient(){
        let array1=vec![65,68,67,70,75];
        let array2=vec![150,130,170,180,220];
        let correlation_coefficient_value=correlation_coefficient_totality(&array1, &array2);
        //println!("{:.2}==0.83",correlation_coefficient_value);
        let rst=float_output_format(correlation_coefficient_value, 2);
        //println!("{}",&rst);
        assert_eq!(rst,0.83);
    }
    #[test]
    fn func_autocorrelation_coefficient() {
        let array=vec![3,6,9,8,7,5,4];
        let rst=autocorrelation_cofficient(&array, 1);
        println!("{}==0.22",rst);
    }


}
mod distribution_test {
    use std::fs;

    use crate::Distribution::Normal;

    #[test]
    fn from_vec_all_work() {
        let f=fs::read_to_string("./test_data/normaldata.CSV").unwrap();
        let normal_test_vec_string:Vec<&str>=f.split('\n').collect();
        let mut normal_test_vec:Vec<f64>=Vec::new();
        for i in normal_test_vec_string {

            if i !="" {
            let temp:f64=i.to_string().trim().parse().unwrap();
            //println!("{}",temp);
            normal_test_vec.push(temp)
            }
        }
        //println!("{:?}",normal_test_vec);
        let n1=Normal::from_vec_totality(&normal_test_vec);
        println!("File:normaldata\nmean:6,Sigma:1");
        println!("Nomal_all: \nmean={},Sigma={}",n1.mean,n1.standard_variance);
        let n1=Normal::from_vec_sample(&normal_test_vec);
        println!("Nomal_sample: \nmean={},Sigma={}",n1.mean,n1.standard_variance);

        let f=fs::read_to_string("./test_data/normaldata_fromweb.CSV").unwrap();
        let normal_test_vec_string:Vec<&str>=f.split('\n').collect();
        let mut normal_test_vec:Vec<f64>=Vec::new();
        for i in normal_test_vec_string {

            if i !="" {
            let temp:f64=i.to_string().trim().parse().unwrap();
            //println!("{}",temp);
            normal_test_vec.push(temp)
            }
        }
        let n1=Normal::from_vec_totality(&normal_test_vec);
        println!("File:normaldata_fromweb\nmean:28.83\nSigma:7.92268");
        println!("Nomal_all: \nmean={},Sigma={}",n1.mean,n1.standard_variance);
        let n1=Normal::from_vec_sample(&normal_test_vec);
        println!("Nomal_sample: \nmean={},Sigma={}",n1.mean,n1.standard_variance);

    }
}
