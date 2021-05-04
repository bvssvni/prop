//! Tactics for Logical AND.

use crate::*;

/// `a ∧ b => b ∧ a`.
pub fn commute<A: Prop, B: Prop>((f0, f1): And<A, B>) -> And<B, A> {
    (f1, f0)
}

/// `(a ∧ b) ∧ c  =>  a ∧ (b ∧ c)`
pub fn assoc<A: Prop, B: Prop, C: Prop>(
    ((x0, x1), x2): And<And<A, B>, C>
) -> And<A, And<B, C>> {
    (x0, (x1, x2))
}

/// `a ∧ (b ∨ c)  =>  (a ∧ b) ∨ (a ∧ c)`
pub fn distrib<A: Prop, B: Prop, C: Prop>(
    (a, x): And<A, Or<B, C>>
) -> Or<And<A, B>, And<A, C>> {
    use Either::*;

    match x {
        Left(b) => Left((a, b)),
        Right(c) => Right((a, c)),
    }
}

/// `(¬a ∧ ¬b) => ¬(a ∨ b)`.
pub fn to_de_morgan<A: Prop, B: Prop>(
    (f0, f1): And<Not<A>, Not<B>>
) -> Not<Or<A, B>> {
    use Either::*;

    match (A::decide(), B::decide()) {
        (Left(a), _) => match f0(a) {},
        (_, Left(b)) => match f1(b) {},
        (Right(a), Right(b)) => Rc::new(move |x| match x {
            Left(xa) => a.clone()(xa),
            Right(xb) => b.clone()(xb),
        })
    }
}

/// `¬(a ∨ b) => (¬a ∧ ¬b)`.
pub fn from_de_morgan<A: Prop, B: Prop>(
    f: Not<Or<A, B>>
) -> And<Not<A>, Not<B>> {
    use Either::*;

    match (A::decide(), B::decide()) {
        (Left(a), _) => match f(Left(a)) {},
        (_, Left(b)) => match f(Right(b)) {},
        (Right(not_a), Right(not_b)) => (not_a, not_b),
    }
}

/// `(true ∧ a) => a`.
pub fn false_arg<A: Prop>((x, _): And<False, A>) -> False {x}
