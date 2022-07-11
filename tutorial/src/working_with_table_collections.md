# Creating a TableCollection

## Creation

We initialize a `TableCollection` with a sequence length.
In `tskit-c`, the genome length is a C `double`.
Here it is a [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) called `tskit::Position`:

```rust, noplaygound, ignore
{{#include ../../tests/table_collection.rs:create_table_collection_with_newtype}}
```

The newtype pattern gives type safety by disallowing you to send a position to a function where a time is required, etc..
However, it can be inconvenient to type out the full type names every time.
Thus, the API defines most functions taking arguments `Into<T>` where `T` is one of our newtypes.
This design means that the following is equivalent to what we wrote above:

```rust, noplaygound, ignore
{{#include ../../tests/table_collection.rs:create_table_collection}}
```

## Adding rows to tables

For each table type (node, edge., etc.), we have a function to add a row.
For example, to add a node:

```rust, noplaygound, ignore
{{#include ../../tests/table_collection.rs:add_node_without_metadata}}
```

Again, we can take advantage of being able to pass in any type that is `Into<_>` the required newtype:

```rust, noplaygound, ignore
{{#include ../../tests/table_collection.rs:add_node_without_metadata_using_into}}
```

See the [API docs](https://docs.rs/tskit) for more details and examples.

## Checking table integrity

The data model involves lazy checking of inputs.
In other words, we can add invalid row data that is not caught by the "add row" functions.
We inherit this behavior from the C API.

We can check that the tables contain valid data by:

```rust, noplaygound, ignore
{{#include ../../tests/table_collection.rs:integrity_check}}
```

