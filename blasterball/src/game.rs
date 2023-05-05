use crate::machine::uefi::{Screen, NO_OF_PIXELS_IN_A_ROW, NO_OF_PIXELS_IN_A_COLUMN};
use crate::display::bitmap::{draw_bitmap, Bitmap};

// The number of rows of blocks that are going to be drawn on screen
const NO_OF_BLOCK_ROWS: usize = 4;

pub fn blasterball(screen: &mut Screen) -> ! {
    // Loading the blocks into an array
    let blue_block_bytes = include_bytes!("./assets/blue_block.bmp");
    let blue_block = Bitmap::new(blue_block_bytes);
    let cyan_block_bytes = include_bytes!("./assets/cyan_block.bmp");
    let cyan_block = Bitmap::new(cyan_block_bytes);
    let green_block_bytes = include_bytes!("./assets/green_block.bmp");
    let green_block = Bitmap::new(green_block_bytes);
    let pink_block_bytes = include_bytes!("./assets/pink_block.bmp");
    let pink_block = Bitmap::new(pink_block_bytes);
    let yellow_block_bytes = include_bytes!("./assets/yellow_block.bmp");
    let yellow_block = Bitmap::new(yellow_block_bytes);
    let paddle_bytes = include_bytes!("./assets/paddle.bmp");
    let paddle = Bitmap::new(paddle_bytes);
    let ball_bytes = include_bytes!("./assets/ball.bmp");
    let ball = Bitmap::new(ball_bytes);
    let blocks = [blue_block, cyan_block, green_block, pink_block, yellow_block];

    // The initial block position is at the top left corner
    // of the screen
    let mut block_position = (0, 0); // (row, column)

    // Cycle through the blocks until 4 rows have been filled
    for (i, block) in blocks.iter().cycle().enumerate() {
        let no_of_blocks_in_a_row = NO_OF_PIXELS_IN_A_ROW / block.width();
        if i >= no_of_blocks_in_a_row * NO_OF_BLOCK_ROWS {
            break;
        }
        draw_bitmap(screen, block, block_position);
        block_position.1 += block.width();
        if block_position.1 >= NO_OF_PIXELS_IN_A_ROW {
            block_position.0 += block.height();
            block_position.1 = 0;
        }
    }

    let initial_paddle_position = (
        NO_OF_PIXELS_IN_A_COLUMN - paddle.height() - 10,
        NO_OF_PIXELS_IN_A_ROW / 2 - 45
    );
    draw_bitmap(screen, &paddle, initial_paddle_position);

    let initial_ball_position = (
        initial_paddle_position.0 - ball.height() - 5,
        initial_paddle_position.1 + paddle.width() / 2 - 10
    );
    draw_bitmap(screen, &ball, initial_ball_position);
    
    loop {}
}