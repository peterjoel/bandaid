

pub enum MyIter<I: Iterator, J: Iterator<Item = I::Item>> {
    A(I),
    B(J),
}

impl<I, J> Iterator for MyIter<I, J> 
    where 
        I: Iterator,
        J: Iterator<Item = I::Item>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        match self {
            MyIter::A(it) => it.next(),
            MyIter::B(it) => it.next()
        }
    }
}

macro_rules! __if_else_iter {
    (
        if $p0:tt $b0:block
        else $b1:block
    ) => {
        #[allow(unused_parens)]
        {
            if $p0 { 
                ::MyIter::A($b0)
            } else {
                ::MyIter::B($b1)
            }
        }
    };
    (
        if $p0:tt $b0:block
        else if $($rest:tt)+
    ) => {
        #[allow(unused_parens)]
        {
            if $p0 { 
                ::MyIter::A($b0)
            } else {
                ::MyIter::B(__if_else_iter! {
                    if $($rest)+
                })
            }
        }
    };
}

macro_rules! __match_iter {
    (
        [$($done:pat,)*] match $val:tt {
            $p:pat => $r:expr $(,)*
        }
    ) => {
        match $val {
            $p => $r,
            $($done => unimplemented!(),)*
        }
    };
    (
        [$($done:pat,)*]
        match $val:tt {
            $p0:pat => $r0:expr,
            $($rest:tt)+
        }
    ) => {
        match $val {
            $p0 => ::MyIter::A($r0),
            _ => ::MyIter::B(__match_iter! {
                [$p0, $($done,)*]
                match $val {
                    $($rest)+
                }
            })
        }
    };
}

/// Fix things up with a macro!
/// 
/// If a `match` of `if else` expression can return different types of iterator on each arm
/// then `band_aid!` can fix it right up.
/// 
/// There is one small caveat, that conditions sometimes have to be placed in parentheses,
/// to work around a limitation of Rust macros.
/// 
/// # Examples
/// 
/// Here, `band_aid!` allows the function to return different types of iterator. Without 
/// `band_aid!`, this would be an error, or else you would have to resort to either boxing
/// the result or maintaining a custom iterator that could combine all of the others.
/// 
/// ```
/// fn mk_iter(foo: i32) -> impl Iterator<Item = i32> {
///     band_aid! {
///         if (foo < 0) {
///             vec![1,2,3].into_iter()
///         } else if (foo < 2) {
///             iter::once(4)
///         } else {
///             iter::empty()
///         }
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! band_aid {
    (
        if $p0:tt $b0:block
        $(else if $pi:tt $bi:block)*
        else $bn:block
    ) => {
        __if_else_iter!{
            if $p0 $b0
            $(else if $pi $bi)*
            else $bn
        }
    };
    (
        match $val:tt {
            $($rest:tt)+
        }
    ) => {
        __match_iter!{
            []
            match $val {
                $($rest)+
            }
        }
    };
}


#[cfg(test)]
mod test {
    use std::iter;
    #[test]
    fn if_else_expr() {
        fn mk_iter(foo: i32) -> impl Iterator<Item = i32> {
            band_aid! {
                if (foo < 0) {
                    vec![1,2,3].into_iter()
                } else if (foo < 2) {
                    iter::once(4)
                } else {
                    iter::empty()
                }
            }
        }
        assert_eq!(vec![4], mk_iter(1).collect::<Vec<_>>());
    }
    #[test]
    fn match_expr() {
        fn mk_iter(foo: i32) -> impl Iterator<Item = i32> {
            enum F { A, B, C(i32) }
            let foo = F::C(foo);
            band_aid! {
                match foo {
                    F::A => vec![1,2,3].into_iter(),
                    F::B => iter::empty(),
                    F::C(n) => iter::once(n),
                }
            }
        }
        assert_eq!(vec![4], mk_iter(4).collect::<Vec<_>>());
    }
}