pub mod generate {
    use rand::Rng;

    pub fn execute(create_num_len: i32, rand_min: i32, rand_max: i32) -> Vec<i32> {
        let mut rand_array:Vec<i32> = Vec::new();
        let mut rng = rand::thread_rng();

        for _i in 0..create_num_len {
            let r = rand::thread_rng().gen_range(rand_min..rand_max);
            rand_array.push(r);
        }

        rand_array
    }
}
