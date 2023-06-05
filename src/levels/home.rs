use super::Level;

pub struct Home {}

impl Level for Home {
    fn init_board(&self, board: &mut Vec<Vec<crate::Hole>>) {
        for x in 0..7 {
            for y in 0..7 {
                board[x][y].filled = false;
                if x > 1 && x < 5 && y > 1 && y < 5 {
                    board[x][y].is_hole = true;
                } else {
                    board[x][y].is_hole = false;
                }
            }
        }

        board[1][2].is_hole = true;
        board[3][1].is_hole = true;
        board[3][1].filled = true;
        board[5][2].is_hole = true;
    }

    fn get_selected(&self) -> (u8, u8){
        (3, 1)
    }
    
    fn get_max_pegs(&self) -> u8 {
        11
    }
}
