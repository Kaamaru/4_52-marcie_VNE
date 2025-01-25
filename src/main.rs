use macroquad::prelude::*;
use dialog::*;
use macroquad::input::*;

mod dialog;


pub static mut DIALOGVISIBILITY : bool = true;

#[macroquad::main("UI showcase")]
async fn main() {

    // println!("{:?}",fs::metadata("/some/file/path.txt")?);
    run_dialog().await;

    loop{









    }
}
