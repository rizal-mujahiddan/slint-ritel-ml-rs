# Implementation Plan: Workflow Automation Tool (Rust + Slint)

## Project Overview
Create a simple, offline, drag-and-drop workflow tool similar to KNIME/Pentaho.

## Core Features
- [x] **Canvas Editor**: Visual interface for creating workflows with connection lines.
- [x] **Node System**: Modular blocks for data input (CSV, XLSX, JSON), processing, and ML.
- [x] **Workflow Engine**: Robust Polars-based engine to execute nodes in DAG order.
- [x] **Local Storage**: SQLite and JSON-based persistence for workflows.
- [x] **Clean Architecture**: Domain-driven design with clear separation of concerns.

## Technical Stack
- **Language**: Rust
- **UI Framework**: Slint (Modern theme with Dark/Light modes)
- **Data Processing**: Polars (Vectorized performance)
- **AI/ML**: Linfa (Classical ML) & Candle (Deep Learning)
- **Database**: SQLite
- **Async Runtime**: Tokio

## Architecture (Clean Architecture)
1. **Domain**: Definitions of `Node`, `Connection`, `Workflow`, `DataPayload`.
2. **Use Cases**: `ExecuteWorkflow`, `ValidateWorkflow`, `Save/LoadWorkflow`.
3. **Interfaces (Repositories)**: `WorkflowRepository`.
4. **Infrastructure**: `SQLiteRepository`, `FileStore`.
5. **Presentation (UI)**: Slint Window, Node components, Dynamic Graph logic.

## Roadmap Status
1. **Phase 1: Foundation (Structure & Domain)** ✅
   - Initialize crates and workspace.
   - Define core entities in `domain`.
2. **Phase 2: UI Prototype** ✅
   - Create the Slint main window.
   - Implement draggable "Node" component with icons.
   - Implement dynamic connection lines.
3. **Phase 3: Execution Engine** ✅
   - Implement node logic (CSV reader, Filter, JSON, XLSX).
   - Integrate Machine Learning libraries (Linfa).
   - Implement robust flow controller with validation.
4. **Phase 4: Integration & Persistence** ✅
   - Connect UI actions to Use Cases.
   - Implement workflow saving/loading (JSON files).
5. **Phase 5: Documentation & Polish** ✅
   - Finalize README.md and BusinessREADME.md.
   - Zero-warning, zero-error compilation.

## Technical Stack Optimization
- **Size**: Optimized via `lto`, `codegen-units = 1`, and `panic = "abort"`.
- **Performance**: High-speed processing using **Polars** and **Linfa**.
- **Efficiency**: Modular clean architecture reduces runtime overhead.
