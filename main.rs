#![no_std]
#![no_main]

use crossbeam::atomic::AtomicCell;
use pc_keyboard::DecodedKey;
use pluggable_interrupt_os::vga_buffer::{clear_screen, plot_str, plot_num, ColorCode, Color, plot};
use pluggable_interrupt_os::HandlerTable;

static TICKED: AtomicCell<bool> = AtomicCell::new(false);
static KEY: AtomicCell<Option<DecodedKey>> = AtomicCell::new(None);

fn cpu_loop() -> ! {
    let mut game = PongGame::new();
    loop {
        if let Ok(_) = TICKED.compare_exchange(true, false) {
            game.tick();
        }
        
        if let Ok(k) = KEY.fetch_update(|k| if k.is_some() {Some(None)} else {None}) {
            if let Some(k) = k {
                game.key(k);
            }
        }
    }
}

fn tick() {
    TICKED.store(true);
}

fn key(key: DecodedKey) {
    KEY.store(Some(key));
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .cpu_loop(cpu_loop)
        .start()
}

struct PongGame {
    ball_row: usize,
    ball_col: usize,
    ball_dx: isize,
    ball_dy: isize,
    paddle_left_row: usize,
    paddle_right_row: usize,
    paddle_height: usize,
    score_left: usize,
    score_right: usize,
}

impl PongGame {
    fn new() -> Self {
        Self {
            ball_row: 12,
            ball_col: 40,
            ball_dx: 1,
            ball_dy: 1,
            paddle_left_row: 10,
            paddle_right_row: 10,
            paddle_height: 4,
            score_left: 0,
            score_right: 0,
        }
    }

    fn tick(&mut self) {
        self.update_ball();
        self.render();
    }

    fn update_ball(&mut self) {
        self.ball_row = (self.ball_row as isize + self.ball_dy) as usize;
        self.ball_col = (self.ball_col as isize + self.ball_dx) as usize;

        if self.ball_row == 0 || self.ball_row == 24 {
            self.ball_dy = -self.ball_dy;
        }

        if self.ball_col == 1 && self.ball_row >= self.paddle_left_row && self.ball_row < self.paddle_left_row + self.paddle_height {
            self.ball_dx = -self.ball_dx;
        }
        if self.ball_col == 78 && self.ball_row >= self.paddle_right_row && self.ball_row < self.paddle_right_row + self.paddle_height {
            self.ball_dx = -self.ball_dx;
        }

        if self.ball_col == 0 {
            self.score_right += 1;
            self.reset_ball();
        } else if self.ball_col == 79 {
            self.score_left += 1;
            self.reset_ball();
        }
    }

    fn reset_ball(&mut self) {
        self.ball_row = 12;
        self.ball_col = 40;
        self.ball_dx = -self.ball_dx;
        self.ball_dy = if self.ball_dy == 0 { 1 } else { self.ball_dy };
    }

    fn render(&self) {
        clear_screen();
        
        for i in 0..self.paddle_height {
            plot('|', 0, self.paddle_left_row + i, ColorCode::new(Color::White, Color::Black));
            plot('|', 79, self.paddle_right_row + i, ColorCode::new(Color::White, Color::Black));
        }

        plot('O', self.ball_col, self.ball_row, ColorCode::new(Color::White, Color::Black));

        plot_num(self.score_left as isize, 30, 0, ColorCode::new(Color::White, Color::Black));
        plot_str(":", 32, 0, ColorCode::new(Color::White, Color::Black));
        plot_num(self.score_right as isize, 34, 0, ColorCode::new(Color::White, Color::Black));
    }

    fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(pc_keyboard::KeyCode::ArrowUp) if self.paddle_right_row > 0 => self.paddle_right_row -= 1,
            DecodedKey::RawKey(pc_keyboard::KeyCode::ArrowDown) if self.paddle_right_row + self.paddle_height < 24 => self.paddle_right_row += 1,
            DecodedKey::Unicode('w') if self.paddle_left_row > 0 => self.paddle_left_row -= 1,
            DecodedKey::Unicode('s') if self.paddle_left_row + self.paddle_height < 24 => self.paddle_left_row += 1,
            _ => {}
        }
    }
}