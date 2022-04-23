pub fn modulo(n: i32, m: i32) -> i32 {
    i32::rem_euclid(n as i32, m as i32) as i32
}

pub fn dec_modulo(n: usize, m: usize) -> usize {
    modulo(n as i32 - 1, m as i32) as usize
}

pub fn inc_modulo(n: usize, m: usize) -> usize {
    modulo(n as i32 + 1, m as i32) as usize
}
