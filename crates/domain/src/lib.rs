use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub node_type: NodeType,
    pub x: f32,
    pub y: f32,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    CsvInput,
    XlsxInput,
    SqliteInput,
    JsonInput,
    Filter,
    Select,
    Join,
    GroupBy,
    KMeans,
    LogisticRegression,
    ImageAnalyze,
    NLPTextAnalysis,
    DeepLearningInference,
    CsvOutput,
    SqliteOutput,
    TableDisplay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: Uuid,
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub from_port: String,
    pub to_port: String,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }
}
