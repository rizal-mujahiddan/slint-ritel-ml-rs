use crate::ports::DataProcessor;
use domain::models::Step;

pub fn transform_data(input: &str, step: &Step) -> Result<(), String> {
    println!("Applying transform step '{}' on '{}'", step.id, input);
    Ok(())
}

pub fn predict<P: DataProcessor>(processor: &P, input: &str, step: &Step) -> Result<(), String> {
    println!("Predicting with model '{}'", step.id);
    let result = processor.process_data(input)?;
    println!("Prediction result: {}", result);
    Ok(())
}
