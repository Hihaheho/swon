# Extensions

Extensions are used to provide additional information without affecting the data. `$` is the prefix of the extension namespace. If you want to use a key that is prefixed with `$`, you must quote it as `"$key"`.
Extensions should be placed before any other bindings.

## $swon

```swon
@ $swon
# version of the SWON specification
version: https://swon.dev/v1
# schema of this SWON file
schema: https://swon.dev/schemas/swon-schema/v1
```

- `$swon-lint.config = "https://swon.dev/default-configs/swon-lint/v1"`
- `$swon-fmt.config = "https://swon.dev/default-configs/swon-fmt/v1"`
- `$root = "script"` to indicate the root key of this section
- `$data-model = "json"` to limit the data types that can be used in a data model, like path data in JSON.

## $variant

`$variant = <variant-name>` to indicate the variant of the current section.
Since JSON doesn't support variants, the schema must be specify which way to convert the variant to JSON data model.

## $local

`$local is a namespace should be used for generic in-document local data store.

Any object can have this extension.
