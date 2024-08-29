# cxx_linker
A procedural macro for linking Rust functions with C++ using Itanium name mangling.
# Example
```rust
use cxx_linker::cxx_link;

extern "C" {
    #[cxx_link("blobstore::foo<int>::bar(int, blobstore::SomeType&, bool)")]
    fn bar(a: i32, b: *mut (), c: bool);
}
```
will be expanded to
```rust
extern "C" {
    #[link_name = "_ZN9blobstore3fooIiE3barEiRNS_8SomeTypeEb"]
    fn bar(a: i32, b: *mut (), c: bool);
}
```
