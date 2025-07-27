use statrs::distribution::{ContinuousCDF, StudentsT};
use statrs::statistics::Statistics;

pub(crate) fn t_test_independent(sample1: &[f64], sample2: &[f64]) -> (f64, f64, f64) {
    let n1 = sample1.len() as f64;
    let n2 = sample2.len() as f64;
    let mean1 = sample1.mean();
    let mean2 = sample2.mean();
    let var1 = sample1.variance();
    let var2 = sample2.variance();

    // Degrees of freedom for F-test
    let df1 = n1 - 1.0;
    let df2 = n2 - 1.0;
    // Pooled standard deviation
    let pooled_std = ((df1 * var1 + df2 * var2) / (n1 + n2 - 2.0)).sqrt();
    // test_ statistic
    let t_stat = (mean1 - mean2) / (pooled_std * (1.0 / n1 + 1.0 / n2).sqrt());
    // Degrees of freedom
    let df = n1 + n2 - 2.0;
    // P-value
    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    let p_value = 2.0 * (1.0 - dist.cdf(t_stat.abs()));
    (t_stat, df, p_value)
}
