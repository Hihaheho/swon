# swon-template

A structured and type-safe templating tool for swon.

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
