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
    let alpha = 0.05;
    match load_salaries(salaries_data_path) {
        Ok((data_scientist_salaries, data_analysts_salaries, data_engineers_salaries)) => {
            if data_analysts_salaries.is_empty() || data_scientist_salaries.is_empty() {
                info!("One of the samples is empty, Cannot perform t-test.");
                return Ok(());
            }

            let (t_stat, df, p_value) = hypothesis_tests::t_test_independent(
                &data_analysts_salaries,
                &data_scientist_salaries,
            );

            if let Ok((f_stat, f_p_value)) = stats::anova::one_way_anova(&[
                &data_analysts_salaries,
                &data_scientist_salaries,
                &data_engineers_salaries,
            ]) {
                info!("One-Way ANOVA (1 - α= {:.2}):", 1.0 - alpha);
                info!("F-statistic: {:.4}", f_stat);
                info!("Critical value: {:.4}", f_p_value);
                if f_stat > f_p_value {
                    info!("There is a significant difference between the groups.");
                } else {
                    info!("The is no significant difference between the groups.")
                }
            }

            info!("---------- Two-sample T-tests Result ----------");
            info!("Degrees of freedom: {:.2}", df);
            info!("T-statistic: {:.4}", t_stat);
            info!("P-vaue: {:.4}", p_value);

            if p_value < alpha {
                info!(
                    "Reject the null hyphotesis. There is a significant difference between the average salaries"
                );
            } else {
                info!("Fail to reject the null hyphotesis. No significant difference detected");
            }
        }
        Err(e) => {
            tracing::error!("Error loading salaries: {}", e);
        }
    }

    let (chi_square_stat, chisq_p_value) = hypothesis_tests::chi_square_test(contingency_table);
    info!("-------------------- Chi-Square Test for independence --------------------");
    info!("Chi-Square statistic: {:.4}", chi_square_stat);
    info!("P-value: {:.4}", chisq_p_value);

    if chisq_p_value < alpha {
        info!(
            "Reject the null hyphotesis. There is an association between Job Title and Experience Level"
        );
    } else {
        info!("Fail to reject the null hyphotesis: no significant association detected.")
    }
    Ok(())
}
