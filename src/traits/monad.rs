use crate::traits::{Applicative, Functor};

/// Trait describing function application in a context
/// where funtion application generates more structure
pub trait Monad<'a, A>: Applicative<'a, A> {
    
    /// Maps values inside structure to structure then reduces to one layer of structure
    ///
    /// # Examples
    ///
    /// Basic Usage:
    ///
    /// ```
    /// use towel::traits::Monad;
    /// 
    /// //Vec example
    /// let v = vec![1, 2, 3];
    /// 
    /// //additional Vec structure generated by function application
    /// let f: fn(&i32) -> Vec<i32> = |&x| vec![x, x];
    ///
    /// //structure reduce to only one layer
    /// assert_eq!(v.bind(&f), vec![1, 1, 2, 2, 3, 3]);
    ///
    /// //Option example
    /// let o = Some("hello world");
    /// let p = Some("foo bar");
    /// 
    /// //additional option structure generated by function application
    /// let g: fn(&&str) -> Option<&'static str> = |x| {
    ///     if *x == "hello world"{
    ///         Some("hello caller")
    ///     }
    ///     else {
    ///         None
    ///     }
    /// };
    /// 
    /// //structure reduced to one lyaer
    /// assert_eq!(o.bind(&g), Some("hello caller"));
    ///
    /// //structure reduced to one lyaer
    /// assert_eq!(p.bind(&g), None);
    fn bind<B, F: Fn(&A) -> Self::HKT<B> + 'a>(&'a self, f: F) -> Self::HKT<B>;
}

impl<'a, A: 'a> Monad<'a, A> for Vec<A> {
    fn bind<B, F: Fn(&A) -> Self::HKT<B>>(&self, f: F) -> Self::HKT<B> {
        self.fmap(f).into_iter().flatten().collect()
    }
}

//map value inside Option to Option then reduce structure
impl<'a, A: 'a> Monad<'a, A> for Option<A> {
    fn bind<B, F: Fn(&A) -> Self::HKT<B>>(&self, f: F) -> Self::HKT<B> {
        match self {
            None => None,
            Some(a) => f(a),
        }
    }
}
