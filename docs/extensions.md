# Extensions

Extensions are used to provide additional information without affecting the data. `$` is the prefix of the extension namespace. If you want to use a key that is prefixed with `$`, you must quote it as `"$key"`.

## $swon

```swon
@ $swon
version = "https://swon.dev/v1"
schema = "https://swon.dev/schemas/swon-schema/v1"
```

- `$swon-lint.config = "https://swon.dev/default-configs/swon-lint/v1"`
- `$swon-fmt.config = "https://swon.dev/default-configs/swon-fmt/v1"`

## $variant

`$variant = <variant-name>` to indicate the variant of the current section.
Since JSON doesn't support variants, the schema must be specify which way to convert the variant to JSON data model.
