# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fast Search is a desktop file search application built with Tauri 2, combining a Vue.js 3 frontend with a Rust backend. The app provides advanced file search capabilities, AI-powered natural language search, content indexing, and real-time statistics.

## Development Commands

### Frontend Development
```bash
npm run dev              # Start Vite dev server (frontend only)
npm run build            # Build frontend for production
npm run preview          # Preview production build
```

### Full Application Development
```bash
npm run tauri dev        # Start full development mode (frontend + backend)
npm run tauri build      # Build complete application for production
```

### Type Checking & Linting
```bash
npm run build            # Includes TypeScript type checking (vue-tsc --noEmit)
```

### Rust Backend
```bash
cd src-tauri
cargo build              # Build Rust backend
cargo test               # Run Rust tests
cargo clean              # Clean build artifacts
```

## Architecture Overview

### Clean Architecture Pattern
The Rust backend follows clean architecture with clear separation:

- **Entities** (`src-tauri/src/entities/`): Core business models (File, Stat, SearchQuery, AI entities)
- **Ports** (`src-tauri/src/ports/`): Interface definitions (traits for Repository, Reader, AI)
- **Adapters** (`src-tauri/src/adapters/`): External implementations (SQLite, file readers, LM Studio)
- **Services** (`src-tauri/src/services/`): Business logic (FileService, AiService, ContentIndexerService)
- **Utils** (`src-tauri/src/utils/`): Utilities and helpers (file operations, async scanning, indexing)

### Frontend State Management
- **Pinia Stores**: Located in `src/shared/store/` with dedicated stores for:
  - `searchStore.ts`: Search functionality and results
  - `settingStore.ts`: Application settings and configurations
  - `aiStore.ts`: AI search state and model management

### Database Layer
- **SQLite** with Rusqlite for data persistence
- **Initialization**: Schema defined in `src-tauri/data/init.sql`
- **Repository Pattern**: Implemented in `src-tauri/src/adapters/repository/sqlite.rs`

### AI Integration
- **LM Studio Integration**: Local AI model support via HTTP API
- **Natural Language Queries**: Converts natural language to search filters
- **Model Management**: Dynamic model loading and health checking
- **Prompt Engineering**: Custom prompts in `src-tauri/data/prompt.txt`

## Key Tauri Commands

### File Operations
- `get_stat()` - Retrieve application statistics
- `sync_files_and_folders(window)` - Scan and index directories
- `search_files(query: SearchQuery)` - Execute file searches
- `open_file(path: String)` - Open files in system explorer
- `reset_data()` - Clear database and start fresh

### Path Management
- `get_all_paths()` - Get configured search paths
- `save_paths(paths: Vec<String>, window)` - Set search directories
- `get_all_folders()` - Get indexed folder list
- `get_all_types()` - Get available file types

### Content Indexing
- `start_content_indexing()` - Begin content indexing process
- `get_uncontent_indexed_files()` - Get files pending content indexing

### AI Features
- `ai_search(natural_query: String, model: String, ai_url: String)` - Natural language search
- `ai_health_check(ai_url: String)` - Check AI service availability
- `ai_list_models(ai_url: String)` - Get available AI models

## Development Patterns

### Frontend Components
- **Composition API**: All Vue components use `<script setup>` syntax
- **TypeScript**: Strict typing enabled throughout frontend
- **Component Structure**: 
  - `components/base/` - Reusable base components
  - `components/card/` - Display cards (File, Folder)
  - `components/sync/` - Synchronization UI components
- **Styling**: Tailwind CSS with Naive UI components

### Rust Error Handling
- Use `Result<T, String>` pattern for Tauri commands
- Implement `thiserror` for custom error types
- Async operations use Tokio runtime

### State Synchronization
- Frontend and backend communicate via Tauri events
- Real-time progress updates during file scanning
- WebviewWindow events for UI notifications

### File Reading & Indexing
Multiple specialized readers handle different file types:
- **TextReader**: TXT, MD, JSON, logs, config files
- **PdfReader**: PDF documents with `lopdf`
- **CsvReader**: CSV files with structured data
- **CodeReader**: Source code files (JS, TS, Python, Rust, etc.)
- **WordReader**: DOC/DOCX files (planned)

### Performance Optimizations
- **Parallel Processing**: Rayon for multi-threaded operations
- **Async Scanning**: Non-blocking directory traversal
- **Pagination**: Frontend handles large result sets
- **SQLite Indexing**: Optimized queries with proper indices

## Configuration

### AI Service Configuration
AI service URLs and models are configurable at runtime. The default LM Studio setup expects:
- Local server at `http://localhost:1234`
- Compatible models (Llama, Mistral, etc.)
- Chat completions API endpoint

### File Type Support
Content indexing supports various file types with size limits:
- Text files: up to 50KB
- PDF files: up to 10MB
- Code files: up to 1000 lines
- CSV files: up to 5000 lines

### Database Schema
The SQLite database includes tables for:
- Files and directories metadata
- Content indexing data
- Search paths configuration
- File type classifications

## Testing Strategy

### Rust Backend Tests
Tests are located in `src-tauri/tests/` with common utilities. Run with:
```bash
cd src-tauri && cargo test
```

### Frontend Testing
Currently uses Vue 3 testing utilities (setup in progress).

## Build Configuration

### Tauri Configuration
- **Development**: Hot reload on port 1420
- **Production**: Optimized bundle with all targets
- **Icons**: Multi-resolution app icons in `src-tauri/icons/`
- **Permissions**: File system access, dialog, and opener plugins

### TypeScript Configuration
- Strict mode enabled with comprehensive linting rules
- Vue SFC support with proper type checking
- ES2020 target with ESNext modules