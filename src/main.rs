#[derive(Clone)]
pub struct MainGame {
    pub board: [[bool; 32]; 32],
    pub cycle: usize,
}
impl ggez::event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while ggez::timer::check_update_time(ctx, 2) {
            self.advance_step();
            ggez::graphics::set_window_title(ctx, &format!("Game of Life - Cycle: {}", self.cycle));
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
            ggez::graphics::Rect::new(0.0, 0.0, 20.0, 20.0),
            ggez::graphics::Color::new(0.25, 0.9, 0.25, 1.0),
        )?;
        let dead_cell = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(0.0, 0.0, 20.0, 20.0),
            ggez::graphics::Color::new(0.15, 0.15, 0.15, 1.0),
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
                offset.0 += 20.0;
                coordinates.0 += 1;
            }

            offset.0 = 0.0;
            offset.1 += 20.0;
            coordinates.0 = 0;
            coordinates.1 += 1;
        }
        ggez::graphics::present(ctx)?;
        Ok(())
    }
}

impl MainGame {
    pub fn new() -> ggez::GameResult<MainGame> {
        let b: [[bool; 32]; 32] = rand::random();
        Ok(Self { board: b, cycle: 0 })
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
        .window_mode(ggez::conf::WindowMode::default().dimensions(640.0, 640.0))
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Game of Life - Cycle: 0")
                .vsync(true),
        )
        .build()
        .unwrap();
    ggez::event::run(ctx, event_loop, MainGame::new().unwrap());
}
