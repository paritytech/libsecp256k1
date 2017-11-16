# SECP256K1 Implementation in Pure Rust

[![Build Status](https://travis-ci.org/ethereumproject/libsecp256k1-rs.svg?branch=master)](https://travis-ci.org/ethereumproject/libsecp256k1-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE)
[![Cargo](https://img.shields.io/crates/v/libsecp256k1.svg)](https://crates.io/crates/libsecp256k1)

SECP256K1 implementation with `no_std` support. Currently we have
implementation for:

* Signature verification.
* Public key recovery from signed messages.
