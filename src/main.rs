use data_loader::{load_contingency_table, load_salaries};
use stats::hypothesis_tests;
use tracing::info;
mod data_loader;
mod stats;

static DS_SALARIES: std::sync::LazyLock<&str> =
    std::sync::LazyLock::new(|| "./assets/ds_salaries.csv");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set the global default subscriber");
    let salaries_data_path = std::path::Path::new(*DS_SALARIES);
    let contingency_table = load_contingency_table(salaries_data_path)?;
    let (data_analysts_salaries, data_scientist_salaries) = load_salaries(salaries_data_path)?;
    if data_analysts_salaries.is_empty() || data_scientist_salaries.is_empty() {
        info!("One of the samples is empty, Cannot perform t-test.");
        return Ok(());
    }
    let (chi_square_stat, chisq_p_value) = hypothesis_tests::chi_square_test(contingency_table);

    let (t_stat, df, p_value) =
        hypothesis_tests::t_test_independent(&data_analysts_salaries, &data_scientist_salaries);

    info!("---------- Two-sample T-tests Result ----------");
    info!("Degrees of freedom: {:.2}", df);
    info!("T-statistic: {:.4}", t_stat);
    info!("P-vaue: {:.4}", p_value);

    let alpha = 0.05;
    if p_value < alpha {
        info!(
            "Reject the null hyphotesis. There is a significant difference between
              the average salaries"
        );
    } else {
        info!("Fail to reject the null hyphotesis. No significant difference detected");
    }

    info!("-------------------- Chi-Square Test for independence --------------------");
    info!("Chi-Square statistic: {:.4}", chi_square_stat);
    info!("P-value: {:.4}", chisq_p_value);

    if chisq_p_value < alpha {
        info!(
            "Reject the null hyphotesis. There is an association between Job Title
    and Experience Level"
        );
    } else {
        info!("Fail to reject the null hyphotesis: no significant association detected.")
    }
    Ok(())
}
