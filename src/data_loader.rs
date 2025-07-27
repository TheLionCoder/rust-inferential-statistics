use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SalaryRecord {
    job_title: String,
    salary_in_usd: f64,
}

pub(crate) fn load_salaries() -> Result<(Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("../assets/ds_salaries.csv")?;

    let mut data_scientists = Vec::new();
    let mut data_analysts = Vec::new();

    for result in reader.deserialize() {
        let record: SalaryRecord = result?;

        match record.job_title.as_str() {
            "Data Analyst" => data_analysts.push(record.salary_in_usd),
            "Data Scientist" => data_scientists.push(record.salary_in_usd),
            _ => (),
        }
    }

    Ok((data_scientists, data_analysts))
}
