# rocketapp
 
## Rust-Rocket framework template Demo

### Clone as:
```
 git clone https://github.com/srikantgdev/rocketapp [optional-app-name]
```

### SQLite3 Database
```
 northwind.db
```

## Run as:
```
 $ cargo run
 
 - or - 
 in release mode:
 
   $ cargo build --release
   $ ./target/release/rocketapp
```

#### Folder structure
```
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── northwind.db
├── src
│   └── main.rs
└── templates
    ├── Rocket.toml
    ├── base.tera
    ├── categories.tera
    ├── customers.tera
    ├── index.tera
    ├── nav.tera
    └── products.tera
```
