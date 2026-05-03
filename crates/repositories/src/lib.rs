use domain::Workflow;
use uuid::Uuid;
use anyhow::Result;

pub trait WorkflowRepository {
    fn save(&self, workflow: &Workflow) -> Result<()>;
    fn load(&self, id: Uuid) -> Result<Workflow>;
    fn list(&self) -> Result<Vec<(Uuid, String)>>;
}
