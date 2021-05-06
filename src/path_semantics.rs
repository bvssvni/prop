//! # Path Semantics
//!
//! Path Semantics has a core axiom which is used to model mathematics.
//!
//! This core axiom is modelled here,
//! lifting proof of path semantical order to expressions of propositions.
//!
//! For more information, see
//! https://github.com/advancedresearch/path_semantics.

use crate::*;

/// Core axiom of Path Semantics.
pub type PSem<F1, F2, X1, X2> = Imply<
    And<And<Eq<F1, F2>, POrdProof<F1, X1>>,
        And<Imply<F1, X1>, Imply<F2, X2>>>,
    Eq<X1, X2>,
>;

/// Sends first argument of Logical AND to higher level.
pub type PAndFst<A, B, C, D> = Imply<
    And<Eq<And<A, B>, C>, Imply<C, D>>,
    Eq<A, D>,
>;
/// Sends second argument of Logical AND to higher level.
pub type PAndSnd<A, B, C, D> = Imply<
    And<Eq<And<A, B>, C>, Imply<C, D>>,
    Eq<B, D>,
>;

/// Proof of path semantical order.
#[derive(Copy)]
pub struct POrdProof<T, U>(std::marker::PhantomData<(T, U)>);

impl<T: POrd<U>, U> Default for POrdProof<T, U> {
    fn default() -> Self {
        POrdProof(std::marker::PhantomData)
    }
}

impl<T, U> Clone for POrdProof<T, U> {
    fn clone(&self) -> POrdProof<T, U> {
        POrdProof(std::marker::PhantomData)
    }
}

impl<T: 'static, U: 'static> Decidable for POrdProof<T, U> {
    fn decide() -> ExcM<POrdProof<T, U>> {
        Left(POrdProof(std::marker::PhantomData))
    }
}

impl<T, U> POrdProof<T, U> {
    /// Creates a new proof from trait constraints.
    pub fn new() -> Self where T: POrd<U> {
        Self::default()
    }

    /// Transivity of path semantical order.
    pub fn transitivity<V>(self, _: POrdProof<U, V>) -> POrdProof<T, V> {
        POrdProof(std::marker::PhantomData)
    }

    /// Transform left argument by equivalence.
    pub fn by_eq_left<V>(self, _: Eq<T, V>) -> POrdProof<T, V> {
        POrdProof(std::marker::PhantomData)
    }

    /// Transform right argument by equivalence.
    pub fn by_eq_right<V>(self, _: Eq<U, V>) -> POrdProof<T, U> {
        POrdProof(std::marker::PhantomData)
    }
}

/// Path semantical order.
///
/// This is implemented by types to define an order
/// such that symbols can not be used inconsistently.
///
/// Uses a marker feature to allow overlapping impls.
#[marker]
pub trait POrd<T> {}

/// Path semantical order for binary operators.
pub trait PBinOrd {
    /// The left argument.
    type Left;
    /// The right argument.
    type Right;
}

impl<T> POrd<T::Left> for T where T: PBinOrd {}
impl<T> POrd<T::Right> for T where T: PBinOrd {}
impl POrd<False> for True {}
impl<T, U> PBinOrd for And<T, U> {
    type Left = T;
    type Right = U;
}
impl<T, U> PBinOrd for Or<T, U> {
    type Left = T;
    type Right = U;
}
impl<T, U> PBinOrd for Imply<T, U> {
    type Left = T;
    type Right = U;
}
impl<T, U> PBinOrd for POrdProof<T, U> {
    type Left = T;
    type Right = U;
}

/// Composition.
pub fn comp<F1: Prop, F2: Prop, F3: Prop, F4: Prop, X1: Prop, X2: Prop>(
    f: PSem<F1, F2, F3, F4>,
    g: PSem<F3, F4, X1, X2>,
    pr_f1_f3: POrdProof<F1, F3>,
    pr_f3_x1: POrdProof<F3, X1>,
    f1_f3: Imply<F1, F3>,
    f2_f4: Imply<F2, F4>,
    f3_x1: Imply<F3, X1>,
    f4_x2: Imply<F4, X2>,
) -> PSem<F1, F2, X1, X2> {
    Rc::new(move |((f1_eq_f2, _pr_f1_x1), (_f1_x1, _f2_x2))| {
        let f3_eq_f4 = f(((f1_eq_f2, pr_f1_f3.clone()), (f1_f3.clone(), f2_f4.clone())));
        let x1_eq_x2 = g(((f3_eq_f4, pr_f3_x1.clone()), (f3_x1.clone(), f4_x2.clone())));
        x1_eq_x2
    })
}

/// Converts core axiom to `PAndFst`.
pub fn to_pand_fst<A: Prop, B: Prop, C: Prop, D: Prop>(
    p: PSem<And<A, B>, C, A, D>
) -> PAndFst<A, B, C, D> {
    let x: POrdProof<And<A, B>, A> = POrdProof::new();
    let y = Rc::new(move |(x, _)| x);
    Rc::new(move |(f, g)| p.clone()(((f, x.clone()), (y.clone(), g))))
}

/// Converts core axiom to `PAndSnd`.
pub fn to_pand_snd<A: Prop, B: Prop, C: Prop, D: Prop>(
    p: PSem<And<A, B>, C, B, D>
) -> PAndSnd<A, B, C, D> {
    let x: POrdProof<And<A, B>, B> = POrdProof::new();
    let y = Rc::new(move |(_, x)| x);
    Rc::new(move |(f, g)| p.clone()(((f, x.clone()), (y.clone(), g))))
}

/// Join `PAndFst` and `PAndSnd`.
pub fn pand_join<A: Prop, B: Prop, C: Prop, D: Prop>(
    p1: PAndFst<A, B, C, D>,
    p2: PAndSnd<A, B, C, D>,
) -> PSem<And<A, B>, C, And<A, B>, D> {
    Rc::new(move |((eq_f_c, _pr), (f_ab, g))| {
        let eq_a_d = p1.clone()((eq_f_c.clone(), g.clone()));
        let eq_b_d = p2.clone()((eq_f_c, g));
        let eq_a_d_copy = eq_a_d.clone();
        let eq_ab_d: Eq<And<A, B>, D> = (Rc::new(move |(a, _)| eq_a_d_copy.0(a)),
                       Rc::new(move |d| (eq_a_d.clone().1(d.clone()), eq_b_d.clone().1(d))));
        eq_ab_d
    })
}
