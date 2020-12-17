use crate::common::Solution;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[allow(unused)]
fn print_state(state: &State) {
    for x in state.dimensions.coords() {
        let s = state
            .dimensions
            .get(x)
            .unwrap()
            .coords()
            .map(|y| {
                state
                    .dimensions
                    .get(x)
                    .unwrap()
                    .get(y)
                    .unwrap()
                    .coords()
                    .map(|z| match state.get(x, y, z).unwrap() {
                        false => '.',
                        true => '#',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("x = {}:\n{}", x, s);
    }
}

#[derive(Clone)]
struct MiddleVec<T> {
    value: Vec<T>,
}

impl<T> MiddleVec<T>
where
    T: Clone,
    T: Default,
{
    fn of(v: T) -> Self {
        MiddleVec { value: vec![v] }
    }

    fn get(&self, coord: isize) -> Option<&T> {
        let i = Self::coord_to_index(coord);
        self.value.get(i)
    }

    fn get_mut(&mut self, coord: isize) -> &mut T {
        let i = Self::coord_to_index(coord);
        if i >= self.value.len() {
            self.value.resize(i + 1, T::default());
        }
        self.value.get_mut(i).unwrap()
    }

    fn coord_to_index(coord: isize) -> usize {
        coord.abs() as usize * 2 - ((coord < 0) as usize)
    }

    fn index_to_coord(index: usize) -> isize {
        if index % 2 == 0 {
            index as isize / 2
        } else {
            -(index as isize / 2) - 1
        }
    }

    fn coords(&self) -> RangeInclusive<isize> {
        let l = self.value.len();
        let r = if l > 0 {
            let c1 = Self::index_to_coord(l - 1);
            let c2 = if l > 1 {
                Self::index_to_coord(l - 2)
            } else {
                0
            };
            // dbg!(c1, c2);
            std::cmp::min(c1, c2)..=std::cmp::max(c1, c2)
        } else {
            0..=-1
        };
        // dbg!(l, &r);
        r
    }

    fn coords_expanded(&self) -> RangeInclusive<isize> {
        let l = self.value.len() + 2;
        let r = if l > 0 {
            let c1 = Self::index_to_coord(l - 1);
            let c2 = if l > 1 {
                Self::index_to_coord(l - 2)
            } else {
                0
            };
            // dbg!(c1, c2);
            std::cmp::min(c1, c2)..=std::cmp::max(c1, c2)
        } else {
            0..=-1
        };
        // dbg!(l, &r);
        r
    }
}

impl<T> Default for MiddleVec<T> {
    fn default() -> Self {
        Self { value: vec![] }
    }
}

impl<T> std::iter::FromIterator<T> for MiddleVec<T>
where
    T: Clone,
    T: Default,
{
    fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut slf = MiddleVec::default();
        for (i, item) in it.into_iter().enumerate() {
            *slf.get_mut(i as isize) = item;
        }
        slf
    }
}

#[derive(Clone)]
struct State {
    pub dimensions: MiddleVec<MiddleVec<MiddleVec<bool>>>,
}

impl State {
    fn new(initial_plane: MiddleVec<MiddleVec<bool>>) -> Self {
        State {
            dimensions: MiddleVec::of(initial_plane),
        }
    }

    fn get(&self, x: isize, y: isize, z: isize) -> Option<&bool> {
        self.dimensions
            .get(x)
            .and_then(|x| x.get(y))
            .and_then(|y| y.get(z))
    }

    fn get_mut(&mut self, x: isize, y: isize, z: isize) -> &mut bool {
        self.dimensions.get_mut(x).get_mut(y).get_mut(z)
    }

    fn coords(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let x = self.dimensions.coords();
        let ymin = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.coords().min())
            .min()
            .unwrap_or(0);
        let ymax = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.coords().max())
            .max()
            .unwrap_or(0);
        let zmin = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.value.iter().flat_map(|zs| zs.coords().min()))
            .min()
            .unwrap_or(0);
        let zmax = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.value.iter().flat_map(|zs| zs.coords().max()))
            .max()
            .unwrap_or(0);
        (x, ymin..=ymax, zmin..=zmax)
    }

    fn coords_expanded(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let (x, y, z) = self.coords();
        (
            x.clone().min().unwrap() - 1..=x.max().unwrap() + 1,
            y.clone().min().unwrap() - 1..=y.max().unwrap() + 1,
            z.clone().min().unwrap() - 1..=z.max().unwrap() + 1,
        )
    }
}

#[derive(Clone)]
struct State4 {
    pub dimensions: MiddleVec<MiddleVec<MiddleVec<MiddleVec<bool>>>>,
}

