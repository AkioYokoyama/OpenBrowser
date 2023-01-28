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
cargo build --release
```

## Usage
Display list of registered sites.
```bash
./target/release/open_browser list
```

Register your site.
```bash
./target/release/open_browser add <name> <url>

ex)
cargo run -- add github https://github.com/
```

Delete a registered site.
```bash
./target/release/open_browser delete <name>

ex)
./target/release/open_browser delete github
```

Open one of the registered sites.
```bash
./target/release/open_browser <name>

ex)
./target/release/open_browser github
```

Open multiple registered sites.
```bash
./target/release/open_browser <name1> <name2>

ex)
./target/release/open_browser github forkwell
```

Display the number of times used
```bash
./target/release/open_browser freq
```
