pub fn smallest_remainder(div_and_remains: Vec<(isize, isize)>) -> isize {
    let product: isize = div_and_remains.iter().map(|(divisor, _)| divisor).product();
    let one_solution: isize = div_and_remains
        .iter()
        .map(|&(divisor, remain)| elem(divisor, remain, product))
        .sum();
    modulo(one_solution, product)
}

pub fn modulo(solution: isize, product: isize) -> isize {
    ((solution % product) + product) % product
}

fn elem(divisor: isize, remain: isize, product: isize) -> isize {
    let prod = product / divisor;
    remain * bezout_coeff(divisor, prod) * prod
}

fn bezout_coeff(divisor: isize, prod: isize) -> isize {
    fn inner_coeff(
        r: isize,
        u: isize,
        v: isize,
        r_prime: isize,
        u_prime: isize,
        v_prime: isize,
    ) -> (isize, isize, isize) {
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
    inner_coeff(prod, 1, 0, divisor, 0, 1).1
}
