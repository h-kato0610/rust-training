fn main() {
    let mut primary_numbers: [i32; 100] = [0; 100];
    let mut primary_number_index = 0;

    for _i in 1..100 {
        if 2 == _i  || _i % 2 == 0 { continue };

        let mut is_divided: i8 = 0;
        for _j in 1.._i {
            if _i % _j == 0 && _i % 1 == 0 { is_divided += 1 }
        }

        if is_divided == 1 {
            primary_numbers[primary_number_index] = _i;
            primary_number_index += 1;
        }
    }

    for _i in 0..primary_numbers.len() {
        print!("{} ", primary_numbers[_i]);
    }
}
