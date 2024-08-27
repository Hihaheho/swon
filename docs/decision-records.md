# (Architecture) Decision Memos

## Abandon cool, but `$ref-*` extensions

- A SWON document should be self-contained as a sane data serialization format. It's always surprising that desirialization requires network access or file system access.
- It will be big security holes, especially with third-party implementations.
