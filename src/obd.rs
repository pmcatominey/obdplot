use std::{collections::HashMap, error::Error, fmt};

pub struct CsvLog {
    pub file_path: String,

    pub x_col: Column,
    pub data_cols: Vec<Column>,
}

#[derive(Debug)]
pub struct Column {
    pub header: String,
    pub values: Vec<f64>,
}

type Record = HashMap<String, f64>;

#[derive(Debug)]
pub enum ObdError {
    LogEmpty,
    Unknown,
}

impl CsvLog {
    pub fn from_file(file_path: String) -> Result<CsvLog, Box<dyn Error>> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(&file_path)?;

        let x_col_header = reader
            .headers()?
            .get(0)
            .ok_or(ObdError::LogEmpty)?
            .to_owned();

        let recordse: Result<Vec<Record>, csv::Error> = reader.deserialize::<Record>().collect();
        let records = recordse?;

        let mut data_cols: Vec<Column> = records
            .first()
            .ok_or(ObdError::LogEmpty)?
            .keys()
            .map(|k| {
                let vals = records
                    .iter()
                    .map(|r| r.get(k).unwrap_or(&0_f64).to_owned())
                    .collect();
                Column {
                    header: k.to_string(),
                    values: vals,
                }
            })
            .collect();

        let x_col_index = data_cols
            .iter()
            .position(|c| c.header == x_col_header)
            .ok_or(ObdError::Unknown)?;
        let x_col = data_cols.remove(x_col_index);

        Ok(CsvLog {
            file_path,
            x_col,
            data_cols,
        })
    }
}

impl fmt::Display for ObdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No rows in CSV")
    }
}

impl Error for ObdError {}
