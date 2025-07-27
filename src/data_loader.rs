use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SalaryRecord {
    job_title: String,
    salary_in_usd: f64,
}

#[derive(Debug, Deserialize)]
struct EmployeeRecord {
    job_title: String,
    experience_level: String,
}

pub(crate) fn load_salaries(
    path: &std::path::Path,
) -> Result<(Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

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

pub(crate) fn load_contingency_table(
    path: &std::path::Path,
) -> Result<HashMap<(String, String), usize>, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut contingency_table = HashMap::new();
    for result in reader.deserialize() {
        let record: EmployeeRecord = result?;
        *contingency_table
            .entry((record.job_title, record.experience_level))
            .or_insert(0) += 1;
    }
    Ok(contingency_table)
}
