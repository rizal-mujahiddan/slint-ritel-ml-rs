use polars::prelude::*;
use use_cases::ports::{DataProcessor, ModelRepository};

pub struct PolarsDataProcessor;

impl DataProcessor for PolarsDataProcessor {
    fn process_data(&self, _data: &str) -> Result<String, String> {
        let df = df!(
            "feature1" => &[1, 2, 3],
            "feature2" => &[4, 5, 6],
        )
        .map_err(|e| e.to_string())?;

        let series = df
            .column("feature1")
            .map_err(|e| e.to_string())?
            .as_series()
            .ok_or("Column 'feature1' is not a numeric series")?;

        let sum = series.sum::<i32>().map_err(|e| e.to_string())?;

        Ok(format!("Prediction result: {}", sum))
    }
}

pub struct InMemoryModelRepo {
    models: std::sync::RwLock<std::collections::HashMap<String, Vec<u8>>>,
}

impl InMemoryModelRepo {
    pub fn new() -> Self {
        Self {
            models: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl ModelRepository for InMemoryModelRepo {
    fn save_model(&self, name: &str, model_data: Vec<u8>) -> Result<(), String> {
        let mut map = self.models.write().map_err(|_| "Lock poisoned")?;
        map.insert(name.to_string(), model_data);
        Ok(())
    }

    fn load_model(&self, name: &str) -> Result<Option<Vec<u8>>, String> {
        let map = self.models.read().map_err(|_| "Lock poisoned")?;
        Ok(map.get(name).cloned())
    }
}
