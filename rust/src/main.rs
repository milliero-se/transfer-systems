extern crate serde_json;
use bit_set::BitSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use ndarray::Array;

#[derive(Clone)]
struct Frame {
    next: usize,
    span: BitSet,
    gens: BitSet,
}
impl Frame {
    fn new(n: usize) -> Frame {
        let mut frame = Frame {
            next: n-1,
            span: BitSet::with_capacity(n),
            gens: BitSet::with_capacity(n),
        };
        frame.span.insert(n-1);

        frame
    }
}

fn main() {
    let group_name = if env::args().len() < 2 {
        print!("Group: ");
        io::stdout().flush().unwrap();
        let mut group_name = String::new();
        io::stdin().read_line(&mut group_name).unwrap();
        
        group_name.trim().to_owned()
    } else {
        env::args().nth(1).unwrap().trim().to_owned()
    };
    //let group_name = "QuaternionGroup()";

    let data = fs::read_to_string("data/".to_owned() + &group_name + ".txt").unwrap();
    let mut data = data.split('\n').into_iter();
    let n: usize = data.next().unwrap().parse().unwrap();

    let mat = data.next().unwrap();
    if data.next().is_some() { panic!() };

    let mat: Vec<Vec<Vec<usize>>> = serde_json::from_str(mat).unwrap();
    let mut mat = Array::from_shape_fn((n,n), |(i,j)| {
        let mut s = BitSet::with_capacity(n);
        for k in &mat[i][j] {
            s.insert(*k);
        }
        
        s
    });
    // At this point, mat[i,j] contains the bitset of meets of vertices i and j.

    // We now update the diagonal mat[i,j] to contain the n-ary meets of the vertex i (n >= 1).
    for i in 0..n {
        // mat[i,i] contains the meets of i with itself.
        // If this contains something other than i, we need to meet these with i as well.
        let mut new_meets = mat[[i,i]].clone();
        new_meets.remove(i);

        while !new_meets.is_empty() {
            // Meet the new_meets with i.
            let mut new_new_meets = BitSet::with_capacity(n);
            for o in &new_meets {
                new_new_meets.union_with(&mat[[i,o]]);
            }
            // Remove the ones we've seen before.
            new_new_meets.difference_with(&mat[[i,i]]);

            // Add these to mat[i,i].
            mat[[i,i]].union_with(&new_new_meets);
            // Continue looping to meet these with i.
            new_meets = new_new_meets;
        }
    }

    let mut stack = vec![Frame::new(n)];
    let mut out: usize = 0;

    while let Some(mut frame) = stack.pop() {
        for next in (0..=frame.next).rev() {
            if frame.span.contains(next) {
                continue;
            }
            // next is the next node to make a decision about.
            // First, decide no. 
            // If there's more work to do, push to stack. Else, update count.
            if next > 0 {
                let mut frame = frame.clone();
                frame.next = next-1;
                stack.push(frame);
            } else {
                out = out.checked_add(1).unwrap();
            }

            // Then, decide yes and keep processing
            frame.gens.insert(next);
            // Add in meets of i in span and j in the n-ary meets of next.
            for i in frame.span.clone().iter() {
                for j in &mat[[next,next]] {
                    frame.span.union_with(&mat[[i,j]]);
                }
            }
        }
        // No more decisions.
        out = out.checked_add(1).unwrap();
    }

    let mut out_file = File::create("out/".to_owned() + &group_name + ".txt").unwrap();
    write!(out_file, "{}\n", out).unwrap();

    println!("{}", out);
}
