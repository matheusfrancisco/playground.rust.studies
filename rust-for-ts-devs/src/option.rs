fn multiply(num: Option<usize>) -> usize {
    // return num.unwrap_or(0) * 5;
    return num.unwrap_or(0) * 5;
}

fn multiply_1(num: Option<usize>) -> Option<usize> {
    return num.map(|x| x * 5);
}

fn multiply_2(num: Option<usize>) -> Option<usize> {
    let num = match num {
        Some(x) => x * 5,
        None => return None,
    };

    return Some(num);
}

fn multiply_3(num: Option<usize>) -> Option<usize> {
    return Some(num? * 5);
}

fn mult(num: Vec<usize>, index: usize) -> usize {
    return num.get(index).unwrap_or(&index) * 5;
}

fn main() {
    println!(" test");
}
