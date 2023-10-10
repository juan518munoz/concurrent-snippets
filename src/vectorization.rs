// IS THIS VECTORIZATION?
fn main() {
    // define for ourselves the amount of threads we want to use
    rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build_global()
        .unwrap();

    // creating a rayon scope allows us to spawn a fork-join scope
    let mut result = 0;
    rayon::scope(|s| {
        for x in 1..4 {
            // create a new scope
            s.spawn(move |t| {
                for y in 1..4 {
                    // create another scope, s won't end until all inner scopes are done
                    t.spawn(move |_| {
                        result += x * y;
                        println!("({},{}) result is now: {}", x, y, result);
                    });
                }
            })
        }
    });

    println!("result on main scope is: {}", result);
}
