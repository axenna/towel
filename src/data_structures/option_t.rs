use crate::prelude::{Applicative, Bound, Functor, Monad};
use std::marker::PhantomData;

//reducing visual noise
type O<A> = Option<A>;

/// An Option Transformer. Composition of a Monadic type and
/// Option.
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// # use towel::data_structures::{Either, OptionT};
/// let x = OptionT::<Either<Option<i32>, Option<i32>>, i32>::new(3);
///
/// //OptionT(Right(Some(3)), PhantomData)
/// println!("{:?}", x);
///
/// ```
#[derive(Debug, PartialEq)]
//using phantom &'a A to tie OptionT to both a lifetime and the type A
pub struct OptionT<'a, M, A>(M, PhantomData<&'a A>);

impl<'a, M, A, B: 'a> Bound<B> for OptionT<'a, M, A>
where
    M: Bound<O<B>>,
{
    //the Bound for optionT, is optionT around the bound for the inner
    //monad when it has an option in it
    type Bound = OptionT<'a, <M as Bound<O<B>>>::Bound, PhantomData<B>>;

    fn wrap(a: B) -> Self::Bound {
        OptionT(<M as Bound<O<B>>>::wrap(Some(a)), PhantomData)
    }
}

impl<'a, M, A, B: 'a, F> Functor<A, B, F> for OptionT<'a, M, A>
where
    //without the lifetime on Box<dyn Fn> it is set as static, so F will also need
    //to be static. But if we want F to be lifetime A, Box<dyn Fn> also has to be lifetime
    //A
    M: Functor<O<A>, O<B>, Box<dyn FnOnce(O<A>) -> O<B> + 'a>>,
    F: FnOnce(A) -> B + 'a,
{
    //because the return type, Bound borrows f, it has to have the same lifetime as f
    //because Bound is an OptionT, OT needs to have the same lifetime as f
    //hence the confusing lifetime usages
    fn fmap(self, f: F) -> Self::Bound {
        OptionT(self.0.fmap(Box::new(move |a| a.fmap(f))), PhantomData)
    }
}

impl<'a, M, A, B: 'a, C: 'a, F> Applicative<A, B, C, F> for OptionT<'a, M, A>
where
    M: Applicative<O<A>, O<B>, O<C>, Box<dyn FnOnce(O<A>, O<B>) -> O<C> + 'a>>,
    F: FnOnce(A, B) -> C + 'a,
{
    type Other = OptionT<
        'a,
        //love me some fully qualified rust syntax
        //basically just as applicative while telling
        //the compiler I know what I'm talking about
        <M as Applicative<O<A>, O<B>, O<C>, Box<dyn FnOnce(O<A>, O<B>) -> O<C> + 'a>>>::Other,
        PhantomData<B>,
    >;

    fn lift_a2(self, other: Self::Other, f: F) -> Self::Bound {
        OptionT(
            //lift inner self and inner other using a function that utilizes
            //the lift_a2 for option to combine the 2 values inside the options
            self.0
                .lift_a2(other.0, Box::new(move |a, b| a.lift_a2(b, f))),
            PhantomData,
        )
    }
}

impl<'a, M, A, B: 'a, F> Monad<A, B, F> for OptionT<'a, M, A>
where
    M: Monad<O<A>, O<B>, Box<dyn FnOnce(O<A>) -> <M as Bound<O<B>>>::Bound + 'a>>,
    F: FnOnce(A) -> Self::Bound + 'a,
{
    fn bind(self, f: F) -> Self::Bound {
        OptionT(
            //uses inner monads bind and makes a function that fits that signature
            self.0.bind(Box::new(move |a| match a {
                //a is type O<A> instead of binding on it direcly
                //because the f we have takes us to an OptionT::Bound
                //we pattern match if it is none we return None lifted into monadic structure
                //satisfying type of inner bind
                //If it has a value we run our function a -> OptionT then access internal value
                //to satisfy type of inner bind fn
                Some(b) => f(b).0,
                None => <M as Bound<O<B>>>::wrap(None),
            })),
            PhantomData,
        )
    }
}

impl<'a, M, A> OptionT<'a, M, A>
where
    M: Bound<O<A>, Bound = M>,
{
    pub fn new(a: A) -> Self {
        OptionT(<M as Bound<O<A>>>::wrap(Some(a)), PhantomData)
    }
}
