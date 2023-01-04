<h1>Open Browser</h1>
Open the site from the terminal with the registered name.

## Install

Add sqlx commands.
```bash
cargo install sqlx-cli
```

Create database.
```bash
sqlx database create
```

Execute migration file.
```bash
sqlx migrate run
```

Code is built.
```bash
cargo build
```

## Usage
Display list of registered sites.
```bash
cargo run -- list
```

Register your site.
```bash
cargo run -- add <name> <url>

ex)
cargo run -- add github https://github.com/
```

Delete a registered site.
```bash
cargo run -- delete <name>

ex)
cargo run -- delete github
```

Open one of the registered sites.
```bash
cargo run -- brow <name>

ex)
cargo run -- brow github
```

Open multiple registered sites.
```bash
cargo run -- brow <name1> <name2>

ex)
cargo run -- brow github forkwell
```
