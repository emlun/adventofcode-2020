use std::collections::HashSet;

pub trait IntersectionAll<'a, T, I> {
    fn intersection_all(self, others: I) -> Self;
}

impl<'a, T, Iter, Set> IntersectionAll<'a, T, Iter> for HashSet<T>
where
    Iter: IntoIterator<Item = Set>,
    T: 'a,
    T: Eq,
    T: std::hash::Hash,
    Set: std::borrow::Borrow<HashSet<T>>,
{
    fn intersection_all(mut self, rest: Iter) -> Self {
        for r in rest.into_iter() {
            self.retain(|elem| r.borrow().contains(elem));
        }
        self
    }
}
