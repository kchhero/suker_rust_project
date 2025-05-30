use sdl2::{
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    keyboard::Keycode,
    event::Event,
    Sdl, TimerSubsystem,
};
use rand::Rng;
use crate::audiocvt::AudioManager;
use crate::blocks::{BLOCK_SIZE, BOARD_WIDTH, BOARD_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT, BLOCKS, COLORS, TetrisBlock};

pub struct Tetris {
    canvas: Canvas<Window>,
    audio: AudioManager,
    board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
    current: TetrisBlock,
    next: TetrisBlock,
    score: usize,
    remaining: usize,
    last_tick: u32,
    timer: TimerSubsystem,
    event_pump: sdl2::EventPump,
}

impl Tetris {
    pub fn new(sdl: &Sdl, mut audio: AudioManager) -> Result<Self, String> {
        let video = sdl.video()?;
        let window = video
            .window("Rust Tetris (SDL2)", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .build();
        let canvas = window.map_err(|e| e.to_string())?.into_canvas().build().map_err(|e| e.to_string())?;
        let timer = sdl.timer()?;
        let event_pump = sdl.event_pump()?;

        audio.play();

        let mut rng = rand::rng();
        let shape = rng.random_range(1..=7);
        let next_shape = rng.random_range(1..=7);

        Ok(Self {
            canvas,
            audio,
            board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            current: TetrisBlock { x: 3, y: 0, shape, rotation: 0 },
            next: TetrisBlock { x: 3, y: 0, shape: next_shape, rotation: 0 },
            score: 0,
            remaining: 30,
            last_tick: timer.ticks(),
            timer,
            event_pump,
        })
    }


    pub fn run(&mut self) -> Result<(), String> {
        while self.remaining > 0 {
            let mut should_place = false;
            let mut move_down = false;

            let events: Vec<Event> = self.event_pump.poll_iter().collect();
            for event in events {
                match event {
                    Event::Quit { .. } => return Ok(()),
                    Event::KeyDown { keycode: Some(key), .. } => {
                        self.handle_input(key);
                    }
                    _ => {}
                }
            }
            // 키 입력에 대한 추가 처리 필요시 여기에 작성
            let now = self.timer.ticks();
            if now - self.last_tick > 500 {
                move_down = true;
                self.last_tick = now;
            }

            if move_down {
                let mut moved = self.current;
                moved.y += 1;

                if !self.check_collision(&moved) {
                    self.current = moved;
                } else {
                    should_place = true;
                }
            }

            if should_place {
                self.place_block();
            }

            self.render()?;
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        self.audio.stop();
        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.draw_board();
        let current_clone = self.current.clone();
        self.draw_tetris(&current_clone);

        self.canvas.present();
        Ok(())
    }

    fn draw_block(&mut self, x: i32, y: i32, color: Color) {
        let rect = Rect::new(
            x * BLOCK_SIZE,
            y * BLOCK_SIZE,
            (BLOCK_SIZE - 1) as u32,
            (BLOCK_SIZE - 1) as u32,
        );
        self.canvas.set_draw_color(color);
        let _ = self.canvas.fill_rect(rect);
    }

    fn draw_tetris(&mut self, t: &TetrisBlock) {
        for dy in 0..4 {
            for dx in 0..4 {
                if BLOCKS[t.shape][t.rotation][dy][dx] != 0 {
                    let x = t.x + dx as i32;
                    let y = t.y + dy as i32;
                    if x >= 0 && y >= 0 {
                        self.draw_block(x, y, COLORS[t.shape]);
                    }
                }
            }
        }
    }

    fn draw_board(&mut self) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let color = if self.board[y][x] == 0 {
                    Color::RGB(22, 22, 22)
                } else {
                    COLORS[self.board[y][x]]
                };
                self.draw_block(x as i32, y as i32, color);
            }
        }
    }

    fn handle_input(&mut self, key: Keycode) {
        let mut moved = self.current;
        match key {
            Keycode::Left => moved.x -= 1,
            Keycode::Right => moved.x += 1,
            Keycode::Down => moved.y += 1,
            Keycode::Up => moved.rotation = (moved.rotation + 1) % 4,
            Keycode::Space => {
                while !self.check_collision(&moved) {
                    self.current = moved;
                    moved.y += 1;
                }
                self.place_block();
                return;
            }
            _ => {}
        }

        if !self.check_collision(&moved) {
            self.current = moved;
        }
    }

    fn check_collision(&self, t: &TetrisBlock) -> bool {
        for dy in 0..4 {
            for dx in 0..4 {
                if BLOCKS[t.shape][t.rotation][dy][dx] != 0 {
                    let nx = t.x + dx as i32;
                    let ny = t.y + dy as i32;
                    if nx < 0 || nx >= BOARD_WIDTH as i32 || ny >= BOARD_HEIGHT as i32 {
                        return true;
                    }
                    if ny >= 0 && self.board[ny as usize][nx as usize] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn place_block(&mut self) {
        for dy in 0..4 {
            for dx in 0..4 {
                if BLOCKS[self.current.shape][self.current.rotation][dy][dx] != 0 {
                    let x = self.current.x + dx as i32;
                    let y = self.current.y + dy as i32;
                    if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                        self.board[y as usize][x as usize] = self.current.shape;
                    }
                }
            }
        }

        for y in 0..BOARD_HEIGHT {
            if self.board[y].iter().all(|&val| val != 0) {
                for row in (1..=y).rev() {
                    self.board[row] = self.board[row - 1];
                }
                self.board[0] = [0; BOARD_WIDTH];
                self.score += 100;
            }
        }

        self.current = self.next;
        let mut rng = rand::rng();
        self.next = TetrisBlock {
            x: 3,
            y: -2,
            shape: rng.random_range(1..=7),
            rotation: 0,
        };
        self.remaining -= 1;
    }
}
