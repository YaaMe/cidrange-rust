use std::collections::HashMap;
use std::collections::HashSet;

pub type DIPTree = HashMap<u32, HashSet<(u8, u8)>>;
pub type CIPTree = HashMap<u16, HashSet<(u8, u8)>>;
pub type BIPTree = HashMap<u8, HashSet<(u8, u8)>>;
pub type AIPTree = HashSet<(u8, u8)>;
pub struct IPChecker {
    pub dtree: DIPTree,
    pub ctree: CIPTree,
    pub btree: BIPTree,
    pub atree: AIPTree,
}
impl IPChecker {
    pub fn check(&self, a: u8, b: u8, c: u8, d: u8) -> bool {
        let key: u32 = (a as u32) << 16 | (b as u32) << 8 | (c as u32);
        if self.dtree.contains_key(&key) {
            return check(d, self.dtree.get(&key).unwrap());
        }
        let key: u16 = (a as u16) << 8 | (b as u16);
        if self.ctree.contains_key(&key) {
            return check(c, self.ctree.get(&key).unwrap());
        }
        if self.btree.contains_key(&a) {
            return check(b, self.btree.get(&a).unwrap());
        }
        if !self.atree.is_empty() {
            return check(a, &self.atree);
        }
        return false;
    }
}
#[macro_export]
macro_rules! build_net {
    ($($cidr:expr),*) => {{
        use rust_cidrange::utils::{DIPTree, CIPTree, BIPTree, AIPTree, IPChecker};
        use std::collections::HashSet;
        let mut d_tree = DIPTree::new();
        let mut c_tree = CIPTree::new();
        let mut b_tree = BIPTree::new();
        let mut a_tree = AIPTree::new();
        $(
            let mut iter = $cidr.splitn(2, "/");
            let ip = iter.next().unwrap();
            let x: u8 = iter.next().unwrap().parse().unwrap();
            let mut ip_iter = ip.splitn(4, ".");
            let a: u8 = ip_iter.next().unwrap().parse().unwrap();
            let b: u8 = ip_iter.next().unwrap().parse().unwrap();
            let c: u8 = ip_iter.next().unwrap().parse().unwrap();
            let d: u8 = ip_iter.next().unwrap().parse().unwrap();
            match x {
                24..=32 => {
                    d_tree.entry((a as u32) << 16 | (b as u32) << 8 | (c as u32))
                        .or_insert(HashSet::new())
                        .insert((d >> (32-x), x-24));
                }
                16..=23 => {
                    c_tree.entry((a as u16) << 8 | (b as u16))
                        .or_insert(HashSet::new())
                        .insert((c >> (24-x), x-16));
                },
                8..=15 => {
                    b_tree.entry(a)
                        .or_insert(HashSet::new())
                        .insert((b >> (16-x), x-8));
                },
                _ => {
                    a_tree.insert((a >> (8-x), x));
                },
            };
        )*;
        IPChecker {
            dtree: d_tree,
            ctree: c_tree,
            btree: b_tree,
            atree: a_tree,
        }
    }}
}

pub fn check(x: u8, r: &HashSet<(u8, u8)>) -> bool {
    if r.contains(&(0, 0)) {
        return true;
    }
    let mut i = 8;
    while i > 0 {
        if r.contains(&(x >> (8 - i), i)) {
            return true;
        }
        i -= 1;
    }
    return false
}
