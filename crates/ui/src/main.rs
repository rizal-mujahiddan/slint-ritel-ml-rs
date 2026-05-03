#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use domain::{Workflow, Node, NodeType};
// use use_cases::WorkflowExecutor;
use infra::SqliteRepository;
use slint::{Model, VecModel, SharedString, ComponentHandle, ModelRc};
use std::rc::Rc;
use uuid::Uuid;
use std::collections::HashMap;
// use native_dialog::{FileDialog, MessageDialog, MessageType};

slint::include_modules!();

fn update_connection_paths(nodes: &Rc<VecModel<NodeData>>, conns: &Rc<VecModel<ConnectionData>>) {
    for i in 0..conns.row_count() {
        let mut conn = conns.row_data(i).unwrap();
        let mut from_pos = (0.0, 0.0);
        let mut to_pos = (0.0, 0.0);
        
        for j in 0..nodes.row_count() {
            let n = nodes.row_data(j).unwrap();
            if n.id == conn.from_id { from_pos = (n.x + 80.0, n.y + 40.0); }
            if n.id == conn.to_id { to_pos = (n.x + 80.0, n.y + 40.0); }
        }
        
        conn.x1 = from_pos.0;
        conn.y1 = from_pos.1;
        conn.x2 = to_pos.0;
        conn.y2 = to_pos.1;
        conn.commands = SharedString::from(format!("M {} {} L {} {}", from_pos.0, from_pos.1, to_pos.0, to_pos.1));
        conns.set_row_data(i, conn);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let _repo = Rc::new(SqliteRepository::new("workflow.db")?);

    let nodes_model = Rc::new(VecModel::from(vec![
        NodeData { 
            id: SharedString::from("1"), 
            name: SharedString::from("Input Data"), 
            node_type: SharedString::from("CsvInput"), 
            x: 100.0, y: 100.0, selected: false, icon: SharedString::from("📄")
        },
        NodeData { 
            id: SharedString::from("2"), 
            name: SharedString::from("Filter Data"), 
            node_type: SharedString::from("Filter"), 
            x: 300.0, y: 200.0, selected: false, icon: SharedString::from("🔍")
        }
    ]));
    
    let conns_model = Rc::new(VecModel::from(vec![
        ConnectionData { from_id: "1".into(), to_id: "2".into(), x1: 0.0, y1: 0.0, x2: 0.0, y2: 0.0, commands: "".into() }
    ]));

    update_connection_paths(&nodes_model, &conns_model);
    
    ui.set_nodes(ModelRc::from(nodes_model.clone()));
    ui.set_connections(ModelRc::from(conns_model.clone()));

    let nodes_model_clone = nodes_model.clone();
    ui.on_add_node(move |node_type| {
        let id = Uuid::new_v4();
        nodes_model_clone.push(NodeData {
            id: SharedString::from(id.to_string()),
            name: SharedString::from(format!("{} Node", node_type)),
            node_type: SharedString::from(node_type.clone()),
            x: 50.0, y: 50.0, selected: false, icon: SharedString::from("⚙️"),
        });
    });

    let nodes_model_clone = nodes_model.clone();
    let conns_model_clone = conns_model.clone();
    ui.on_node_moved(move |id, x, y| {
        for i in 0..nodes_model_clone.row_count() {
            let mut node = nodes_model_clone.row_data(i).unwrap();
            if node.id == id {
                node.x = x;
                node.y = y;
                nodes_model_clone.set_row_data(i, node);
                break;
            }
        }
        update_connection_paths(&nodes_model_clone, &conns_model_clone);
    });

    let ui_weak = ui.as_weak();
    ui.on_node_selected(move |id| {
        let ui = ui_weak.unwrap();
        ui.set_selected_node_id(id.clone());
        let nodes = ui.get_nodes();
        for i in 0..nodes.row_count() {
            let mut node = nodes.row_data(i).unwrap();
            node.selected = node.id == id;
            nodes.set_row_data(i, node);
        }
    });

    let nodes_model_clone = nodes_model.clone();
    let conns_model_clone = conns_model.clone();
    let ui_weak = ui.as_weak();
    ui.on_delete_node(move |id| {
        // Remove node
        for i in 0..nodes_model_clone.row_count() {
            if nodes_model_clone.row_data(i).unwrap().id == id {
                nodes_model_clone.remove(i);
                break;
            }
        }
        // Remove associated connections
        let mut i = 0;
        while i < conns_model_clone.row_count() {
            let conn = conns_model_clone.row_data(i).unwrap();
            if conn.from_id == id || conn.to_id == id {
                conns_model_clone.remove(i);
            } else {
                i += 1;
            }
        }
        ui_weak.unwrap().set_selected_node_id("".into());
    });

    let nodes_model_clone = nodes_model.clone();
    let ui_handle = ui.as_weak();
    ui.on_run_workflow(move || {
        let ui = ui_handle.upgrade().unwrap();
        ui.set_logs(SharedString::from("Running workflow..."));
        let mut wf = Workflow::new("Execution Workflow".to_string());
        for i in 0..nodes_model_clone.row_count() {
            let n = nodes_model_clone.row_data(i).unwrap();
            wf.add_node(Node {
                id: Uuid::parse_str(&n.id).unwrap_or(Uuid::nil()),
                name: n.name.to_string(),
                node_type: NodeType::TableDisplay,
                x: n.x, y: n.y, properties: HashMap::new(),
            });
        }
        // ... execution logic ...
    });

    ui.run()?;
    Ok(())
}
