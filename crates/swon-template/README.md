# swon-template

Templating extension for SWON (SWan Object Notation) files.

A structured and type-safe templating tool for swon.

**This crate is still under development and published for name reservation purpose.**

Part of the [SWON](https://swon.dev) project - a minimalist, schema-friendly format with an algebraic data model that's compatible with JSON.

## Design

A template is a normal swon file which using `$template` extensions.

```swon
name.$template.path = .name
name.$type: string

childs.$template.for {
  path = .childs
  map {
    name.$template.if = .childs[].active
    name.$template.path = .childs[].name
    name.$type: string
  }
}
```
