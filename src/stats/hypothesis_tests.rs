use std::collections::{HashMap, HashSet};

use statrs::distribution::{ChiSquared, ContinuousCDF, StudentsT};
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

pub(crate) fn chi_square_test(contingency_table: HashMap<(String, String), usize>) -> (f64, f64) {
    // Extract unique job tables and experience levels
    let mut unique_job_titles = HashSet::new();
    let mut unique_experience_levels = HashSet::new();

    for (job, experience_lvl) in contingency_table.keys() {
        unique_job_titles.insert(job.clone());
        unique_experience_levels.insert(experience_lvl.clone());
    }

    let unique_job_titles_vec = unique_job_titles.into_iter().collect::<Vec<_>>();
    let unique_experience_levels_vec = unique_experience_levels.into_iter().collect::<Vec<_>>();

    let mut observed = Vec::new();
    let mut expected = Vec::new();

    let total_count = contingency_table.values().sum::<usize>();

    for job in &unique_job_titles_vec {
        for exp in &unique_experience_levels_vec {
            let observed_count = *contingency_table
                .get(&(job.clone(), exp.clone()))
                .unwrap_or(&0);
            observed.push(observed_count as f64);

            // Calculate marginals totals
            let row_total = unique_experience_levels_vec
                .iter()
                .map(|e| {
                    *contingency_table
                        .get(&(job.clone(), e.clone()))
                        .unwrap_or(&0)
                })
                .sum::<usize>();
            let column_total = unique_job_titles_vec
                .iter()
                .map(|j| {
                    *contingency_table
                        .get(&(j.clone(), exp.clone()))
                        .unwrap_or(&0)
                })
                .sum::<usize>();

            let expected_count = (row_total as f64) * (column_total as f64) / (total_count as f64);
            expected.push(expected_count);
        }
    }

    // Calculate chi-square statistic
    let chi_square_stat = observed
        .iter()
        .zip(expected.iter())
        .map(|(o, e)| {
            if *e != 0.0 {
                (*o - *e).powi(2) / *e
            } else {
                0.0
            }
        })
        .sum::<f64>();
    // Degrees of freedom
    let df = (unique_job_titles_vec.len() - 1) * (unique_experience_levels_vec.len() - 1);
    // P-value
    let dist = ChiSquared::new(df as f64).unwrap();
    let p_value = 1.0 - dist.cdf(chi_square_stat);
    (chi_square_stat, p_value)
}

pub(crate) fn paired_t_test(sample1: &[f64], sample2: &[f64]) -> Option<(f64, f64, f64)> {
    if sample1.len() != sample2.len() || sample2.len() < 2 {
        return None;
    }

    let n = sample1.len() as f64;
    let (sum_diff, sum_sq_diff) = sample1
        .iter()
        .zip(sample2)
        .map(|(x, y)| x - y)
        .fold((0.0, 0.0), |(sum, sum_sq), diff| {
            (sum + diff, sum_sq + diff.powi(2))
        });
    let mean_diff = sum_diff / n;
    let variance = (sum_sq_diff - sum_diff.powi(2) / n) / (n - 1.0);
    let std_diff = variance.sqrt();

    let std_error = std_diff / n.sqrt();
    let t_stat = if std_error > 0.0 {
        mean_diff / std_error
    } else {
        f64::INFINITY
    };
    let df = n - 1.0;

    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    let p_value = 2.0 * (1.0 - dist.cdf(t_stat.abs()));
    Some((t_stat, df, p_value))
}
