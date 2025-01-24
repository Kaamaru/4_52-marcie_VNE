use macroquad::prelude::*;
use dialog::run_dialog;

mod dialog;
#[macroquad::main("UI showcase")]
async fn main() {
    run_dialog().await;
}
