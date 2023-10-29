fn main() {
    // 89 -> 01010110
    // 27 -> 00011011
    //and    00010010

    let bit_and = 86 & 27;
    println!("{}", bit_and);

    let bit_or = 86 | 27;
    println!("{}", bit_or);

    let bit_xor = 86 ^ 27;
    println!("{}", bit_xor);

    let bit_shif_left = 86 << 1;
    println!("{}", bit_shif_left);

    let bit_shif_right = 86 >> 1;
    println!("{}", bit_shif_right);

 
}



main()
