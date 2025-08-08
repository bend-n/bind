#![feature(tuple_trait, unboxed_closures, fn_traits)]
#![allow(private_bounds)]
use core::ops::FnMut;
use std::marker::Tuple;

trait Head: Tuple {
    type Head;
    type Tail: Tuple;

    fn rejoin(_: Self::Head, _: Self::Tail) -> Self;
}

macro_rules! impls {(
  $($Hd:tt $($Tail:tt)*)?
) => ($(impls!($($Tail)*);)?$(
impl<$Hd $(, $Tail)*> Head for ($Hd, $($Tail),*) {
    type Head = $Hd;
    type Tail = ($($Tail, )*);
    fn rejoin($Hd: Self::Head, tail: Self::Tail) -> Self {
        let ($($Tail, )*) = tail;
        ($Hd, $($Tail),*)
    }
})?)}
impls![_13 _12 _11 _10 _9 _8 _7 _6 _5 _4 _3 _2 _1 _0];

impl<Args: Head, F: FnMut<Args> + Sized> Bind<Args> for F {}
pub trait Bind<Args: Head>: FnMut<Args> + Sized {
    fn bind<T: Clone>(self, with: T) -> impl FnMut<Args::Tail, Output = Self::Output>
    where
        Args: Head<Head = T>,
    {
        return Fn { f: self, t: with };
        struct Fn<Args: Head, F: FnMut<Args>> {
            f: F,
            t: Args::Head,
        }
        impl<Args: Head, F: FnMut<Args>> FnOnce<Args::Tail> for Fn<Args, F> {
            type Output = F::Output;
            extern "rust-call" fn call_once(mut self, args: Args::Tail) -> Self::Output {
                let args = Args::rejoin(self.t, args);
                self.f.call_mut(args)
            }
        }
        impl<Args: Head<Head: Clone>, F: FnMut<Args>> FnMut<Args::Tail> for Fn<Args, F> {
            extern "rust-call" fn call_mut(&mut self, args: Args::Tail) -> Self::Output {
                let args = Args::rejoin(self.t.clone(), args);
                self.f.call_mut(args)
            }
        }
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
