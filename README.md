# risotto

A CLI config file manager. Allowing for a simple git repository to contain all your config files which can be symlinked or copied to their respective location.

## Installation

```bash
git clone https://github.com/HazelTheWitch/risotto.git
cd risotto
cargo install --path .
```

## Usage

To initialize an empty risotto.toml file in your current directory use\:

```bash
risotto init
```

To apply a risotto.toml file use:

```bash
risotto apply
```

## Config File Format

### **`risotto.toml`**

```toml
[[config]]
source = "./path/to/source/file"
target = "~/path/to/config/file"
link = true  # if true or omitted risotto will symlink the files
```