# 🏗️ SEL Project Structure Guide

## 📁 Directory Layout
~/sel-production/SEL/ # WORKSPACE ROOT
├── Cargo.toml # Workspace configuration
├── README.md # Project overview
├── DOCUMENTATION_DAY_2.md # Day 2 technical docs
├── DEPENDENCIES_DOCUMENTATION.md # Dependencies guide
├── PROJECT_STRUCTURE.md # This file
│
├── crates/ # All Rust crates
│ ├── sel-core/ # Sovereign DNA (library)
│ │ ├── Cargo.toml # name = "sel-core", version = "0.1.0"
│ │ ├── src/
│ │ │ ├── lib.rs # Main library file
│ │ │ ├── canonical.rs # 10-step canonicalization
│ │ │ ├── hash_chain.rs # Tamper-proof logging
│ │ │ └── env.rs # Environment normalization
│ │ └── target/ # Build outputs (gitignored)
│ │
│ ├── sel-engine/ # Execution Engine (binary)
│ │ ├── Cargo.toml # name = "sel-engine", version = "0.2.0"
│ │ ├── src/
│ │ │ ├── main.rs # CLI entry point
│ │ │ ├── lib.rs # Library interface
│ │ │ ├── canonical_adapter.rs # Integration with sel-core
│ │ │ └── engine/ # Engine module (Day 3)
│ │ └── target/ # Build outputs (gitignored)
│ │
│ └── sel-validator/ # Validator (future)
│
└── target/ # Workspace build outputs (gitignored)

text

## 🚀 How to Navigate

### From anywhere to workspace root:
```bash
cd ~/sel-production/SEL
From workspace to specific crate:
bash
cd crates/sel-core      # For sel-core development
cd crates/sel-engine    # For sel-engine development
Building options:
bash
# Build entire workspace (recommended)
cd ~/sel-production/SEL
cargo build --release

# Build specific crate
cd ~/sel-production/SEL/crates/sel-core
cargo build --release

# Run tests for all crates
cd ~/sel-production/SEL
cargo test

# Run tests for specific crate
cd ~/sel-production/SEL/crates/sel-engine
cargo test
🔗 Dependency Relationships
sel-engine depends on sel-core:
toml
# In crates/sel-engine/Cargo.toml:
[dependencies]
sel-core = { path = "../sel-core", version = "0.1.0" }
Usage in code:
rust
// In sel-engine source code:
use sel_core::canonicalize_json;
use sel_core::HashChain;
🛠️ Common Commands Reference
Command	Location	Purpose
cargo build	Workspace root	Build all crates
cargo test	Workspace root	Test all crates
cargo check	Any crate	Type check without building
cargo run -- --help	sel-engine/	Show CLI help
cargo doc --open	Workspace root	Open documentation
⚠️ Common Issues & Solutions
Issue: "could not find Cargo.toml"
Solution: Make sure you're in the right directory:

bash
# Wrong:
cd ~/sel-production/SEL/crates/sel-core
cargo build --release  # Works (crate-level)
cargo build --release  # Fails (trying to build workspace)

# Right:
cd ~/sel-production/SEL
cargo build --release  # Builds entire workspace
Issue: "unused import" warnings
Solution: Either fix the import or add #[allow(unused_imports)]

Issue: "dependency not found"
Solution: Check path in Cargo.toml is correct:

toml
# Correct:
sel-core = { path = "../sel-core" }

# Wrong:
sel-core = { path = "../../sel-core" }
📍 Current Location Check
To check where you are:

bash
# Show current directory
pwd

# Show project structure from here
ls -la

# Check if Cargo.toml exists
[ -f "Cargo.toml" ] && echo "Workspace root" || echo "Not workspace root"
🎯 Golden Rule
Always start from workspace root for workspace commands:

bash
cd ~/sel-production/SEL
# Then run cargo commands
For crate-specific work, go to crate directory:

bash
cd ~/sel-production/SEL/crates/sel-engine
# Then edit files, run crate-specific tests
*تم إنشاء هذا الملف بواسطة SEL Team - Day 2*
