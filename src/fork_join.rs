use rayon::prelude::*;

fn main() {

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let seq_exp_sum: i32 = data.iter() // <-- sequential iteration
                                .map(|x| x * x)
                                .sum();

    let par_exp_sum: i32 = data.par_iter() // <-- parallel iteration 
                                .map(|x| x * x)
                                .sum();

    assert_eq!(seq_exp_sum, par_exp_sum)
}
