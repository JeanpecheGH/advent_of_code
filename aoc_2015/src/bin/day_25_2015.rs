fn main() {
    let row: u64 = 2978;
    let column: u64 = 3083;

    let rank = code_rank(row, column);

    let code_1: u64 = 20151125;
    //We already have code n°1, so we start at 2
    let code_n: u64 = (2..=rank).fold(code_1, |acc, _| mult_and_rest(acc));

    println!("The {}th valid code is {}", rank, code_n);
}

fn code_rank(r: u64, c: u64) -> u64 {
    //First, we compute the last number of the target diagonal (so the number in row 1, column (c+r-1))
    //This number has the sum of all numbers from 1 to (c+r-1)
    //Then we substract the number of rows minus 1 to finish on our target number (because we already start from row 1)
    let n = c + r - 1;
    ((n + 1) * n) / 2 - r + 1
}

fn mult_and_rest(code: u64) -> u64 {
    (code * 252533) % 33554393
}
