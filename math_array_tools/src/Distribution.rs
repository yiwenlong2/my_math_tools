use crate::{Basic::mean, Statistic::{standard_variance_totality, standard_variance_sample}};

pub struct Normal{
    pub mean:f64,
    pub standard_variance:f64,
}

impl Normal {
    pub fn from_vec_totality<U: Copy + std::ops::Add<Output = U> + std::convert::Into<f64>>(array:&Vec<U>) ->Normal{
        let mut n=Normal{mean:0.0,standard_variance:1.0,};
        n.mean=mean(&array);
        n.standard_variance=standard_variance_totality(&array);
        return n;
    }
    pub fn from_vec_sample<T: Copy + std::ops::Add<Output = T> + std::convert::Into<f64>>(array:&Vec<T>)->Normal {
        let mut n=Normal{mean:0.0,standard_variance:1.0,};
        n.mean=mean(&array);
        n.standard_variance=standard_variance_sample(&array);
        return n;
    }
}
