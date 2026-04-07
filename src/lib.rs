#![feature(tuple_trait, unboxed_closures, fn_traits)]
#![allow(private_bounds)]
use core::ops::FnMut;

use ttools::Tupl;

impl<Args: Tupl, F: FnMut<Args> + Sized> Bind<Args> for F {}
pub struct Fn1<Args: Tupl, F: FnMut<Args>> {
    f: F,
    t: Args::Head,
}
impl<Args: Tupl, F: FnMut<Args>> FnOnce<Args::Tail> for Fn1<Args, F> {
    type Output = F::Output;
    extern "rust-call" fn call_once(mut self, args: Args::Tail) -> Self::Output {
        let args = Args::cons(self.t, args);
        self.f.call_mut(args)
    }
}
impl<Args: Tupl<Head: Clone>, F: FnMut<Args>> FnMut<Args::Tail> for Fn1<Args, F> {
    extern "rust-call" fn call_mut(&mut self, args: Args::Tail) -> Self::Output {
        let args = Args::cons(self.t.clone(), args);
        self.f.call_mut(args)
    }
}

pub struct Fn2<Args: Tupl, F: FnMut<Args>> {
    f: F,
    t: Args::Last,
}
impl<Args: Tupl, F: FnMut<Args>> FnOnce<Args::Init> for Fn2<Args, F> {
    type Output = F::Output;
    extern "rust-call" fn call_once(mut self, args: Args::Init) -> Self::Output {
        let args = Args::snoc(args, self.t);
        self.f.call_mut(args)
    }
}
impl<Args: Tupl<Last: Clone>, F: FnMut<Args>> FnMut<Args::Init> for Fn2<Args, F> {
    extern "rust-call" fn call_mut(&mut self, args: Args::Init) -> Self::Output {
        let args = Args::snoc(args, self.t.clone());

        self.f.call_mut(args)
    }
}

pub trait Bind<Args: Tupl>: FnMut<Args> + Sized {
    fn bind<T>(self, with: T) -> Fn1<Args, Self>
    where
        Args: Tupl<Head = T>,
    {
        return Fn1 { f: self, t: with };
    }
    fn rbind<T: Clone>(self, with: T) -> Fn2<Args, Self>
    where
        Args: Tupl<Last = T>,
    {
        return Fn2 { f: self, t: with };
    }
}

pub trait Compose<I, R> {
    fn compose<T>(self, other: impl FnMut(T) -> I) -> impl FnMut(T) -> R;
}
impl<I, R, F: FnMut(I) -> R> Compose<I, R> for F {
    fn compose<T>(mut self, mut other: impl FnMut(T) -> I) -> impl FnMut(T) -> R {
        move |x| self(other(x))
    }
}

#[test]
fn x() {
    let x = |x: u32, y: u32, z: u32| x + y + z;
    let mut x = x.bind(4).bind(5);
    let z = x(4);
}
