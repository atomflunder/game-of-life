const CELLS_WIDTH: usize = 90;
const CELLS_HEIGHT: usize = 60;
const SCREEN_WIDTH: usize = 1200;
const SCREEN_HEIGHT: usize = 800;

#[derive(Clone)]

pub struct MainGame {
    pub board: [[bool; CELLS_WIDTH]; CELLS_HEIGHT],
    pub cycle: usize,
    pub started: bool,
}
impl ggez::event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(ctx, 3) {
            if self.started {
                self.advance_step();
                ggez::graphics::set_window_title(
                    ctx,
                    &format!("Game of Life - Cycle: {}", self.cycle),
                );
            }
        }

        if !self.started {
            let mouse_pos = ggez::input::mouse::position(ctx);
            let cell_x = (mouse_pos.x / (SCREEN_WIDTH as f32 / CELLS_WIDTH as f32)) as usize;
            let cell_y = (mouse_pos.y / (SCREEN_HEIGHT as f32 / CELLS_HEIGHT as f32)) as usize;
            if ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left) {
                self.board[cell_y][cell_x] = true;
            } else if ggez::input::mouse::button_pressed(
                ctx,
                ggez::input::mouse::MouseButton::Right,
            ) {
                self.board[cell_y][cell_x] = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        ggez::graphics::clear(ctx, ggez::graphics::Color::BLACK);

        let mut offset = (0.0, 0.0);
        let mut coordinates: (usize, usize) = (0, 0);

        let alive_cell = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(
                0.0,
                0.0,
                SCREEN_WIDTH as f32 / CELLS_WIDTH as f32,
                SCREEN_HEIGHT as f32 / CELLS_HEIGHT as f32,
            ),
            ggez::graphics::Color::new(0.9, 0.65, 0.65, 1.0),
        )?;

        let dead_cell = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(
                0.0,
                0.0,
                SCREEN_WIDTH as f32 / CELLS_WIDTH as f32,
                SCREEN_HEIGHT as f32 / CELLS_HEIGHT as f32,
            ),
            ggez::graphics::Color::new(0.2, 0.2, 0.2, 1.0),
        )?;

        for line in &self.board {
            for cell in line {
                if *cell {
                    ggez::graphics::draw(
                        ctx,
                        &alive_cell,
                        ggez::graphics::DrawParam::new().dest([offset.0, offset.1]),
                    )?;
                } else {
                    ggez::graphics::draw(
                        ctx,
                        &dead_cell,
                        ggez::graphics::DrawParam::new().dest([offset.0, offset.1]),
                    )?;
                }
                offset.0 += SCREEN_WIDTH as f32 / CELLS_WIDTH as f32;
                coordinates.0 += 1;
            }

            offset.0 = 0.0;
            offset.1 += SCREEN_HEIGHT as f32 / CELLS_HEIGHT as f32;
            coordinates.0 = 0;
            coordinates.1 += 1;
        }
        ggez::graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == ggez::event::KeyCode::Space {
            self.started = !self.started;
        }
    }
}

impl MainGame {
    pub fn new() -> ggez::GameResult<MainGame> {
        let b = [[false; CELLS_WIDTH]; CELLS_HEIGHT];
        Ok(Self {
            board: b,
            cycle: 0,
            started: false,
        })
    }

    pub fn advance_step(&mut self) {
        let mut new_board = self.clone();
        for (i, line) in self.board.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                let living_neighbors = self.get_neighbors((i, j));

                if !*cell {
                    if living_neighbors == 3 {
                        new_board.board[i][j] = true;
                    }
                } else if !(2..=3).contains(&living_neighbors) {
                    new_board.board[i][j] = false;
                }
            }
        }
        self.board = new_board.board;
        self.cycle += 1;
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        self.board[y][x] = !self.board[y][x];
    }

    fn get_neighbors(&self, coordinates: (usize, usize)) -> usize {
        let mut living_neighbors: usize = 0;
        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        if coordinates.0 > 0 {
            neighbors.push((coordinates.0 - 1, coordinates.1));
            if coordinates.1 > 0 {
                neighbors.push((coordinates.0 - 1, coordinates.1 - 1));
            }
            if coordinates.1 < self.board[0].len() - 1 {
                neighbors.push((coordinates.0 - 1, coordinates.1 + 1));
            }
        }
        if coordinates.1 > 0 {
            neighbors.push((coordinates.0, coordinates.1 - 1));
            if coordinates.0 < self.board.len() - 1 {
                neighbors.push((coordinates.0 + 1, coordinates.1 - 1));
            }
        }
        if coordinates.0 < self.board.len() - 1 {
            neighbors.push((coordinates.0 + 1, coordinates.1));
            if coordinates.1 < self.board[0].len() - 1 {
                neighbors.push((coordinates.0 + 1, coordinates.1 + 1));
            }
        }
        if coordinates.1 < self.board[0].len() - 1 {
            neighbors.push((coordinates.0, coordinates.1 + 1));
        }

        for neighbor in neighbors {
            if self.board[neighbor.0][neighbor.1] {
                living_neighbors += 1;
            }
        }
        living_neighbors
    }
}
fn main() {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Game of Life", "atomflunder")
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
        )
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Game of Life - Cycle: 0")
                .vsync(true),
        )
        .build()
        .unwrap();
    ggez::event::run(ctx, event_loop, MainGame::new().unwrap());
}
