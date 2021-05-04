//! Tactics for Logical EQ.

use crate::*;

/// `(a = b) ∧ (b = c) => (a = c)`.
pub fn transitivity<A: Prop, B: Prop, C: Prop>((f0, f1): Eq<A, B>, (g0, g1): Eq<B, C>) -> Eq<A, C> {
    (Rc::new(move |x| g0(f0(x))), Rc::new(move |x| f1(g1(x))))
}

/// `a => (a = ¬¬a)`.
pub fn double_neg<A: Prop>(a: A) -> Eq<A, Not<Not<A>>> {
    let double_neg = a.double_neg();
    (Rc::new(move |x| not::double(x)), Rc::new(move |x| double_neg(x)))
}

/// `(a = b) => (b = a)`.
pub fn commute<A: Prop, B: Prop>((f0, f1): Eq<A, B>) -> Eq<B, A> {
    (f1, f0)
}

/// `(a => b) = (¬a ∨ b)`.
pub fn imply_to_or<A: Prop, B: Prop>() -> Eq<Imply<A, B>, Or<Not<A>, B>> {
    (Rc::new(move |x| imply::to_or(x)), Rc::new(move |x| imply::from_or(x)))
}

/// `a = a`.
pub fn refl<A: Prop>() -> Eq<A, A> {
    (Rc::new(move |x| x), Rc::new(move |x| x))
}

/// `(a = b) = (¬a = ¬b)`
pub fn modus_tollens<A: Prop, B: Prop>((f0, f1): Eq<A, B>) -> Eq<Not<B>, Not<A>> {
    let f02 = imply::modus_tollens(f0);
    let f12 = imply::modus_tollens(f1);
    (f02, f12)
}

/// `(¬a = ¬b) = (a = b)`
pub fn rev_modus_tollens<A: Prop, B: Prop>((f0, f1): Eq<Not<A>, Not<B>>) -> Eq<B, A> {
    let f02 = imply::rev_modus_tollens(f0);
    let f12 = imply::rev_modus_tollens(f1);
    (f02, f12)
}

/// `(true = a) => a`.
pub fn is_true<A: Prop>((f0, _): Eq<True, A>) -> A {
    f0(True)
}

/// `(false = a) => ¬a`.
pub fn is_false<A: Prop>((_, f1): Eq<False, A>) -> Not<A> {
    f1
}
