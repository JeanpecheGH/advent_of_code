pub fn smallest_remainder(div_and_remains: Vec<(i128, i128)>) -> i128 {
    let product: i128 = div_and_remains.iter().map(|(divisor, _)| divisor).product();
    let one_solution: i128 = div_and_remains
        .iter()
        .map(|&(divisor, remain)| elem(divisor, remain, product))
        .sum();
    modulo(one_solution, product)
}

pub fn modulo(solution: i128, product: i128) -> i128 {
    ((solution % product) + product) % product
}

fn elem(divisor: i128, remain: i128, product: i128) -> i128 {
    let prod = product / divisor;
    remain * bezout_coeff(divisor, prod) * prod
}

fn bezout_coeff(divisor: i128, prod: i128) -> i128 {
    inner_coeff(prod, 1, 0, divisor, 0, 1).1
}

pub fn bezout_triplet(a: u128, b: u128) -> (i128, i128, i128) {
    inner_coeff(a as i128, 1, 0, b as i128, 0, 1)
}

fn inner_coeff(
    r: i128,
    u: i128,
    v: i128,
    r_prime: i128,
    u_prime: i128,
    v_prime: i128,
) -> (i128, i128, i128) {
    if r_prime == 0 {
        (r, u, v)
    } else {
        let q = r / r_prime;
        inner_coeff(
            r_prime,
            u_prime,
            v_prime,
            r - q * r_prime,
            u - q * u_prime,
            v - q * v_prime,
        )
    }
}
