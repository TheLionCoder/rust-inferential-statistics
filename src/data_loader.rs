use std::collections::HashMap;

use serde::Deserialize;

type Salary = (Vec<f64>, Vec<f64>, Vec<f64>);

#[derive(Debug, Deserialize, PartialEq)]
enum Jobtitle {
    #[serde(rename = "Data Analyst")]
    DataAnalyst,
    #[serde(rename = "Data Scientist")]
    DataScientist,
    #[serde(rename = "Data Engineer")]
    DataEngineer,
    #[serde(other)]
    Other,
}

impl std::fmt::Display for Jobtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[derive(Debug, Deserialize)]
struct SalaryRecord {
    job_title: Jobtitle,
    salary_in_usd: f64,
}

#[derive(Debug, Deserialize)]
struct EmployeeRecord {
    job_title: Jobtitle,
    experience_level: String,
}

pub(crate) fn load_salaries(path: &std::path::Path) -> Result<Salary, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    reader
        .deserialize()
        .try_fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, result| {
            let record: SalaryRecord = result?;
            match record.job_title {
                Jobtitle::DataScientist => acc.0.push(record.salary_in_usd),
                Jobtitle::DataAnalyst => acc.1.push(record.salary_in_usd),
                Jobtitle::DataEngineer => acc.2.push(record.salary_in_usd),
                Jobtitle::Other => (),
            }
            Ok(acc)
        })
}

pub(crate) fn load_contingency_table(
    path: &std::path::Path,
) -> Result<HashMap<(String, String), usize>, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    reader
        .deserialize()
        .try_fold(HashMap::new(), |mut acc, result| {
            let record: EmployeeRecord = result?;

            let key = (record.job_title.to_string(), record.experience_level);
            *acc.entry(key).or_insert(0) += 1;
            Ok(acc)
        })
}
