# Time Test

```rust
for _ in 0..1000 {
    git2::Repository::open_bare(".git").unwrap();
}

for _ in 0..1000 {
    rusqlite::Connection::open("1.db").unwrap();
}
```

Open git repository takes more time than sqlite database

```bash
Open git        : 117.98568ms
Open sqlite     : 75.901934ms
```

