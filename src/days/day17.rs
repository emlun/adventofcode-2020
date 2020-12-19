use crate::common::Solution;
use std::convert::TryFrom;
use std::ops::Range;
use std::ops::RangeInclusive;

#[derive(Clone)]
struct DoubleVec<T> {
    pos: Vec<T>,
    neg: Vec<T>,
}

impl<T> DoubleVec<T>
where
    T: Clone,
    T: Default,
{
    fn of(v: T) -> Self {
        DoubleVec {
            pos: vec![v],
            neg: vec![],
        }
    }

    fn get(&self, coord: isize) -> Option<&T> {
        if coord >= 0 {
            self.pos.get(usize::try_from(coord).unwrap())
        } else {
            self.neg.get(usize::try_from(coord.abs() - 1).unwrap())
        }
    }

    fn get_mut(&mut self, coord: isize) -> &mut T {
        if coord >= 0 {
            let i = usize::try_from(coord).unwrap();
            if i >= self.pos.len() {
                self.pos.resize(i + 1, T::default());
            }
            self.pos.get_mut(i).unwrap()
        } else {
            let j = usize::try_from(coord.abs() - 1).unwrap();
            if j >= self.neg.len() {
                self.neg.resize(j + 1, T::default());
            }
            self.neg.get_mut(j).unwrap()
        }
    }

    fn coords_expanded(&self) -> Range<isize> {
        (-isize::try_from(self.neg.len()).unwrap() - 1)
            ..(isize::try_from(self.pos.len()).unwrap() + 1)
    }

    fn iter(&self) -> std::iter::Chain<std::slice::Iter<'_, T>, std::slice::Iter<'_, T>> {
        self.pos.iter().chain(self.neg.iter())
    }
}

impl<T> Default for DoubleVec<T> {
    fn default() -> Self {
        Self {
            pos: vec![],
            neg: vec![],
        }
    }
}

impl<T> std::iter::FromIterator<T> for DoubleVec<T>
where
    T: Clone,
    T: Default,
{
    fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut slf = DoubleVec::default();
        for (i, item) in it.into_iter().enumerate() {
            *slf.get_mut(i as isize) = item;
        }
        slf
    }
}

#[derive(Clone)]
struct State {
    pub dimensions: DoubleVec<DoubleVec<DoubleVec<DoubleVec<bool>>>>,
}

impl State {
    fn new(initial_plane: DoubleVec<DoubleVec<bool>>) -> Self {
        Self {
            dimensions: DoubleVec::of(DoubleVec::of(initial_plane)),
        }
    }

    fn get(&self, x: isize, y: isize, z: isize, w: isize) -> Option<&bool> {
        self.dimensions
            .get(x)
            .and_then(|x| x.get(y))
            .and_then(|y| y.get(z))
            .and_then(|z| z.get(w))
    }

    fn get_mut(&mut self, x: isize, y: isize, z: isize, w: isize) -> &mut bool {
        self.dimensions.get_mut(x).get_mut(y).get_mut(z).get_mut(w)
    }

    fn coords_expanded(
        &self,
    ) -> (
        Range<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let x = self.dimensions.coords_expanded();
        let ymin = self
            .dimensions
            .iter()
            .flat_map(|ys| ys.coords_expanded().min())
            .min()
            .unwrap_or(0);
        let ymax = self
            .dimensions
            .iter()
            .flat_map(|ys| ys.coords_expanded().max())
            .max()
            .unwrap_or(0);
        let zmin = self
            .dimensions
            .iter()
            .flat_map(|ys| ys.iter().flat_map(|zs| zs.coords_expanded().min()))
            .min()
            .unwrap_or(0);
        let zmax = self
            .dimensions
            .iter()
            .flat_map(|ys| ys.iter().flat_map(|zs| zs.coords_expanded().max()))
            .max()
            .unwrap_or(0);
        let wmin = self
            .dimensions
            .iter()
            .flat_map(|ys| {
                ys.iter()
                    .flat_map(|zs| zs.iter().flat_map(|ws| ws.coords_expanded().min()))
            })
            .min()
            .unwrap_or(0);
        let wmax = self
            .dimensions
            .iter()
            .flat_map(|ys| {
                ys.iter()
                    .flat_map(|zs| zs.iter().flat_map(|ws| ws.coords_expanded().max()))
            })
            .max()
            .unwrap_or(0);
        (x, ymin..=ymax, zmin..=zmax, wmin..=wmax)
    }
}

fn simulate(state: State, dxyzw: &[(isize, isize, isize, isize)], steps: usize) -> usize {
    let mut buf1 = state.clone();
    let mut buf2 = state;

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    for _ in 0..steps {
        let (xs, ys, zs, ws) = current.coords_expanded();

        for x in xs {
            for y in ys.clone() {
                for z in zs.clone() {
                    for w in ws.clone() {
                        let num_neighbors = dxyzw
                            .iter()
                            .map(|(dx, dy, dz, dw)| current.get(x + dx, y + dy, z + dz, w + dw))
                            .filter(|tile| *tile.unwrap_or(&false))
                            .count();

                        let next_value = if *current.get(x, y, z, w).unwrap_or(&false) {
                            num_neighbors == 2 || num_neighbors == 3
                        } else {
                            num_neighbors == 3
                        };

                        let n = next.get(x, y, z, w);
                        if (next_value && n != Some(&true)) || !next_value && n == Some(&true) {
                            *next.get_mut(x, y, z, w) = next_value;
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut current, &mut next);
    }

    current
        .dimensions
        .iter()
        .flat_map(|dim| dim.iter())
        .flat_map(|dim| dim.iter())
        .flat_map(|dim| dim.iter())
        .filter(|tile| **tile)
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let initial_state: State = State::new(
        lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect::<DoubleVec<bool>>()
            })
            .collect::<DoubleVec<DoubleVec<bool>>>(),
    );

    let dxyz: Vec<(isize, isize, isize, isize)> = ((-1)..=1)
        .flat_map(|dy| ((-1)..=1).map(move |dz| (dy, dz)))
        .flat_map(|(dy, dz)| ((-1)..=1).map(move |dw| (0, dy, dz, dw)))
        .filter(|(dx, dy, dz, dw)| (*dx, *dy, *dz, *dw) != (0, 0, 0, 0))
        .collect();

    let dxyzw: Vec<(isize, isize, isize, isize)> = ((-1)..=1)
        .flat_map(|dx| ((-1)..=1).map(move |dy| (dx, dy)))
        .flat_map(|(dx, dy)| ((-1)..=1).map(move |dz| (dx, dy, dz)))
        .flat_map(|(dx, dy, dz)| ((-1)..=1).map(move |dw| (dx, dy, dz, dw)))
        .filter(|(dx, dy, dz, dw)| (*dx, *dy, *dz, *dw) != (0, 0, 0, 0))
        .collect();

    (
        simulate(initial_state.clone(), &dxyz, 6).to_string(),
        simulate(initial_state, &dxyzw, 6).to_string(),
    )
}
