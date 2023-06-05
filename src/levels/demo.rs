use super::Level;

pub struct Intro {
}

impl Level for Intro {
    fn init_board(&self, board: &mut Vec<Vec<crate::Hole>>) {
        for x in 0..7 {
            for y in 0..7 {
                if x % 2 == 0 && y %2 == 0 {
                    board[x][y].is_hole = true;
                    board[x][y].filled = true;
                } else if x % 2 == 1 && y %2 == 1 {
                    board[x][y].is_hole = true;
                    board[x][y].filled = false;
                } else {
                    board[x][y].filled = false;
                }
            }
        }
    }

    fn get_selected(&self) -> (u8, u8){
        (2, 3)
    }
    
    fn get_max_pegs(&self) -> u8 {
        2
    }
}