use statrs::distribution::{ContinuousCDF, FisherSnedecor};
pub fn one_way_anova(groups: &[&[f64]]) -> Result<(f64, f64), &'static str> {
    let k = groups.len() as f64;

    if k < 2.0 {
        return Err("ANOVA requires at leats two groups");
    }

    // N observations
    let n_total: usize = groups.iter().map(|&g| g.len()).sum();
    let n_total_f64 = n_total as f64;

    if n_total_f64 <= k {
        return Err("The total number of observations must be greather than
     the number of groups");
    }

    // Calculate mean
    let total_sum: f64 = groups.iter().flat_map(|&g| g.iter()).sum();
    let grand_mean: f64 = total_sum / n_total_f64;

    // Sum of squares Between groups (SSB)
    let ssb = groups
        .iter()
        .filter(|g| !g.is_empty())
        .map(|&g| {
            let group_len = g.len() as f64;
            let group_mean = g.iter().copied().sum::<f64>() / group_len;
            group_len * (group_mean - grand_mean).powi(2)
        })
        .sum::<f64>();
    // Sum of squares within groups (SSW) or SSE
    let ssw = groups
        .iter()
        .filter(|g| !g.is_empty())
        .map(|g| {
            let group_len = g.len() as f64;
            let group_mean = g.iter().sum::<f64>() / group_len;
            g.iter().map(|&x| (x - group_mean).powi(2)).sum::<f64>()
        })
        .sum::<f64>();

    // Calculates degrees of freedom
    let df_between = k - 1.0;
    let df_within = n_total_f64 - k;

    // Calculate mean squares
    let ms_between = ssb / df_between;

    if df_within == 0.0 {
        return Err("Cannot compute with 0 degrees of freedom within groups.");
    }

    let ms_within = ssw / df_within;
    if ms_within == 0.0 {
        return Ok((f64::INFINITY, 0.0));
    }

    // Calculate f-statistic
    let f_statistic = ms_between / ms_within;

    // Calculate p_value
    let f_dist = FisherSnedecor::new(df_between, df_within).map_err(|_| {
        "Failed to create F-distribution. Check 
         degrees of freedom"
    })?;
    let p_value = 1.0 - f_dist.cdf(f_statistic);
    Ok((f_statistic, p_value))
}
