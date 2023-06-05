use super::Level;

pub struct Intro {
}

impl Level for Intro {
    fn init_board(&self, board: &mut Vec<Vec<crate::Hole>>) {
        for x in 0..7 {
            for y in 0..7 {
                if x == 2 && y == 3 {
                    board[x][y].is_hole = true;
                    board[x][y].filled = true;
                }
    
                if x == 3 && y == 3 {
                    board[x][y].is_hole = true;
                    board[x][y].filled = false;
                }
    
                if x == 4 && y == 3 {
                    board[x][y].is_hole = true;
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