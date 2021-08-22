## Note

Although not concretely used, it looks like that without this `src` folder
running `cargo run -p tutor-... --bin ...` throws this error:

```

error: failed to parse manifest at `.../ezytutors_v2/Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present

```
