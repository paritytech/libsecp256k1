# SECP256K1 implementation in pure Rust

* [Cargo](https://crates.io/crates/libsecp256k1)
* [Documentation](https://docs.rs/libsecp256k1)

SECP256K1 implementation with `no_std` support. Currently we have implementation for:

* Convert a private key to a public key.
* Sign messages.
* Signature verification.
* Public key recovery from signed messages.
* Shared secrets.

## Feature flags

* `std`: If disabled, works in `no_std` environment. Enabled by default.
* `hmac`: Add certain features that requires the HMAC-DRBG. This includes
  signing. Enabled by default.
* `static-context`: To speed up computation, the library uses a pre-computed
  table context for many `ecmult` operations. This feature flag puts the context
  directly as static variables. If disabled, the context must be created from
  heap manually. Increases binary size, enabled by default.
* `lazy-static-context`: Instead of storing the pre-computed table context as
  static variables, store it as a variable that dynamically allocates the
  context in heap via `lazy_static`. Only one of `static-context` and
  `lazy-static-context` can be enabled, or both disabled. Impact bootstrap
  performance, disabled by default.
