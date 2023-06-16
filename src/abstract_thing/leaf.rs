use std::alloc::System;
use std::time::SystemTime;
use itertools::repeat_n;
use itertools::Itertools;
use chrono::{DateTime, Utc};
use rayon::prelude::*;

struct XianLiTi {}

impl XianLiTi {
    fn new() -> Self {
        st("线粒体初始化");
        XianLiTi {}
    }

    fn work(&self) {
        st("线粒体工作");
    }
}

struct Cell {
    compose:Vec<XianLiTi>,
}

impl Cell {
    fn new() -> Self {
        st("细胞初始化");
        Cell {
            compose: (0..2000)
                .map(|i| XianLiTi::new())
                .collect(),
        }
    }
}

struct Leaf {
    compose:Vec<Cell>,
}

impl Leaf {
    fn new() -> Self {
        st("叶子初始化");
        Leaf {
            compose: (0..2)
                .map(|i| Cell::new())
                .collect(),
        }
    }
}

struct Tree {
    compose:Vec<Leaf>,
}

impl Tree {
    fn new() -> Self {
        st("树初始化");
        Tree {
            compose: (0..2)
                .map(|i| Leaf::new())
                .collect(),
        }
    }
}

fn st(s:&str) {
    let now: DateTime<Utc> = Utc::now();
    println!("{} {}",s, now.format("%Y-%m-%d %H:%M:%S.%3f.%6f"));
}

#[test]
fn test_tree() {
    let tree = Tree::new();
    tree.compose.iter().for_each(|leaf| {
        leaf.compose.iter().for_each(|cell| {
            cell.compose.par_iter().for_each(|xian_li_ti| {
                xian_li_ti.work()
            });
        });
    });
}
