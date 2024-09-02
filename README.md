# SWON

## Focus

- Minimalist
- Schema-frieldly
- Algebraic data model
- JSON data model compatible
- JSON schema compatible
- Rich Editor Experience
- Human friendly
- Dedicated templating language
- Programmatically editable

## Example

```swon
$swon.version: https://swon.dev/versions/v0.1.0
title: test
language: en
target_window: Main

@ actions[]
$variant: use-script
script-id: title

@ actions[]
$variant: sleep
seconds = 2.0

@ actions[]
$variant: set-base-background-image
image: images/backgrounds/blank.png

@ actions[]
$variant: set-scene
scene: scenes/room_a.scn.ron

@ actions[] {
  $variant: set-text

  @ pages[]
  text: Hi,

  @ pages[]
  speaker: Ryo
  text: I'm Ryo.
}
```

## TODO

- [ ] swon-parol: Complete the grammar and parser
- [ ] swon-ls: Syntax highlighting
- [ ] swon-schema: SWON Schema specification
- [ ] swon-ls: Schema validation
- [ ] serde-swon: Serde support
- [ ] swon-dev: Making the landing page on <https://swon.dev>
- [ ] swon-fmt: Make the formatter
- [ ] swon-cli: sub command to convert SWON to other formats
- [ ] swon-lint: Some lint rules
- [ ] swon-template: Templating extension for SWON files

## Credits

- [Parol](https://github.com/jsinger67/parol) for the parser generator
- [TOML](https://toml.io) for the document structure and its minimalisity
- [jq](https://jqlang.github.io/jq/) for the key syntax
- [Serde](https://serde.rs/) for the data model and attributes (especially about enum representation)
- [JSON Schema](https://json-schema.org) for the idea of describing schema in the same language as the data
- [Helm](https://helm.sh) for the idea of templating
- [YAML](https://yaml.org) for easy nesting and the `:` syntax
