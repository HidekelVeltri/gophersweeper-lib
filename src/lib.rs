use rand::Rng;
use std::collections::HashSet;

const SMALL:  (usize, usize) = (10, 8);
const MEDIUM: (usize, usize) = (18, 12);
const BIG:    (usize, usize) = (24, 20);

const EASY:   f32 = 0.1;
const NORMAL: f32 = 0.15;
const HARD:   f32 = 0.2;

#[derive(Default)]
pub struct Cell {
    pub is_exposed: bool,
    pub is_flagged: bool,
    pub has_gopher: bool,
    pub surrounding_gophers: u8,
}

pub struct GopherSweeper {
    pub config: GameConfig,
    remaining_cells: usize,
    field: Vec<Vec<Cell>>,
}

impl GopherSweeper {
    pub fn new(config: GameConfig) -> Self {
        let (width, height) = config.size();
        let gophers = config.gophers();

        let mut result = GopherSweeper {
            config,
            remaining_cells: width * height - gophers,
            field: Vec::with_capacity(height),
        };

        for y in 0..height {
            result.field.push(Vec::with_capacity(width));

            for _ in 0..width {
                result.field[y].push(Cell::default());
            }
        }

        let mut rng = rand::thread_rng();
        let mut random_coords: (usize, usize);
        let mut planted_gophers = HashSet::with_capacity(gophers);

        while planted_gophers.len() < gophers {
            random_coords = (rng.gen_range(0..width), rng.gen_range(0..height));

            if planted_gophers.insert(random_coords) {
                result.field[random_coords.1][random_coords.0].has_gopher = true;

                for (x, y) in result.surrounding_cells_coords(random_coords.0, random_coords.1) {
                    result.field[y][x].surrounding_gophers += 1;
                }
            }
        }

        result
    }

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.field[y][x]
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> ToggleFlagResult {
        let mut cell = &mut self.field[y][x];

        if cell.is_exposed {
            return ToggleFlagResult::CellWasExposed;
        }

        cell.is_flagged = !cell.is_flagged;

        if cell.is_flagged {
            ToggleFlagResult::Enabled
        } else {
            ToggleFlagResult::Disabled
        }
    }

    pub fn try_expose_cell(&mut self, x: usize, y: usize) -> ExposeResult {
        let cell = &self.field[y][x];

        if cell.is_exposed { return ExposeResult::WasAlreadyExposed }
        if cell.is_flagged { return ExposeResult::IsFlagged }
        if cell.has_gopher { return ExposeResult::HasGopher }

        self.expose_recursively(x, y);

        if self.remaining_cells == 0 { return ExposeResult::Win }

        ExposeResult::Exposed
    }

    fn expose_recursively(&mut self, x: usize, y: usize) {
        let mut cell = &mut self.field[y][x];

        cell.is_exposed = true;
        self.remaining_cells -= 1;

        if cell.surrounding_gophers == 0 {
            for (x, y) in self.surrounding_cells_coords(x, y) {
                if !self.field[y][x].is_exposed {
                    self.expose_recursively(x, y);
                }
            }
        }
    }

    fn surrounding_cells_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::with_capacity(8);

        let (w, h) = self.config.size();

        if x > 0 {
            result.push((x - 1, y));
        }

        if y > 0 {
            result.push((x, y - 1));
        }

        if x + 1 < w {
            result.push((x + 1, y));
        }

        if y + 1 < h {
            result.push((x, y + 1));
        }

        if x > 0 && y > 0 {
            result.push((x - 1, y - 1));
        }

        if x > 0 && y + 1 < h {
            result.push((x - 1, y + 1));
        }

        if x + 1 < w && y > 0 {
            result.push((x + 1, y - 1));
        }

        if x + 1 < w && y + 1 < h {
            result.push((x + 1, y + 1));
        }

        result
    }
}

impl<'a> IntoIterator for &'a GopherSweeper {
    type Item = &'a Vec<Cell>;
    type IntoIter = std::slice::Iter<'a, Vec<Cell>>;

    fn into_iter(self) -> Self::IntoIter {
        self.field.iter()
    }
}

pub enum ExposeResult {
    Exposed,
    WasAlreadyExposed,
    IsFlagged,
    HasGopher,
    Win,
}

pub enum ToggleFlagResult {
    Enabled,
    Disabled,
    CellWasExposed,
}

#[derive(Default)]
pub enum FieldSize {
    #[default]
    Small,
    Medium,
    Big,
    Custom {
        width: usize,
        height: usize,
    },
}

#[derive(Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
    Custom {
        gophers_percentage: f32
    },
}

#[derive(Default)]
pub struct GameConfig {
    field_size: FieldSize,
    difficulty: Difficulty,
}

impl GameConfig {
    pub fn new(field_size: FieldSize, difficulty: Difficulty) -> Self {
        GameConfig {
            field_size,
            difficulty,
        }
    }
    
    pub fn size(&self) -> (usize, usize) {
        match self.field_size {
            FieldSize::Small => SMALL,
            FieldSize::Medium => MEDIUM,
            FieldSize::Big => BIG,
            FieldSize::Custom { width, height } => (width, height),
        }
    }
    
    pub fn gophers(&self) -> usize {
        let (width, height) = self.size();
        
        (match self.difficulty {
            Difficulty::Easy => EASY,
            Difficulty::Normal => NORMAL,
            Difficulty::Hard => HARD,
            Difficulty::Custom { gophers_percentage } => gophers_percentage,
        } * (width * height) as f32).ceil() as usize
    }
}
