# extendr - A safe and user friendly R extension interface using Rust.

Low-level R library bindings

[![Travis Build Status](https://api.travis-ci.org/extendr/extendr.svg?branch=master)](https://travis-ci.org/extendr/extendr)
[![Crates.io](http://meritbadge.herokuapp.com/extendr-api)](https://crates.io/crates/extendr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://docs.rs/mio/badge.svg)](https://docs.rs/extendr-api/latest/extendr_api/)

## Installation - Rust

Extendr is available on [crates.io](https://crates.io/crates/extendr_api).

Simply add this line to the `[dependencies]` section of a rust crate.
You will then be able to call R code from Rust.

```toml
[dependencies]
extendr_api = "0.1.3"
```
## Installation - R

For now, copy one of the examples in `examples/R`. An R package will
be available shortly help to build R packages in Rust.

## Overview

Extendr is a Rust extension mechanism for R

It is intended to be easier to use than the C interface and
Rcpp as Rust gives type safety and freedom from segfaults.

The following code illustrates a simple structure trait
which is written in Rust. The data is defined in the `struct`
declaration and the methods in the `impl`.

```
use extendr_api::*;

struct Person {
    pub name: String,
}

#[extendr]
impl Person {
    fn new() -> Self {
        Self { name: "".to_string() }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[extendr]
fn aux_func() {
}


// Macro to generate exports
extendr_module! {
    mod classes;
    impl Person;
    fn aux_func;
}
```

The `#[extendr]` attribute causes the compiler to generate
wrapper and registration functions for R which are called
when the package is loaded.

The `extendr_module!` macro lists the module name and exported functions
and interfaces.

This library aims to provide an interface that will be familiar to
first-time users of Rust or indeed any compiled language.

Anyone who knows the R library should be able to write R extensions.

## Goals of the project

Instead of wrapping R objects, we convert to Rust native objects
on entry to a function. This makes the wrapped code clean and dependency
free. The ultimate goal is to allow the wrapping of existing 
Rust libraries without markup, but in the meantime, the markup
is as light as possible.

```
#[extendr]
pub fn my_sum(v: &[f64]) -> f64 {
    v.iter().sum()
}
```

You can interact in more detail with R objects using the RObj
type which wraps the native R object type. This supports a large
subset of the R internals functions, but wrapped to prevent
accidental segfaults and failures.

```
## extendr roadmap

### Basic
- [x] Be able to build simple rust extensions for R.
- [x] Wrap the R SEXP object safely (Robj)
- [x] Iterator support for matrices and vectors.
- [x] Class support.

### Documentation
- [x] Begin documentation.
- [ ] Begin book-form documentation.
- [ ] Paper for Bioinformatics.
- [ ] Build and publish CRAN R package.
- [ ] Publish Use R! series book.

### Automation
- [x] Auto-generate binding wrappers.
- [ ] Auto-generate NAMESPACE and lib.R.

### Features
- [ ] Feature-gated support for ndarray.
- [ ] Feature-gated support for rayon.

### R packages
- [ ] Bindings for rust-bio

