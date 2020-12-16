use std::collections::HashSet;

pub trait IntersectionAll<'a, T, I> {
    fn intersection_all(self, others: I) -> Self;
}

impl<'a, T, I> IntersectionAll<'a, T, I> for HashSet<T>
where
    I: IntoIterator<Item = &'a HashSet<T>>,
    T: 'a,
    T: Eq,
    T: std::hash::Hash,
{
    fn intersection_all(mut self, rest: I) -> HashSet<T> {
        for r in rest.into_iter() {
            self.retain(|elem| r.contains(elem));
        }
        self
    }
}
