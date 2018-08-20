# bandaid
Rust macros to patch things up.

In particular, this fixes a common problem where iterators in different arms of `if else` or `match` expressions have incompatible types.

## Examples

The following code would not compile because each conditional branch returns an iterator of a different type:

```Rust
fn mk_iter(foo: i32) -> impl Iterator<Item = i32> {
    if foo < 0 {
        vec![1, 2, 3].into_iter()
    } else if foo < 2 {
        iter::once(4)
    } else {
        iter::empty()
    }
}
```

This can be quickly patched up with `band_aid!`:

```Rust
#[macro_use]
extern crate bandaid;

fn mk_iter(foo: i32) -> impl Iterator<Item = i32> {
    band_aid! {
        if (foo < 0) {
            vec![1, 2, 3].into_iter()
        } else if (foo < 2) {
            iter::once(4)
        } else {
            iter::empty()
        }
    }
}
```

The main caveat is that some extra parantheses need to be introduced around the conditions, as you can see in the example. This is to work around a limitation of Rust macros.

