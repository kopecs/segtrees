// @file segtree.rs
// @author Danny Sleator, translated by Abi Kim (abigalek)
// @brief Segment tree implementation in Rust.
// Implements the segtree described in the lectures notes.

// Module that you use to access the segment tree data structure.
mod segtree_mod {
    pub struct SegmentTree {
        n: usize,
        arr: Vec<i32>,
        identity: i32
    }

    fn super_ceiling(nn: usize) -> usize {
        let mut p = 1;
        while p < nn {
            p = 2 * p;
        }
        return p;
    }

    impl SegmentTree {
        // Call new from this module to construct a segment tree.
        pub fn new(size: usize) -> SegmentTree {
            return SegmentTree {
                n: super_ceiling(size),
                arr: vec![0; 2 * super_ceiling(size)],
                identity: 0
            };
        }

        // Implements Assign in the lecture notes.
        // Assigns value x to index i in the array A, and also recomputes
        // every other value in the path up to the root from index i by 
        // adding together the values of the two children below it.
        pub fn assign(&mut self, i: usize, x: i32) {
            let mut i_prime = i + self.n;
            self.arr[i_prime] = x;
            i_prime = SegmentTree::parent(i_prime);
            while i_prime > 0 {
                self.arr[i_prime] = SegmentTree::glue (self.arr[SegmentTree::left_child(i_prime)], self.arr[SegmentTree::right_child(i_prime)]);
                i_prime = SegmentTree::parent(i_prime);
            }
        }

        // Finds sum_{i <= k <= j} A[k]
        // More information in function f's documentation.
        pub fn range_sum(&self, i: usize, j: usize) -> i32 {
            return self.f (1, 0, self.n - 1, i, j);
        }

        // Prints the contents of the array arrt
        pub fn print_a(&self) {
            for i in 0..self.arr.len() {
                println!("A[{}] = {}", i, self.arr[i]);
            }
        }

        fn parent(i: usize) -> usize { return i/2; }

        fn left_child(i: usize) -> usize { return 2 * i; }

        fn right_child(i: usize) -> usize { return (2 * i) + 1; }

        fn glue (a: i32, b: i32) -> i32 { return a + b; }

        fn f (&self, v: usize, l: usize, r: usize, i: usize, j: usize) -> i32 {
        // We’re currently at A[v]. 1 <= v < 2*N.
        // The range [l,r] is that of the current block, wrt user variables [0,n-1].
        // The range [i,j] is the range of the query, wrt user variables [0,n-1].
        // The size of the range [l,r] = r-l+1 is a power of 2.
        // The range [l,r] contains the range [i,j].
        // This function returns the answer to the query.

            if l == i && r == j {
                return self.arr[v];
            } else {
                let m = (l+r)/2; // split [l,r] into [l,m] [m+1,r]
                let t1 = if i <= m { self.f(SegmentTree::left_child(v), l, m, i, std::cmp::min(m,j)) } else { self.identity };
                let t2 = if j > m { self.f(SegmentTree::right_child(v), m+1, r, std::cmp::max(i,m+1), j) } else { self.identity };
                return SegmentTree::glue (t1, t2);
            }
        }
    }
}

fn main() {
    let mut s = segtree_mod::SegmentTree::new(7);
    s.assign(3,7);
    s.assign(4,1);
    s.print_a();

    println!("range_sum(2, 7) = {}", s.range_sum(2, 7));
    println!("range_sum(0, 3) = {}", s.range_sum(0, 3));
    println!("range_sum(4, 5) = {}", s.range_sum(4, 5));
    println!("range_sum(5, 5) = {}", s.range_sum(5, 5));
}