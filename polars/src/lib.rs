//! # Polars: *<small>DataFrames in Rust</small>*
//!
//! Polars is a DataFrame library for Rust. It is based on [Apache Arrows](https://arrow.apache.org/) memory model.
//! Apache arrow provides very cache efficient columnar data structures, and is becoming the defacto
//! standard for columnar data.
//!
//! This means that Polars data structures can be shared zero copy with processes in many different
//! languages.
//!
//! ## Tree Of Contents
//!
//! * [Cookbooks](#cookbooks)
//! * [Data structures](#data-structures)
//!     - [DataFrame](#dtaframe)
//!     - [Series](#series)
//!     - [ChunkedArray](#chunkedarray)
//! * [SIMD](#simd)
//! * [API](#api)
//! * [Compile times](#compile-times)
//! * [Performance](#performance-and-string-data)
//!     - [Custom allocator](#custom-allocator)
//! * [Config](#config-with-env-vars)
//! * [WASM target](#compile-for-wasm)
//!
//! ## Cookbooks
//! See examples in the cookbooks:
//!
//! * [Eager](crate::docs::eager)
//! * [Lazy](crate::docs::lazy)
//!
//! ## Data Structures
//! The base data structures provided by polars are `DataFrame`, `Series`, and `ChunkedArray<T>`.
//! We will provide a short top down view of these data structures.
//!
//! ### DataFrame
//! A `DataFrame` is a 2 dimensional data structure that is backed by a `Series`, and it could be
//! seen as an abstraction on `Vec<Series>`. Operations that can be executed on `DataFrame`s are very
//! similar to what is done in a `SQL` like query. You can `GROUP`, `JOIN`, `PIVOT` etc. The closes
//! arrow equivalent to a `DataFrame` is a [RecordBatch](https://docs.rs/arrow/4.0.0/arrow/record_batch/struct.RecordBatch.html),
//! and Polars provides zero copy coercion.
//!
//! ### Series
//! `Series` are the type agnostic columnar data representation of Polars. They provide many
//! operations out of the box, many via the [Series struct](crate::prelude::Series) and
//! [SeriesTrait trait](crate::series/trait.SeriesTrait.html). Whether or not an operation is provided
//! by a `Series` is determined by the operation. If the operation can be done without knowing the
//! underlying columnar type, this operation probably is provided by the `Series`. If not, you must
//! downcast to the typed data structure that is wrapped by the `Series`. That is the `ChunkedArray<T>`.
//!
//! ### ChunkedArray
//! `ChunkedArray<T>` are wrappers around an arrow array, that can contain multiples chunks, e.g.
//! `Vec<dyn ArrowArray>`. These are the root data structures of Polars, and implement many operations.
//! Most operations are implemented by traits defined in [chunked_array::ops](crate::chunked_array::ops),
//! or on the [ChunkedArray struct](crate::chunked_array::ops).
//!
//! ## SIMD
//! Polars / Arrow uses packed_simd to speed up kernels with SIMD operations. SIMD is an optional
//! `feature = "simd"`, and requires a nightly compiler. If you don't need SIMD, **Polars runs on stable!**
//!
//! ## API
//! Polars supports an eager and a lazy API, and strives to make them both equally capable.
//! The eager API is similar to [pandas](https://pandas.pydata.org/), and is easy to get started.
//! The lazy API is similar to [Spark](https://spark.apache.org/), and builds a query plan that will
//! be optimized. This may be less intuitive, but you may gain of additional performance.
//!
//! ### Eager
//! Read more in the pages of the following data structures /traits.
//!
//! * [DataFrame struct](crate::frame::DataFrame)
//! * [Series struct](crate::series::Series)
//! * [Series trait](crate::series::SeriesTrait)
//! * [ChunkedArray struct](crate::chunked_array::ChunkedArray)
//! * [ChunkedArray operations traits](crate::chunked_array::ops)
//!
//! ### Lazy
//! Unlock full potential with lazy computation. This allows query optimizations and provides Polars
//! the full query context so that the fastest algorithm can be chosen.
//!
//! **[Read more in the lazy module.](polars_lazy)**
//!
//! ## Compile times
//! A DataFrame library typically consists of
//!
//! * Tons of features
//! * A lot of datatypes
//!
//! Both of these really put large strains on compile times. To keep Polars lean, we make both **opt-in**,
//! meaning that you only pay the compilation cost, if you need it.
//!
//! ## Compile times and opt-in features
//! The opt-in features are:
//!
//! * `pivot` - [pivot operation](crate::frame::groupby::GroupBy::pivot) on `DataFrame`s
//! * `random` - Generate array's with randomly sampled values
//! * `ndarray`- Convert from `DataFrame` to `ndarray`
//! * `downsample` - [downsample operation](crate::frame::DataFrame::downsample) on `DataFrame`s
//! * `sort_multiple` - Allow sorting a `DataFrame` on multiple columns
//! * `is_in` - [Check for membership in `Series`](crate::chunked_array::ops::IsIn)
//! * `temporal` - Conversions between [Chrono](https://docs.rs/chrono/) and Polars for temporal data types
//! * `simd (nightly only)` - SIMD operations
//! * `parquet` - Read Apache Parquet format
//! * `json` - Json serialization
//! * `ipc` - Arrow's IPC format serialization
//! * `lazy` - Lazy API
//! * `strings` - Extra string utilities for `Utf8Chunked`
//! * `object` - Experimental support for generic ChunkedArray's called `ObjectChunked<T>` (generic over `T`).
//!              These will downcastable from Series through the [Any](https://doc.rust-lang.org/std/any/index.html) trait.
//! * `serde` - Support for [serde](https://crates.io/crates/serde) serialization and deserialization
//!
//! ## Compile times and opt-in data types
//! As mentioned above, Polars `Series` are wrappers around
//! `ChunkedArray<T>` without the generic parameter `T`.
//! To get rid of the generic parameter, all the possible value of `T` are compiled
//! for `Series`. This gets more expensive the more types you want for a `Series`. In order to reduce
//! the compile times, we have decided to default to a minimal set of types and make more `Series` types
//! opt-in.
//!
//! Note that if you get strange compile time errors, you probably need to opt-in for that `Series` dtype.
//! The opt-in dtypes are:
//!
//! | data type               | feature flag      |
//! |-------------------------|-------------------|
//! | Time64NanoSecondType    | dtype-time64-ns   |
//! | DurationNanosecondType  | dtype-duration-ns |
//! | DurationMillisecondType | dtype-duration-ms |
//! | Date32Type              | dtype-date32      |
//! | Date64Type              | dtype-date64      |
//! | Int8Type                | dtype-i8          |
//! | Int16Type               | dtype-i16         |
//! | UInt8Type               | dtype-u8          |
//! | UInt16Type              | dtype-u16         |
//! | UInt64Type              | dtype-u64         |
//!
//!
//! Or you can choose on of the preconfigured pre-sets.
//!
//! * `dtype-full` - all opt-in dtypes.
//! * `dtype-slim` - slim preset of opt-in dtypes.
//!
//! ## Performance and string data
//! Large string data can really slow down your queries.
//! Read more in the [performance section](crate::docs::performance)
//!
//! ### Custom allocator
//! A DataFrame library naturally does a lot of heap allocations. It is recommended to use a custom
//! allocator. [Mimalloc](https://docs.rs/mimalloc/0.1.25/mimalloc/) for instance, shows a significant
//! performance gain in runtime as well as memory usage.
//!
//! #### Usage
//! ```ignore
//! use mimalloc::MiMalloc;
//!
//! #[global_allocator]
//! static GLOBAL: MiMalloc = MiMalloc;
//! ```
//!
//! #### Cargo.toml
//! ```ignore
//! [dependencies]
//! mimalloc = { version = "*", default-features = false }
//! ```
//! ## Config with ENV vars
//!
//! * `POLARS_PAR_SORT_BOUND` -> Sets the lower bound of rows at which Polars will use a parallel sorting algorithm.
//!                              Default is 1M rows.
//! * `POLARS_FMT_MAX_COLS` -> maximum number of columns shown when formatting DataFrames.
//! * `POLARS_FMT_MAX_ROWS` -> maximum number of rows shown when formatting DataFrames.
//! * `POLARS_TABLE_WIDTH` -> width of the tables used during DataFrame formatting.
//! * `POLARS_MAX_THREADS` -> maximum number of threads used to initialize thread pool (on startup).
//! * `POLARS_VERBOSE` -> print logging info to stderr
//!
//! ## Compile for WASM
//! To be able to pretty print a `DataFrame` in `wasm32-wasi` you need to patch the `prettytable-rs`
//! dependency. If you add this snippet to your `Cargo.toml` you can compile and pretty print when
//! compiling to `wasm32-wasi` target.
//!
//! ```toml
//! [patch.crates-io]
//! prettytable-rs = { git = "https://github.com/phsym/prettytable-rs", branch = "master"}
//! ```
//!
//! ## User Guide
//! If you want to read more, [check the User Guide](https://pola-rs.github.io/polars-book/).
pub mod docs;
pub mod prelude;

pub use polars_core::{
    chunked_array, datatypes, doc, error, frame, functions, series, testing, toggle_string_cache,
};

pub use polars_core::apply_method_all_arrow_series;
pub use polars_core::df;

#[cfg(feature = "polars-io")]
pub use polars_io as io;
#[cfg(feature = "lazy")]
pub use polars_lazy as lazy;
