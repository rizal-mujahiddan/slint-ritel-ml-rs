# Ritel Workflow Automation (Rust + Slint)

A powerful, offline-first ETL and AI Workflow tool.

## 🚀 Key Features
- **Visual ETL**: Drag-and-drop nodes for CSV, XLSX, JSON, and SQLite.
- **AI/ML Nodes**: Integrated K-Means, Logistic Regression, and Deep Learning (Candle).
- **Computer Vision**: Image analysis nodes with ResNet support.
- **Offline First**: Runs entirely on your machine, no cloud API required.
- **Clean Architecture**: Built with Rust for safety and performance.
- **Theme Support**: Dark and Light modes.

## 🛠️ Tech Stack
- **UI**: [Slint](https://slint.dev/)
- **Data Engine**: [Polars](https://pola.rs/)
- **ML**: [Linfa](https://github.com/rust-ml/linfa)
- **Deep Learning**: [Candle](https://github.com/huggingface/candle)
- **Database**: [SQLite](https://www.sqlite.org/)

## 📂 Project Structure
```text
.
├── Cargo.toml          # Workspace configuration
├── crates/
│   ├── ui/             # Slint UI & App logic
│   ├── domain/         # Core entities (Nodes, Connections)
│   ├── use_cases/      # Workflow Engine & Node Logic
│   ├── repositories/   # Data Access Traits
│   └── infra/          # SQLite & File implementations
├── BusinessREADME.md   # Business analysis & Architecture diagrams
└── implementation_plan.md # Detailed roadmap and status
```

## 🏗️ Getting Started

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --package ui
```

## 🎨 Workflow Editor
The editor supports dragging nodes from the palette on the left. You can save your workflows as `.json` files for sharing or load them back later.
