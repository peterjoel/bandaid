

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
            let foo = F::C(4);
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