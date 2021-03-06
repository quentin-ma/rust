// ignore-tidy-linelength
// ignore-windows failing on win32 bot
// ignore-freebsd: gdb package too new
// ignore-android: FIXME(#10381)
// compile-flags:-g

// The pretty printers being tested here require the patch from
// https://sourceware.org/bugzilla/show_bug.cgi?id=21763
// min-gdb-version 8.1

// min-lldb-version: 310

// === GDB TESTS ===================================================================================

// gdb-command: run

// gdb-command: print btree_set
// gdb-check:$1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}

// gdb-command: print empty_btree_set
// gdb-check:$2 = BTreeSet<i32>(len: 0)

// gdb-command: print btree_map
// gdb-check:$3 = BTreeMap<i32, i32>(len: 15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}

// gdb-command: print empty_btree_map
// gdb-check:$4 = BTreeMap<i32, u32>(len: 0)

// gdb-command: print nasty_btree_map
// gdb-check:$5 = BTreeMap<i32, pretty_std_collections::MyLeafNode>(len: 1) = {[1] = pretty_std_collections::MyLeafNode (11)}

// gdb-command: print vec_deque
// gdb-check:$6 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}

// gdb-command: print vec_deque2
// gdb-check:$7 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}

#![allow(unused_variables)]
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

struct MyLeafNode(i32); // helps to ensure we don't blindly replace substring "LeafNode"

fn main() {
    // BTreeSet
    let mut btree_set = BTreeSet::new();
    for i in 0..15 {
        btree_set.insert(i);
    }

    let mut empty_btree_set: BTreeSet<i32> = BTreeSet::new();

    // BTreeMap
    let mut btree_map = BTreeMap::new();
    for i in 0..15 {
        btree_map.insert(i, i);
    }

    let mut empty_btree_map: BTreeMap<i32, u32> = BTreeMap::new();

    let mut nasty_btree_map: BTreeMap<i32, MyLeafNode> = BTreeMap::new();
    nasty_btree_map.insert(1, MyLeafNode(11));

    // VecDeque
    let mut vec_deque = VecDeque::new();
    vec_deque.push_back(5);
    vec_deque.push_back(3);
    vec_deque.push_back(7);

    // VecDeque where an element was popped.
    let mut vec_deque2 = VecDeque::new();
    for i in 1..8 {
        vec_deque2.push_back(i)
    }
    vec_deque2.pop_front();
    vec_deque2.push_back(8);

    zzz(); // #break
}

fn zzz() {
    ()
}
