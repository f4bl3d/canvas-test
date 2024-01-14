pub static COLORS: [(&str, &str); 6] = [
    ("Black", "#000000"),
    ("Green", "#3DC06C"),
    ("Red", "#FF0000"),
    ("Blue", "#4F8DE4"),
    ("Yellow", "#FAE589"),
    ("White", "#FFFFFF"),
];

static DEFAULT_COLOR: &str = COLORS[0].1;

pub static PEN_SIZES: [f64; 4] = [1.0, 2.0, 4.0, 8.0];

static DEFAULT_PEN_SIZE: f64 = PEN_SIZES[0];

pub struct State {
    width: u32,
    height: u32,
    is_drawing: bool,
    color: String,
    pen_size: f64,
    undo_list: Vec<String>,
    redo_list: Vec<String>,
}

impl State {
    pub fn new(w: u32, h: u32) -> State {
        State {
            width: w,
            height: h,
            is_drawing: false,
            color: DEFAULT_COLOR.to_string(),
            pen_size: DEFAULT_PEN_SIZE,
            undo_list: vec![],
            redo_list: vec![],
        }
    }

    pub fn start_drawing(&mut self) {
        self.redo_list = vec![];
        self.is_drawing = true;
    }

    pub fn stop_drawing(&mut self) {
        self.is_drawing = false;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing
    }

    pub fn update_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

    pub fn update_pen_size(&mut self, size: f64) {
        self.pen_size = size;
    }

    pub fn get_pen_size(&self) -> f64 {
        self.pen_size
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn add_undo_state(&mut self, prev: String) {
        self.undo_list.push(prev);
    }

    pub fn add_redo_state(&mut self, next: String) {
        self.redo_list.push(next);
    }

    pub fn undo(&mut self) -> Option<String> {
        self.undo_list.pop()
    }

    pub fn redo(&mut self) -> Option<String> {
        self.redo_list.pop()
    }
}
