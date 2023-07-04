//! Collection of "perms" (algs for permutating pieces) for NxNxN cubes

use once_cell::sync::Lazy;

use crate::generic::alg::Alg;

use super::AxisMove;

/// R U R' U'. Yeee
pub static SEXY_MOVE: Lazy<Alg<AxisMove>> = Lazy::new(|| "R U R' U'".try_into().unwrap());

/// The [T perm](http://algdb.net/puzzle/333/pll/t). Swaps RUF-RUB and RU-LU
pub static T: Lazy<Alg<AxisMove>> =
    Lazy::new(|| "R U R' U' R' F R2 U' R' U' R U R' F'".try_into().unwrap());

/// The [U perm](http://algdb.net/puzzle/333/pll/ua) (specifically Ua). Cycles RUF->RUB->RUL
pub static U: Lazy<Alg<AxisMove>> = Lazy::new(|| "R2 U' R' U' R U R U R U' R".try_into().unwrap());

/// The [J perm](http://algdb.net/puzzle/333/pll/jb) (specifically Jb). Swaps RU-UF and RUF-RUB
pub static J: Lazy<Alg<AxisMove>> =
    Lazy::new(|| "R U R' F' R U R' U' R' F R2 U' R' U'".try_into().unwrap());
