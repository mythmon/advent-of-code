use cursive::{direction::Orientation, event::Key, logger, views, Cursive};
use std::iter::Iterator;
use year2019::intcode::{IntcodeComputer, PauseReason};

struct State {
    screen: Screen,
    content: views::TextContent,
    score: views::TextContent,
    computer: IntcodeComputer,
}

fn main() {
    // setup computer
    let input = include_str!("input");
    let program = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut computer = IntcodeComputer::build(program).done();
    computer.write_mem(0, 2); // insert two quarters

    // Draw first screen
    let mut screen = Screen::new(43, 23);

    let score = update_screen(None, &mut computer, &mut screen);

    let state = State {
        content: views::TextContent::new(screen.render()),
        screen,
        computer,
        score: views::TextContent::new(format!("{}", score.unwrap_or(0))),
    };

    logger::init();

    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        views::LinearLayout::new(Orientation::Vertical)
            .child(views::TextView::new_with_content(state.score.clone()))
            .child(views::TextView::new_with_content(state.content.clone())),
    );

    siv.set_user_data(state);

    siv.add_global_callback(Key::Left, |s| {
        log::info!("LEFT");
        let state: &mut State = s.user_data().unwrap();
        if let Some(new_score) = update_screen(Some(-1), &mut state.computer, &mut state.screen) {
            state.score.set_content(format!("{}", new_score));
        }
        state.content.set_content(state.screen.render());
    });
    siv.add_global_callback(Key::Down, |s| {
        log::info!("DOWN");
        let state: &mut State = s.user_data().unwrap();
        if let Some(new_score) = update_screen(Some(0), &mut state.computer, &mut state.screen) {
            state.score.set_content(format!("{}", new_score));
        }
        state.content.set_content(state.screen.render());
    });
    siv.add_global_callback(Key::Right, |s| {
        log::info!("RIGHT");
        let state: &mut State = s.user_data().unwrap();
        if let Some(new_score) = update_screen(Some(1), &mut state.computer, &mut state.screen) {
            state.score.set_content(format!("{}", new_score));
        }
        state.content.set_content(state.screen.render());
    });

    siv.run();
}

fn update_screen(
    input: Option<isize>,
    computer: &mut IntcodeComputer,
    screen: &mut Screen,
) -> Option<isize> {
    let mut output = vec![];
    let mut score = None;
    if let Some(i) = input {
        computer.add_input(i);
    }
    loop {
        match computer.run_until_io() {
            PauseReason::Output(v) => output.push(v),
            PauseReason::Halt | PauseReason::Input => break,
        }
    }

    log::info!("Got {} output chunks", output.len() / 3);

    for chunk in output.chunks(3) {
        if let &[x, y, symbol] = chunk {
            if x == -1 && y == 0 {
                score = Some(symbol);
            } else {
                assert!(x >= 0);
                assert!(y >= 0);
                screen.set(x as usize, y as usize, symbol.into());
            }
        } else {
            panic!("wrong number of outputs");
        }
    }

    score
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Symbol {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<isize> for Symbol {
    fn from(s: isize) -> Self {
        match s {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("Unknown symbol {}", s),
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(fmt, "  ")?,
            Self::Wall => write!(fmt, "██")?,
            Self::Block => write!(fmt, "[]")?,
            Self::Paddle => write!(fmt, "==")?,
            Self::Ball => write!(fmt, "()")?,
        };
        Ok(())
    }
}

struct Screen {
    data: Vec<Symbol>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, || Symbol::Empty);
        Self {
            data,
            width,
            height,
        }
    }

    fn render(&self) -> String {
        let mut buffer = String::new();
        for row in self.data.chunks(self.width) {
            for symbol in row {
                buffer += &format!("{}", symbol);
            }
            buffer += "\n";
        }
        buffer
    }

    fn set(&mut self, x: usize, y: usize, symbol: Symbol) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[x + self.width * y] = symbol;
    }
}
