#![feature(proc_macro)]

extern crate accel;
extern crate accel_derive;

use accel_derive::kernel;  // #[macro_use]は使わない
use accel::*;

#[kernel]
#[depends("accel-core" = "0.1")]  // これでCargo.tomlとextern crateに追加
pub unsafe fn add(a: *const f64, b: *const f64, c: *mut f64, n: usize) {
    let i = accel_core::index(); // threadId.x等をラップしたもの
    if (i as usize) < n {
        *c.offset(i) = *a.offset(i) + *b.offset(i);
        // この辺はまだ未完成(- -;)
    }
}

fn main() {
    let n = 8; // debug用に少なく
    // Unified Memory版Vecを用意(0-fill)
    let mut a = UVec::new(n).unwrap();
    let mut b = UVec::new(n).unwrap();
    let mut c = UVec::new(n).unwrap();

    // CPU側で初期化
    for i in 0..n {
        a[i] = i as f64;
        b[i] = 2.0 * i as f64;
    }
    println!("a = {:?}", a.as_slice());
    println!("b = {:?}", b.as_slice());

    let grid = Grid::x(64);
    let block = Block::x(64);
    // CPU -> GPUに転送
    add(grid, block, a.as_ptr(), b.as_ptr(), c.as_mut_ptr(), n);

    device::sync().unwrap(); // 実行を待つ
    // GPU -> CPUに転送
    println!("c = {:?}", c.as_slice());
}