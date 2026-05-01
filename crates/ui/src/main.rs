#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use infra::{InMemoryModelRepo, PolarsDataProcessor};
// use use_cases::ports::DataProcessor;
use use_cases::use_cases::process_data_with_pipeline;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processor = PolarsDataProcessor;
    let _model_repo = InMemoryModelRepo::new(); // prefixed with _ to silence warning

    let pipeline =
        domain::models::Pipeline::new("pipeline-1".to_string(), "My First ETL".to_string());
    match process_data_with_pipeline(&pipeline, &processor) {
        Ok(result) => println!("Pipeline result: {}", result),
        Err(e) => eprintln!("Pipeline error: {}", e),
    }

    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();

    Ok(())
}
