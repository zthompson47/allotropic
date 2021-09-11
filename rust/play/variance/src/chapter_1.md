# Variance

The Rust Reference has a table on
[Variance](https://doc.rust-lang.org/reference/subtyping.html) which lists types
and their respective variance in lifetime (`'a`) and type (`T`).  It refers to
the concept of "subtyping relation", which orders lifetimes and types by whether
one contains a subset of the other's abilities.  For example, `'static` is a
subtype of a `'a` because it encompasses all lifetimes, including `'a`.  A 

## `&'a T` variance in `'a`
## `&'a T` variance in `T`
