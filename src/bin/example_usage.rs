// Example usage of the count_csv_column_values function
use std::collections::HashMap;

// This would be the function from src/readcards.rs
fn count_csv_column_values(fname: &str) -> Result<Vec<HashMap<String, i32>>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(fname)?;
    
    // Get headers to know column names
    let headers = rdr.headers()?.clone();
    let num_columns = headers.len();
    
    // Initialize a vector of HashMaps, one for each column
    let mut column_counts: Vec<HashMap<String, i32>> = vec![HashMap::new(); num_columns];
    
    // Process each record
    for result in rdr.records() {
        let record = result?;
        
        // For each field in the record, try to parse as integer and count
        for (col_index, field) in record.iter().enumerate() {
            if col_index < num_columns {
                // Try to parse the field as an integer
                if let Ok(int_value) = field.parse::<i32>() {
                    let count_map = &mut column_counts[col_index];
                    let key = int_value.to_string();
                    *count_map.entry(key).or_insert(0) += 1;
                }
                // If it's not an integer, we skip it (don't count non-integer values)
            }
        }
    }
    
    Ok(column_counts)
}

fn main() {
    match count_csv_column_values("cards.csv") {
        Ok(column_counts) => {
            println!("CSV Column Integer Value Counts:");
            println!("=================================");
            
            let headers = ["Level", "Color", "PV", "Black", "Blue", "Green", "Red", "White"];
            
            for (col_index, counts) in column_counts.iter().enumerate() {
                let header = headers.get(col_index).unwrap_or(&"Unknown");
                println!("\nColumn {}: {} ", col_index, header);
                
                if counts.is_empty() {
                    println!("  (No integer values found)");
                } else {
                    let mut sorted_counts: Vec<_> = counts.iter().collect();
                    sorted_counts.sort_by_key(|(k, _)| k.parse::<i32>().unwrap_or(0));
                    
                    for (value, count) in sorted_counts {
                        println!("  {} appears {} times", value, count);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
        }
    }
}
