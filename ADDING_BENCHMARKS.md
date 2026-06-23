# How to add a library to the benchmarks

## General organization

The main benchmark binary is in `benches/bench.rs`, which calls into public
sub-functions for each sub-benchmark in the crate lib.

The benchmark functionalities for each associated crate live under
`src/benchmark_*.rs`. Each library under test should have both its dependency
and all of its associated code gated by a dedicated feature. The full set of
libraries under test at any given time is set by the "default-encoding-set"
feature in `Cargo.toml`.

## Benchmark organization and naming

The benchmarks generally are divided among four suites, the datasets that are
seen in the README. Any new library should implement encoding & decoding as
appropriate for the top level data types of these four suites
(`datasets::log::Logs`, `datasets::mesh::Mesh`,
`datasets::minecraft_savedata::Players`, and `datasets::mk48::Update`) and
their subsidiary types. Most libraries can apply `derive`s to the appropriate
structs, though others may need `From` impls to convert to their generated
struct types.

Individual benchmarks are named by the pattern: `{suite}/{feature}/{bench}`, or
`{suite}/{feature}/{bench} ({variant})`. `{feature}` should almost always be
the unambiguous name of the crate, and if it is not a line must be added to
`tools/config.json` under `"crate_matching"` to clarify. (This is useful both
when multiple versions of a crate are under test, such as `bincode`, and when
a library has multiple modes that merit it appearing on multiple rows of the
results, such as `capnp`.)

If a library's encoding, decoding, etc. abilities have multiple modes the names
of the appropriate benchmarks under its benchmarking group can be parenthesized
with the `{variant}`: for example, `prost` tests encoding both with and without
including the time to convert into the library's generated struct, `bilrost`
can encode to different kinds of output buffers in different ways, and `rkyv`
supports accessing the contents of an archived struct with or without full
validation.

## Benchmarks for implementations of common encodings

Multiple different libraries may implement a common encoding, for example
JSON or CBOR. When a library (or a specific benchmark "feature" group)
implements a common encoding this way, it can be added to `tools/config.json`
under `"common_encodings"`, which causes libraries that work with similar data
encodings to be grouped together in the README tables.

## Achievement standards

There are a number of different things an encoding library may be able to do,
which we generally compare in common columns of the results table in the
README:

* **"serialize"**: Given an input struct, encode the contents of that struct to a
  string or byte buffer.
* **"deserialize"**: The reverse process, decoding that same data *back* into the
  same struct type (or the library's equivalent). The resulting data must be
  equal to the value from the input.
* **"borrow"**: Exactly the same as "deserialize", but with string data
  referring to the data still in the input buffer rather than copying it to a
  new allocation. The suites which have string data have corresponding struct
  types that contain borrowed string data instead, and implement the trait
  `datasets::BorrowableData` for this type association. For libraries which
  naturally support decoding from borrowed data[^textborrow],
* **"access"**: For zero-copy libraries, accessing a single field value inside
  the input buffer, without necessarily validating that the whole structure is
  valid and coherent.
* **"read"**: For zero-copy libraries, read each the values from the struct
  without necessarily creating an owned struct outside of the buffer being
  decoded.
* **"update"**: For zero-copy libraries, update some values in each of the
  sub-structs encoded in a *mutable* buffer containing the encoded data.

Output buffers may be preallocated and reused by the benchmarks.

[textborrow]: Some serde-based text encodings can even support fallibly
decoding borrowed strings; this may be possible in for example JSON when the
text never contains escapes. For these purposes the strings in the benchmark
suites should remain relatively simple, so it should be possible to benchmark
these capabilities (as `serde_json` does).

## Codegen

Generated code used by the library, such as code created from protobuf files,
should be regenerated at build time conditionally by feature. The generated
code should be checked in and up to date, and will be regenenerated and checked
for any changes in CI.
