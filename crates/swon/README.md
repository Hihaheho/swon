# swon

SWON (SWan Object Notation) is a minimalist, schema-friendly format with an algebraic data model that's compatible with JSON.

This is the core library for working with SWON format.

**This crate is still under development and published for name reservation purpose.**

## Features

- Minimalist syntax
- Schema-friendly design
- Algebraic data model
- JSON data model compatible
- Rich editor experience
- Human friendly
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
```

Visit [swon.dev](https://swon.dev) for more information.
