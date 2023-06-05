use super::Level;

pub struct Euro {
}

impl Level for Euro {
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

                if x==1 && y==1 {
                    is_hole = true;
                }

                if x==1 && y==5 {
                    is_hole = true;
                }

                if x==5 && y==1 {
                    is_hole = true;
                }

                if x==5 && y==5 {
                    is_hole = true;
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