mod step_logic; // declares the module from step_logic.rs

// No `use crate::step_logic;` here!

pub mod ports {
    pub trait DataProcessor: Send + Sync {
        fn process_data(&self, data: &str) -> Result<String, String>;
    }
    // ✅ Now properly exported
    pub trait ModelRepository: Send + Sync {
        fn save_model(&self, name: &str, model_data: Vec<u8>) -> Result<(), String>;
        fn load_model(&self, name: &str) -> Result<Option<Vec<u8>>, String>;
    }
}

pub mod use_cases {
    use super::ports::DataProcessor;
    use super::step_logic;
    use domain::models::{Pipeline, StepType};

    pub fn process_data_with_pipeline<P: DataProcessor>(
        pipeline: &Pipeline,
        processor: &P,
    ) -> Result<String, String> {
        let input = "Sample input data".to_string();
        for step in &pipeline.steps {
            match step.step_type {
                StepType::Transform => {
                    step_logic::transform_data(&input, step)?;
                }
                StepType::ModelPredict => {
                    step_logic::predict(processor, &input, step)?;
                }
                _ => {}
            }
        }
        Ok("Processing complete".to_string())
    }
}
