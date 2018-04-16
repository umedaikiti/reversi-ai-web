mod reversi_ai;
use reversi_ai::reversi::{ReversiBoard, Color, U64Board, Move};
use reversi_ai::reversi::reversi_ai::best_move_alpha_beta2;
static mut BOARD : U64Board = reversi_ai::reversi::U64BOARD0;

fn convert_color(color : i8) -> Option<Color> {
    return if color == 1 {
        Some(Color::X)
    }
    else if color == -1 {
        Some(Color::O)
    }
    else {
        None
    };
}
#[no_mangle]
pub unsafe extern fn reset() {
    BOARD = reversi_ai::reversi::U64BOARD0;
}

#[no_mangle]
pub unsafe extern fn get_board(x : u32, y : u32) -> i8 {
    return match BOARD.get(x, y) {
        Some(Color::X) => 1,
        Some(Color::O) => -1,
        None => 0,
    };
}

#[no_mangle]
pub unsafe extern fn is_valid_move(x : u32, y : u32, color : i8) -> bool {
    if let Some(color) = convert_color(color) {
        return BOARD.valid_move(x, y, color);
    }
    else {
        return false;
    }
}

#[no_mangle]
pub unsafe extern fn set_disk(x : u32, y : u32, color : i8) -> bool {
    if let Some(color) = convert_color(color) {
        return BOARD.do_move(Move::Mv(x,y), color);
    }
    else {
        return true;
    }
}

#[no_mangle]
pub unsafe extern fn ai_think(color : i8) -> i32 {
    if let Some(color) = convert_color(color) {
        return match best_move_alpha_beta2(&BOARD, color) {
            Move::Pass => -1,
            Move::Mv(x, y) => ((x << 3) | y) as i32,
        };
    } else {
        return -1;
    }
}

#[no_mangle]
pub unsafe extern fn has_valid_moves(color : i8) -> bool {
    if let Some(color) = convert_color(color) {
        return reversi_ai::reversi::reversi_ai::has_valid_moves(&BOARD, color);
    } else {
        return false;
    }
}
