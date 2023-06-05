use super::Level;

pub struct Rect {}

impl Level for Rect {
    fn init_board(&self, board: &mut Vec<Vec<crate::Hole>>) {
        for x in 0..7 {
            for y in 0..7 {
                if x > 1 && x < 5 && y > 1 && y < 6 {
                    board[x][y].is_hole = true;
                    board[x][y].filled = false;
                }
            }
        }

        board[2][5].filled = true;
    }

    fn get_selected(&self) -> (u8, u8){
        (2, 5)
    }
    
    fn get_max_pegs(&self) -> u8 {
        11
    }
}
