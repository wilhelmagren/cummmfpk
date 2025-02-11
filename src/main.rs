///
/// bitboards:
/// 
/// white pawns:                    black pawns:
/// 
///  0xFF << 48                      0xFF << 8
/// 
///  0 0 0 0 0 0 0 0                0 0 0 0 0 0 0 0
///  0 0 0 0 0 0 0 0                1 1 1 1 1 1 1 1
///  0 0 0 0 0 0 0 0                0 0 0 0 0 0 0 0
///  0 0 0 0 0 0 0 0                0 0 0 0 0 0 0 0
///  0 0 0 0 0 0 0 0                0 0 0 0 0 0 0 0
///  0 0 0 0 0 0 0 0                0 0 0 0 0 0 0 0
///  1 1 1 1 1 1 1 1                0 0 0 0 0 0 0 0
///  0 0 0 0 0 0 0 0                0 0 0 0 0 0 0 0

static BB_VOID: u64 = 0x0000000000000000;
static BB_ALL: u64 = 0xFFFFFFFFFFFFFFFF;

static BB_WHITE_PAWNS: u64 = 0xFF << 48;
static BB_WHITE_KNIGHTS: u64 = 0x42 << 56;
static BB_WHITE_BISHOPS: u64 = 0x24 << 56;
static BB_WHITE_ROOKS: u64 = 0x81 << 56;
static BB_WHITE_QUEEN: u64 = 0x10 << 56;
static BB_WHITE_KING: u64 = 0x08 << 56;

static BB_BLACK_PAWNS: u64 = 0xFF << 8;
static BB_BLACK_KNIGHTS: u64 = 0x42;
static BB_BLACK_BISHOPS: u64 = 0x24;
static BB_BLACK_ROOKS: u64 = 0x81;
static BB_BLACK_QUEEN: u64 = 0x10;
static BB_BLACK_KING: u64 = 0x08;

fn print_bitboard(bb: u64) {
    for i in 0..8 {
        for j in 0..8 {
            let shift = i * 8 + j;
            let mask = 1 << shift;
            if (bb & mask) != 0 {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}

fn main() {
    println!("cengine");
    println!("BB_VOID:\t{:#064b}", BB_VOID);
    println!("BB_ALL:\t\t{:#064b}", BB_ALL);

    println!("BB_BLACK_PAWNS:\t\t{:#064b}", BB_BLACK_PAWNS);
    println!("BB_BLACK_KNIGHTS:\t{:#064b}", BB_BLACK_KNIGHTS);
    println!("BB_BLACK_BISHOPS:\t{:#064b}", BB_BLACK_BISHOPS);
    println!("BB_BLACK_ROOKS:\t\t{:#064b}", BB_BLACK_ROOKS);
    println!("BB_BLACK_QUEEN:\t\t{:#064b}", BB_BLACK_QUEEN);
    println!("BB_BLACK_KING:\t\t{:#064b}", BB_BLACK_KING);

    println!("BB_WHITE_PAWNS:\t\t{:#064b}", BB_WHITE_PAWNS);
    println!("BB_WHITE_KNIGHTS:\t{:#064b}", BB_WHITE_KNIGHTS);
    println!("BB_WHITE_BISHOPS:\t{:#064b}", BB_WHITE_BISHOPS);
    println!("BB_WHITE_ROOKS:\t\t{:#064b}", BB_WHITE_ROOKS);
    println!("BB_WHITE_QUEEN:\t\t{:#064b}", BB_WHITE_QUEEN);
    println!("BB_WHITE_KING:\t\t{:#064b}", BB_WHITE_KING);

    print_bitboard(BB_WHITE_BISHOPS);
}
