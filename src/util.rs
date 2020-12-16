pub mod iter;
pub mod permutations;

pub fn sign(i: i64) -> i64 {
    if i == 0 {
        0
    } else {
        i / i.abs()
    }
}
