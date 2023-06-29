use crate::machine::uefi::{Screen, NO_OF_PIXELS_IN_A_ROW, NO_OF_PIXELS_IN_A_COLUMN};
use crate::sync::{mutex::Mutex};
use crate::display::bitmap::{draw_bitmap, Bitmap, erase_bitmap}; // NEW
use crate::alloc::allocator;
use crate::alloc::boxed_fn::BoxedFn;
use crate::event_hook::{self, EventKind, EventInfo};
use crate::machine::keyboard::{KeyEvent, KeyDirection, KeyCode};

// The number of rows of blocks that are going to be drawn on screen
const NO_OF_BLOCK_ROWS: usize = 4;

pub fn blasterball(screen: &Mutex<Option<&mut Screen>>) -> ! {
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

    let mut paddle_pos = (
        NO_OF_PIXELS_IN_A_COLUMN - paddle.height() - 10,
        NO_OF_PIXELS_IN_A_ROW / 2 - 45
    );
    draw_bitmap(screen, &paddle, paddle_pos);

    let mut ball_pos = (
        paddle_pos.0 - ball.height() - 5,
        paddle_pos.1 + paddle.width() / 2 - 10
    );
    draw_bitmap(screen, &ball, ball_pos);

    event_hook::hook_event(EventKind::Keyboard, BoxedFn::new(|info| {
        if let EventInfo::Keyboard(KeyEvent { keycode, direction }) = info {
            if direction == KeyDirection::Down {
                if keycode == KeyCode::ArrowLeft {
                    erase_bitmap(screen, &paddle, paddle_pos);
                    paddle_pos.1 = paddle_pos.1.checked_sub(10)
                        .or(Some(0))
                        .unwrap();
                    draw_bitmap(screen, &paddle, paddle_pos);
                } else if keycode == KeyCode::ArrowRight {
                    erase_bitmap(screen, &paddle, paddle_pos);
                    paddle_pos.1 += 10;
                    if paddle_pos.1 + paddle.width() > NO_OF_PIXELS_IN_A_ROW {
                        paddle_pos.1 = NO_OF_PIXELS_IN_A_ROW - paddle.width();
                    }
                    draw_bitmap(screen, &paddle, paddle_pos);
                }
            }
        }
    }, allocator::get_allocator()));
    
    loop {}
}