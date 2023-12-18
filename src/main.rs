use csv::Writer;
use pdf_extract::*;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reports_folder = Path::new("reports");

    for entry in fs::read_dir(reports_folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().unwrap_or_default() == "pdf" {
            let mut content = String::new();
            let mut file = File::open(&path)?;
            file.read_to_string(&mut content)?;

            // Parse the PDF content here
            let transactions = parse_pdf_content(content);

            // Convert to CSV
            let csv_file_path = path.with_extension("csv");
            write_csv(csv_file_path, transactions)?;
        }
    }

    Ok(())
}

fn parse_pdf_content(content: String) -> Vec<Transaction> {
    // Your PDF parsing logic goes here
    // Return a vector of transactions
    // You might need to adjust this based on the structure of your PDFs
    println!("{:?}", content);
    vec![]
}

fn write_csv(
    path: PathBuf,
    transactions: Vec<Transaction>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path(path)?;

    // Write header
    wtr.write_record(&[
        "Date",
        "Description",
        "Amount",
        "Balance",
        "Bank Account",
        "Comment",
    ])?;

    // Write rows
    for transaction in transactions {
        wtr.write_record(&[
            transaction.date,
            transaction.description,
            transaction.amount.to_string(),
            transaction.balance.to_string(),
            transaction.bank_account,
            transaction.comment,
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

// Define a Transaction struct based on your requirements
struct Transaction {
    date: String,
    description: String,
    amount: f64,
    balance: f64,
    bank_account: String,
    comment: String,
}
