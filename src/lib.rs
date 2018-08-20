

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

macro_rules! band_aid {
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
                ::MyIter::B(band_aid! {
                    if $($rest)+
                })
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
        mk_iter(1);
    }
}