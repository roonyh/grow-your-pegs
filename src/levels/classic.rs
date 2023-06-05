use super::Level;

pub struct Classic {
}

impl Level for Classic {
    fn init_board(&self, board: &mut Vec<Vec<crate::Hole>>) {
        for x in 0..7 {
            for y in 0..7 {
                let mut is_hole = true;
                if x < 2 && y < 2 {
                    is_hole = false;
                }
    
                if x > 4 && y > 4 {
                    is_hole = false;
                }
    
                if x > 4 && y < 2 {
                    is_hole = false;
                }
    
                if x < 2 && y > 4 {
                    is_hole = false;
                }
    
                board[x][y].is_hole = is_hole;
                board[x][y].filled = x == 3 && y == 3;
            }
        }
    }

    fn get_selected(&self) -> (u8, u8){
        (3, 3)
    }

    fn get_max_pegs(&self) -> u8 {
        32
    }
}