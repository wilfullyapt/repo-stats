# Repo Stats
*Designed to quickly gather the stats of a Repo*

#### Currently Supported Languages
- Python
- Rust
- C
- C++

# Setup
1. Clone the repo
2. Compile with `cargo build --release`
3. `./target/release/repostats install` to install to `~/.local/bin`

# Usage
- **`analyze <target directory> <filetype>`**
  - Analyzes code statistics in the specified directory.
  - `<target directory>`: Path to the directory (absolute or relative to the current working directory).
  - `<filetype>`: One of `python`, `rust`, `c`, `cpp`.
  - Example: `repostats analyze code-dir python`

- **`install`**
  - Installs `repostats` to `~/.local/bin`, overwriting if it exists.
  - Example: `repostats install`

- **`uninstall`**
  - Removes `repostats` from `~/.local/bin`.
  - Example: `repostats uninstall`
