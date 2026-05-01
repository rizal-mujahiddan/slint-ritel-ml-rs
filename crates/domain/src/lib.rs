pub mod models {
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pipeline {
        pub id: String,
        pub name: String,
        pub steps: Vec<Step>,
        pub config: HashMap<String, String>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Step {
        pub id: String,
        pub step_type: StepType,
        pub parameters: HashMap<String, String>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum StepType {
        Extract,
        Transform,
        Load,
        ModelTrain,
        ModelPredict,
    }

    impl Pipeline {
        pub fn new(id: String, name: String) -> Self {
            Self {
                id,
                name,
                steps: Vec::new(),
                config: HashMap::new(),
            }
        }

        pub fn add_step(&mut self, step: Step) {
            self.steps.push(step);
        }
    }
}
