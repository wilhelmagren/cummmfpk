"""

"""

from typing import Optional

STARTING_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

COLORS = [ WHITE, BLACK ] = range(2)

PIECES = [ NONE, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING ] = range(7)


BB_VOID = 0b0000000000000000000000000000000000000000000000000000000000000000
BB_ALL = 0b1111111111111111111111111111111111111111111111111111111111111111

SQUARES = [
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8 ] = range(64)

BB_SQUARES = [
    BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1,
    BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2,
    BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3,
    BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4,
    BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5,
    BB_A6, BB_B6, BB_C6, BB_D6, BB_E6, BB_F6, BB_G6, BB_H6,
    BB_A7, BB_B7, BB_C7, BB_D7, BB_E7, BB_F7, BB_G7, BB_H7,
    BB_A8, BB_B8, BB_C8, BB_D8, BB_E8, BB_F8, BB_G8, BB_H8
] = [ 1 << i for i in SQUARES ]

############################################
# White Bitboards
############################################

BB_WHITE_PAWNS = BB_A2 | BB_B2 | BB_C2 | BB_D2 | BB_E2 | BB_F2 | BB_G2 | BB_H2
BB_WHITE_KNIGHTS = BB_B1 | BB_G1
BB_WHITE_BISHOPS = BB_C1 | BB_F1
BB_WHITE_ROOKS = BB_A1 | BB_H1
BB_WHITE_QUEENS = BB_D1
BB_WHITE_KING = BB_E1

############################################
# Black Bitboards
############################################

BB_BLACK_PAWNS = BB_A7 | BB_B7 | BB_C7 | BB_D7 | BB_E7 | BB_F7 | BB_G7 | BB_H7
BB_BLACK_KNIGHTS = BB_B8 | BB_G8
BB_BLACK_BISHOPS = BB_C8 | BB_F8
BB_BLACK_ROOKS = BB_A8 | BB_H8
BB_BLACK_QUEENS = BB_D8
BB_BLACK_KING = BB_E8


# https://www.codeproject.com/Articles/5313417/Worlds-fastest-Bitboard-Chess-Movegenerator
# https://www.chessprogramming.org/Bitboards
# https://www.youtube.com/watch?v=bomzBSaW7GQ


class Bitboard(object):
    def __init__(self, fen: Optional[str] = None):
        self._bitboards = {
            WHITE: {
                PAWN: BB_WHITE_PAWNS,
                KNIGHT: BB_WHITE_KNIGHTS,
                BISHOP: BB_WHITE_BISHOPS,
                ROOK: BB_WHITE_ROOKS,
                QUEEN: BB_WHITE_QUEENS,
                KING: BB_WHITE_KING
            },
            BLACK: {
                PAWN: BB_BLACK_PAWNS,
                KNIGHT: BB_BLACK_KNIGHTS,
                BISHOP: BB_BLACK_BISHOPS,
                ROOK: BB_BLACK_ROOKS,
                QUEEN: BB_BLACK_QUEENS,
                KING: BB_BLACK_KING
            }
        }

    def _generate_bishop_moves(self, color):
        pass

    
if __name__ == "__main__":

    print(SQUARES)
    print(STARTING_FEN)
    print(WHITE)
    print(PAWN)
    print(BB_VOID)
    print(BB_ALL)
    print(A2)
    print(BB_A2)
    print(bin(BB_WHITE_PAWNS))