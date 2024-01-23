mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 433
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        use std::rc::Rc;

        if start_gene == end_gene {
            return 0;
        }

        let mut table = HashMap::<Rc<String>, usize>::with_capacity(bank.len() + 2);

        let start_gene = Rc::new(start_gene);

        table.insert(start_gene.clone(), 0);

        for gene in bank {
            let idx = table.len();
            table.entry(Rc::new(gene)).or_insert(idx);
        }

        if !table.contains_key(&end_gene) {
            return -1;
        }

        let mut edges = vec![vec![]; table.len()];

        for (from_gene, &from_index) in table.iter() {
            for (to_gene, &to_index) in table.iter() {
                if from_gene == to_gene {
                    continue;
                }

                let diff_count = from_gene
                    .bytes()
                    .zip(to_gene.bytes())
                    .filter(|(a, b)| a != b)
                    .count();

                if diff_count == 1 {
                    edges[from_index].push(to_index);
                }
            }
        }

        // this vec holds the latest node dists
        let mut dists = vec![u8::MAX; table.len()];
        dists[table[&start_gene]] = 0u8;

        // heap for Dijkstra's algorithm
        // first item is distance, which constantly increase one by one
        // second item is the node index
        let mut heap = BinaryHeap::<(Reverse<u8>, u8)>::new();
        heap.extend((0..edges.len()).map(|s| (Reverse(dists[s]), s as u8)));

        while let Some((Reverse(from_mutations), gene_idx)) = heap.pop() {
            let mutations2 = dists[gene_idx as usize];

            if mutations2 == u8::MAX || mutations2 != from_mutations {
                // this node is too far aways or is outdated
                continue;
            }

            for &to_idx in edges[gene_idx as usize].iter() {
                let to_mutations = dists[to_idx];

                if from_mutations + 1 < to_mutations {
                    dists[to_idx] = from_mutations + 1;
                    heap.push((Reverse(from_mutations + 1), to_idx as u8));
                }
            }
        }

        let end_idx = table[&end_gene];
        if dists[end_idx] == u8::MAX {
            -1
        } else {
            dists[end_idx] as i32
        }
    }
}
