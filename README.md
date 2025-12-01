# yapyrg

Python bindings for [ripgrep](https://github.com/BurntSushi/ripgrep)'s underlying crates (\`grep-searcher\` and \`ignore\`).

It provides a simple interface to search files recursively while respecting \`.gitignore\` rules.

![CI](https://github.com/brunoportis/yapyrg/actions/workflows/CI.yml/badge.svg)

## Installation

```bash
uv add yapyrg
```

## Usage

```python
import yapyrg

# Search recursively for "pattern" in the current directory
matches = yapyrg.search(".", r"pattern")

# Returns a list of dicts
for m in matches:
    print(f"{m['path']}:{m['line']} - {m['content']}")
```

## Development

```bash
uv run maturin develop
```