impl State4 {
    fn new(initial_plane: MiddleVec<MiddleVec<bool>>) -> Self {
        Self {
            dimensions: MiddleVec::of(MiddleVec::of(initial_plane)),
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

    fn coords(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let x = self.dimensions.coords();
        let ymin = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.coords().min())
            .min()
            .unwrap_or(0);
        let ymax = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.coords().max())
            .max()
            .unwrap_or(0);
        let zmin = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.value.iter().flat_map(|zs| zs.coords().min()))
            .min()
            .unwrap_or(0);
        let zmax = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| ys.value.iter().flat_map(|zs| zs.coords().max()))
            .max()
            .unwrap_or(0);
        let wmin = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| {
                ys.value
                    .iter()
                    .flat_map(|zs| zs.value.iter().flat_map(|ws| ws.coords().min()))
            })
            .min()
            .unwrap_or(0);
        let wmax = self
            .dimensions
            .value
            .iter()
            .flat_map(|ys| {
                ys.value
                    .iter()
                    .flat_map(|zs| zs.value.iter().flat_map(|ws| ws.coords().max()))
            })
            .max()
            .unwrap_or(0);
        (x, ymin..=ymax, zmin..=zmax, wmin..=wmax)
    }

    fn coords_expanded(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let (x, y, z, w) = self.coords();
        (
            x.clone().min().unwrap() - 1..=x.max().unwrap() + 1,
            y.clone().min().unwrap() - 1..=y.max().unwrap() + 1,
            z.clone().min().unwrap() - 1..=z.max().unwrap() + 1,
            w.clone().min().unwrap() - 1..=w.max().unwrap() + 1,
        )
    }
}

fn simulate(state: State, dxyz: &[(isize, isize, isize)], steps: usize) -> usize {
    let mut buf1 = state.clone();
    let mut buf2 = state;

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    for t in 0..steps {
        // dbg!(t);
        // print_state(&current);

        let (xs, ys, zs) = current.coords_expanded();
        for x in xs {
            for y in ys.clone() {
                for z in zs.clone() {
                    let num_neighbors = dxyz
                        .iter()
                        .map(|(dx, dy, dz)| current.get(x + dx, y + dy, z + dz))
                        .filter(|tile| *tile.unwrap_or(&false))
                        .count();

                    *next.get_mut(x, y, z) = if *current.get(x, y, z).unwrap_or(&false) {
                        if num_neighbors == 2 || num_neighbors == 3 {
                            true
                        } else {
                            false
                        }
                    } else {
                        if num_neighbors == 3 {
                            true
                        } else {
                            false
                        }
                    };
                }
            }
        }

        std::mem::swap(&mut current, &mut next);
    }

    current
        .dimensions
        .value
        .iter()
        .flat_map(|dim| dim.value.iter())
        .flat_map(|dim| dim.value.iter())
        .filter(|tile| **tile)
        .count()
}

fn simulate4(state: State4, dxyzw: &[(isize, isize, isize, isize)], steps: usize) -> usize {
    let mut buf1 = state.clone();
    let mut buf2 = state;

    let mut current = &mut buf1;
    let mut next = &mut buf2;

    for t in 0..steps {
        // dbg!(t);
        // print_state(&current);

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

                        *next.get_mut(x, y, z, w) = if *current.get(x, y, z, w).unwrap_or(&false) {
                            if num_neighbors == 2 || num_neighbors == 3 {
                                true
                            } else {
                                false
                            }
                        } else {
                            if num_neighbors == 3 {
                                true
                            } else {
                                false
                            }
                        };
                    }
                }
            }
        }

        std::mem::swap(&mut current, &mut next);
    }

    current
        .dimensions
        .value
        .iter()
        .flat_map(|dim| dim.value.iter())
        .flat_map(|dim| dim.value.iter())
        .flat_map(|dim| dim.value.iter())
        .filter(|tile| **tile)
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: State = State::new(
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
                    .collect::<MiddleVec<bool>>()
            })
            .collect::<MiddleVec<MiddleVec<bool>>>(),
    );

    // print_state(&map);

    let dxyz: Vec<(isize, isize, isize)> = ((-1)..=1)
        .flat_map(|dx| ((-1)..=1).map(move |dy| (dx, dy)))
        .flat_map(|(dx, dy)| ((-1)..=1).map(move |dz| (dx, dy, dz)))
        .filter(|(dx, dy, dz)| (*dx, *dy, *dz) != (0, 0, 0))
        .collect();

    let dxyzw: Vec<(isize, isize, isize, isize)> = ((-1)..=1)
        .flat_map(|dx| ((-1)..=1).map(move |dy| (dx, dy)))
        .flat_map(|(dx, dy)| ((-1)..=1).map(move |dz| (dx, dy, dz)))
        .flat_map(|(dx, dy, dz)| ((-1)..=1).map(move |dw| (dx, dy, dz, dw)))
        .filter(|(dx, dy, dz, dw)| (*dx, *dy, *dz, *dw) != (0, 0, 0, 0))
        .collect();

    let state_b = State4::new(map.dimensions.value[0].clone());
    (
        simulate(map, &dxyz, 6).to_string(),
        simulate4(state_b, &dxyzw, 6).to_string(),
    )
}
