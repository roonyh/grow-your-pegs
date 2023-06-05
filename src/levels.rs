use crate::Hole;

pub mod classic;
pub mod intro;
pub mod rectangle;
pub mod home;
pub mod euro;

pub trait Level {
    //starting_selected: (u8, u8);
    fn init_board(&self, board: &mut Vec<Vec<Hole>>);
    fn get_selected(&self) -> (u8, u8);
    fn get_max_pegs(&self) -> u8;
}
