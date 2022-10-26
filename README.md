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

To add a config file to an existing risotto.toml (for example ~/.config/example.conf) use:

```bash
risotto add -t "~/.config/example.conf" -l ./example.conf
```

Note the usage of quotes to keep the target path from expading to `/home/{user}/.config/example.conf`. This keeps your `risotto.toml` file user agnostic.

## Config File Format

### **`risotto.toml`**

```toml
[[config]]
source = "./path/to/source/file"
target = "~/path/to/config/file"
link = true  # if true or omitted risotto will symlink the files
```