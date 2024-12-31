use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(VIOLET);
        next_frame().await
    }
}
