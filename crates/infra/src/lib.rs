use domain::Workflow;
use rusqlite::{params, Connection};
use anyhow::Result;
use uuid::Uuid;

use repositories::WorkflowRepository;

pub struct SqliteRepository {
    conn: Connection,
}

impl WorkflowRepository for SqliteRepository {
    fn save(&self, workflow: &Workflow) -> Result<()> {
        self.save_workflow(workflow)
    }

    fn load(&self, id: Uuid) -> Result<Workflow> {
        self.load_workflow(id)
    }

    fn list(&self) -> Result<Vec<(Uuid, String)>> {
        self.list_workflows()
    }
}

impl SqliteRepository {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn save_workflow(&self, workflow: &Workflow) -> Result<()> {
        let data = serde_json::to_string(workflow)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO workflows (id, name, data) VALUES (?1, ?2, ?3)",
            params![workflow.id.to_string(), workflow.name, data],
        )?;
        Ok(())
    }

    pub fn load_workflow(&self, id: Uuid) -> Result<Workflow> {
        let mut stmt = self.conn.prepare("SELECT data FROM workflows WHERE id = ?1")?;
        let data: String = stmt.query_row(params![id.to_string()], |row| row.get(0))?;
        let workflow: Workflow = serde_json::from_str(&data)?;
        Ok(workflow)
    }

    pub fn list_workflows(&self) -> Result<Vec<(Uuid, String)>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM workflows")?;
        let rows = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((Uuid::parse_str(&id_str).unwrap_or_default(), name))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
