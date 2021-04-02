## Note

Although not concretely used, it looks like that without this `src` folder
running `cargo run -p tutor-nodb --bin basic-server` throws this error:

```

error: failed to parse manifest at `.../rust_servers_services_apps/ezytutors/Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present

```
