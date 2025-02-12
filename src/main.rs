use std::ops::Shl;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
/// Exhaustive enum of the chess board squares.
enum Square {
    a8, b8, c8, d8, e8, f8, g8, h8,
    a7, b7, c7, d7, e7, f7, g7, h7,
    a6, b6, c6, d6, e6, f6, g6, h6,
    a5, b5, c5, d5, e5, f5, g5, h5,
    a4, b4, c4, d4, e4, f4, g4, h4,
    a3, b3, c3, d3, e3, f3, g3, h3,
    a2, b2, c2, d2, e2, f2, g2, h2,
    a1, b1, c1, d1, e1, f1, g1, h1,
}

fn set_bit(bitboard: &mut u64, square: Square) {
    *bitboard |= 1u64 << (square as i32);
}

fn get_bit(bitboard: u64, square: Square) -> u64 {
    bitboard & (1 << (square as i32))
}

fn print_bitboard(bitboard: u64) {
    println!();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            let mask = 1u64 << square;
            if file == 0 {
                print!(" {} ", 8 - rank);
            }
            print!(" {}", if bitboard & mask != 0 { '1' } else { '0' });
        }
        println!();
    }

    println!("    a b c d e f g h\n");
}

fn main() {
    let mut bb: u64 = 0u64;
    print_bitboard(bb);
    set_bit(&mut bb, Square::e4);
    print_bitboard(bb);
}
