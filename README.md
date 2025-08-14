# 🔍 Fast Search

**An advanced file search application with artificial intelligence**

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vue.js](https://img.shields.io/badge/Vue.js-35495E?style=flat&logo=vue.js&logoColor=4FC08D)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-24C8DB?style=flat&logo=tauri&logoColor=white)](https://tauri.app/)
[![Code of Conduct](https://img.shields.io/badge/Code%20of%20Conduct-Contributor%20Covenant-4baaaa.svg)](CODE_OF_CONDUCT.md)

Fast Search is a modern desktop application built with **Tauri 2**, **Vue.js 3**, and **Rust**, offering ultra-fast file search with artificial intelligence capabilities for natural language queries.

## 📸 Preview
## ✨ Key Features

### 🔍 **Advanced Search**
- **Instant search** across files and folders
- **Smart filters** by type, size, and date
- **Content search** within files (PDF, TXT, CSV, Source code)
- **Intelligent pagination** of results
- **CSV export** of search results

### 🤖 **Artificial Intelligence**
- **Natural language search** with LM Studio
- **Automatic filter generation** from your descriptions
- **Local AI model support** (Llama, Mistral, etc.)
- **Conversational interface** that's intuitive
- **Privacy guaranteed** (100% local processing)

### 📊 **Management & Analytics**
- **Real-time statistics** of indexed files
- **Automatic background indexing**
- **Advanced performance diagnostics**
- **Smart folder synchronization**

### 🎨 **Modern Interface**
- **Responsive design** with Naive UI and Tailwind CSS
- **Dark/light themes** with automatic switching
- **File preview** (images, documents)
- **Intuitive navigation** with keyboard shortcuts
- **Quick path copying** to clipboard

### ⚡ **Optimized Performance**
- **Rust backend** for maximum speed
- **SQLite database** for persistence
- **Parallel file processing** with Rayon
- **Asynchronous operations** with Tokio

## 🚀 Quick Start

### Prerequisites
- **Node.js** (version 18 or higher)
- **Rust** (latest stable version)
- **Tauri CLI**: `npm install -g @tauri-apps/cli`

### Installation
```bash
# Clone the repository
git clone https://github.com/math-dev-24/fast-search.git
cd fast-search

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build the application
npm run tauri build
```

## 📖 User Guide

### First Use
1. **Synchronization**: Click "Synchronize" and select your folders
2. **Indexing**: The app automatically scans your files
3. **Search**: Use the search bar to instantly find your files

### AI Search
1. Switch to "AI Search" mode
2. Describe your search in natural language
3. Select an available AI model
4. Let AI generate optimal filters

### Content Search
- Enable content indexing in settings
- Search within text of PDFs, documents, and code files
- Export your results to CSV format

## 🛠️ Technology Stack

### Frontend (Vue.js 3 + TypeScript)
- **Vue.js 3** - Progressive framework with Composition API
- **TypeScript** - Static typing for robustness
- **Naive UI** - Modern and elegant UI components
- **Tailwind CSS** - Utility-first CSS framework
- **Pinia** - Reactive state management
- **Vue Router** - Client-side routing
- **Vite** - Ultra-fast build tool
- **VueUse** - Collection of Vue.js utilities
- **Vicons** - Modern and consistent icons
- **Luxon** - Advanced date handling

### Backend (Rust + Tauri)
- **Rust** - Ultra-performant systems language
- **Tauri 2** - Framework for native desktop apps
- **SQLite** - Lightweight and reliable database
- **Rusqlite** - Optimized SQLite driver for Rust
- **Walkdir** - Efficient directory traversal
- **Chrono** - Date and time manipulation
- **Serde** - JSON serialization/deserialization
- **Rayon** - CPU-intensive task parallelization
- **Tokio** - High-performance async runtime
- **Reqwest** - HTTP client for external APIs
- **Lopdf** - PDF content extraction

### Artificial Intelligence
- **LM Studio** - Integration with local models
- **Multi-model support** - Llama, Mistral, CodeLlama, etc.
- **OpenAI-compatible API** - Industry standard
- **Local processing** - Privacy guaranteed

## 🏗️ Technical Architecture

### Frontend Structure
```
src/
├── components/          # Reusable components
│   ├── base/           # Base components (Header, etc.)
│   ├── card/           # Display cards (File, Folder)
│   ├── sync/           # Synchronization interface
│   └── ...             # Other specialized components
├── views/              # Main application pages
├── composables/        # Reusable business logic
├── stores/             # Pinia state management
├── types/              # TypeScript definitions
└── utils/              # Utility functions
```

### Backend Structure (Clean Architecture)
```
src-tauri/src/
├── entities/           # Business data models
├── services/           # Business logic and use cases
├── adapters/           # Concrete implementations
│   ├── repository/     # Data access (SQLite)
│   ├── reader/         # Specialized file readers
│   └── ai/             # AI integration (LM Studio)
├── ports/              # Interfaces and contracts
└── utils/              # Utilities and helpers
```

### Database
- **SQLite** with performance-optimized schema
- **Smart indexes** on search fields
- **Transactions** for data consistency
- **Automatic schema migrations**

## 🤝 Contributing

We warmly welcome all contributions!

### 🌟 Types of Contributions
- 🐛 **Bug Reports**: Report issues you encounter
- 💡 **Feature Requests**: Propose new functionality
- 📝 **Documentation**: Improve documentation
- 🔧 **Code**: Implement features or bug fixes
- 🌍 **Translations**: Help internationalize the app
- 🎨 **Design**: Improve user interface
- 🧪 **Testing**: Add tests to improve quality

### 📋 Contribution Process
1. **Read our [Contributing Guide](CONTRIBUTING.md)**
2. **Follow our [Code of Conduct](CODE_OF_CONDUCT.md)**
3. **Fork** the project
4. **Create a branch**: `git checkout -b feature/my-new-feature`
5. **Commit**: `git commit -m 'feat: add my new feature'`
6. **Push**: `git push origin feature/my-new-feature`
7. **Open a Pull Request** with detailed description

### 🏆 Recognition
All contributors are automatically added to the CONTRIBUTORS.md file and appear on the project's GitHub page.

## 📋 Roadmap

### ✅ **Completed Features**
- [x] Advanced search with smart filters
- [x] Modern responsive interface
- [x] Dark/light themes
- [x] AI search with LM Studio
- [x] Content indexing (PDF, TXT, CSV, Code)
- [x] CSV export of results
- [x] Real-time statistics
- [x] File preview
- [x] Performance diagnostics

### 🔄 **In Development**
- [ ] **Search history** with saved configurations
- [ ] **Automatic detection** of new files
- [ ] **Advanced preview** (PDF, Word, more formats)
- [ ] **Customizable keyboard shortcuts**

### 🎯 **Next Priorities**
- [ ] **Comprehensive automated tests** (unit + integration)
- [ ] **Multi-language support** (internationalization)
- [ ] **REST API** for external integrations
- [ ] **Plugin and extension** system
- [ ] **Optional cloud sync**
- [ ] **Real-time indexing** with file watchers
- [ ] **Machine Learning** for smart suggestions

### 🚀 **Long-term Vision**
- [ ] **Mobile app** (iOS/Android) with sync
- [ ] **Web interface** for remote access
- [ ] **Collaboration** and config sharing
- [ ] **Advanced local AI** with specialized models

## 🐛 Troubleshooting

### Common Issues

<details>
<summary><strong>🔍 Search Problems</strong></summary>

- **No results**: Check that folders are synchronized
- **Incomplete results**: Restart indexing from settings
- **Slow search**: Reduce number of indexed folders
</details>

<details>
<summary><strong>🤖 AI Problems</strong></summary>

- **Model unavailable**: Check that LM Studio is running
- **Connection error**: Verify URL in settings
- **Inconsistent responses**: Try a different model
</details>

<details>
<summary><strong>⚙️ Technical Problems</strong></summary>

- **App won't start**: Check prerequisites (Node.js, Rust)
- **Corrupted database**: Delete `db.sqlite` and restart
- **Compilation errors**: `cargo clean && cargo build`
</details>

### 💬 Getting Help
- 📖 Check our [Wiki](https://github.com/math-dev-24/fast-search/wiki)
- 🐛 Open an [Issue](https://github.com/math-dev-24/fast-search/issues)
- 💬 Join [Discussions](https://github.com/math-dev-24/fast-search/discussions)
- 📧 Contact the team via [maintainers](mailto:your-email@example.com)

## 📊 Performance & Statistics

### 🚀 **Benchmarks**
- **Indexing**: 100k+ files in < 30 seconds
- **Search**: Results in < 100ms for most queries
- **Memory**: Optimized usage, < 200MB in normal operation
- **Startup**: Fast boot in < 2 seconds

### 🏅 **Compatibility**
- **Windows**: Windows 10+ (x64, ARM64)
- **macOS**: macOS 10.15+ (Intel, Apple Silicon)
- **Linux**: Ubuntu 18.04+, Fedora, Arch, etc.

### 📈 **Recommended Limits**
- **Indexed files**: Up to 1M files
- **File sizes**: PDF < 50MB, others < 10MB
- **Simultaneous folders**: Up to 100 root directories

## 🙏 Acknowledgments

### 🎯 **Core Team**
- **[@math-dev-24](https://github.com/math-dev-24)** - Creator and lead maintainer

### 🌟 **Contributors**
Thanks to all contributors who help improve Fast Search! Your name will automatically appear here after your first contribution.

<!-- Contributors will be automatically added here -->

### 🛠️ **Open Source Technologies**
Big thanks to the projects that make Fast Search possible:
- [Tauri](https://tauri.app/) - Modern and secure desktop framework
- [Vue.js](https://vuejs.org/) - Progressive JavaScript framework
- [Rust](https://www.rust-lang.org/) - Safe and performant systems language
- [LM Studio](https://lmstudio.ai/) - Local AI platform
- [Naive UI](https://www.naiveui.com/) - Elegant component library
- [SQLite](https://www.sqlite.org/) - Reliable embedded database

### 🎨 **Design & Inspiration**
- Design inspired by modern UX/UI best practices
- Icons from [Iconify](https://iconify.design/)
- Color palette optimized for accessibility

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### 🔓 **Why MIT?**
- ✅ **Complete freedom**: Use, modify, distribute as you want
- ✅ **Commercial use allowed**: Integrate into commercial projects
- ✅ **No copyleft**: No obligation to share your modifications
- ✅ **Simple and clear**: Short and easy to understand license
- ✅ **Enterprise compatible**: Widely accepted in professional environments

## 🌟 Support the Project

### ⭐ **GitHub Star**
If Fast Search helps you, give us a ⭐ on GitHub! It motivates us to continue.

### 📢 **Share**
- Tell your colleagues and friends about Fast Search
- Write a blog post about your experience
- Tweet with #FastSearchApp

### 💰 **Sponsoring** (optional)
If you'd like to financially support development:
- [GitHub Sponsors](https://github.com/sponsors/math-dev-24) (to be configured)
- [Buy me a coffee](https://www.buymeacoffee.com/mathdev24) (to be configured)

---

## 📞 Contact & Support

### 🔗 **Useful Links**
- **Repository**: [github.com/math-dev-24/fast-search](https://github.com/math-dev-24/fast-search)
- **Documentation**: [Project Wiki](https://github.com/math-dev-24/fast-search/wiki)
- **Releases**: [Downloads](https://github.com/math-dev-24/fast-search/releases)

### 💌 **Stay Updated**
- **Watch** the repository for notifications
- **Releases** for new versions only
- **Discussions** to participate in conversations

---

<div align="center">

**Made with ❤️ by the Fast Search community**

*Fast Search - Find everything, instantly.*

[![GitHub stars](https://img.shields.io/github/stars/math-dev-24/fast-search?style=social)](https://github.com/math-dev-24/fast-search/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/math-dev-24/fast-search?style=social)](https://github.com/math-dev-24/fast-search/network/members)
[![GitHub watchers](https://img.shields.io/github/watchers/math-dev-24/fast-search?style=social)](https://github.com/math-dev-24/fast-search/watchers)

</div>