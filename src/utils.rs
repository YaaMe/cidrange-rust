use std::collections::HashMap;
use std::collections::HashSet;

pub type DIPTree = HashMap<u32, HashSet<(u8, u8)>>;
pub type CIPTree = HashMap<u16, HashSet<(u8, u8)>>;
pub type BIPTree = HashMap<u8, HashSet<(u8, u8)>>;
pub type AIPTree = HashSet<(u8, u8)>;

#[macro_export]
macro_rules! build_net {
    ($($y:expr,$a:expr,$b:expr,$c:expr,$d:expr,$($t:expr,$x:expr),*);*) => {
        use std::collections::HashSet;
        pub fn gen_tree() -> (DIPTree, CIPTree, BIPTree, AIPTree) {
            let mut d_tree = DIPTree::new();
            let mut c_tree = CIPTree::new();
            let mut b_tree = BIPTree::new();
            let mut a_tree = AIPTree::new();
            $(
                let mut set = HashSet::new();
                $(set.insert(($t, $x));)*
                match $y {
                    3 => {d_tree.insert(($a as u32) << 16 | ($b as u32) << 8 | ($c as u32), set);},
                    2 => {c_tree.insert(($a as u16) << 8 | ($b as u16), set);},
                    1 => {b_tree.insert($a, set);},
                    - => {a_tree = set;},
                    _ => (),
                };
            )*
            return (d_tree, c_tree, b_tree, a_tree);
        }
    };
}

pub fn check(x: u8, r: &HashSet<u8, u8>) -> bool {
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
