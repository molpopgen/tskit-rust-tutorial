# Introduction

## Do you need `tskit-rust`?

The use-cases for `tskit-rust` are the same as for `tskit-c`:

1. Developing a new performance-oriented application.
2. The input/output of this application will be a `.trees` file.

## What does `tskit-rust` add?

Briefly, you get the performance of `C` and the strong safety guarantees of `rust`.

## What is `tskit-rust` missing?

The crate does not cover the entire `C` API.
However, client code can make direct calls to that API via the module `tskit::bindings`.

## Adding `tskit` as a dependency to a rust project

In your `Cargo.toml`:

```{toml}
[dependencies]
tskit = "~0.9.0"
```

## Conventions used in this document

We assume a working knowledge of rust.
Thus, we skip over the details of things like matching idioms, etc.,
and just `.unwrap()`.

We also assume familiarity with the `tskit` [data model](https://tskit.dev/tskit/docs/stable/data-model.html).
