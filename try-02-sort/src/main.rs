mod create_rand_num;
mod sort;

pub use crate::create_rand_num::generate;
pub use crate::sort::quick_sort;

use std::env;

fn main() {
    // Args Process
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("\nInput Your Arg to = {} [app <create num len> <RAND_MIN> <RAND_MAX>\n", args.len());
    }
    let create_num_len: i32 = args[1].trim().parse::<i32>().expect("[Input your Integer]");
    let rand_min: i32 = args[2].trim().parse::<i32>().expect("[Input your Integer]");
    let rand_max: i32 = args[3].trim().parse::<i32>().expect("[Input your Integer]");
    
    // Get Result
    let random_numbers = generate::execute(create_num_len, rand_min, rand_max);

    let r = sort::quick_sort::get_sorted_vector(random_numbers);

    print!("_-----------------");
    print!("{}", r);
}
