mod rendering;
mod tetromino;
mod window;

use array2d::Array2D;
use femtovg::{renderer::OpenGl, Canvas, Color};
use glutin::{
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::ControlFlow::WaitUntil;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use crate::{
    rendering::render,
    tetromino::{Piece, Rotation, Tetromino},
    window::{create_canvas, create_window},
};

const ROWS: usize = 20;
const COLS: usize = 10;

fn handle_keyboard_input(event: KeyEvent, state: &mut AppState, window: &Window) {
    if state.game_over || event.state == ElementState::Released {
        return;
    }
    match event.logical_key {
        Key::Named(x) => match x {
            NamedKey::ArrowUp => {
                rotate_piece(state, MovementType::Right);
                window.request_redraw();
            }
            NamedKey::ArrowLeft => {
                move_piece(state, MovementType::Left);
                window.request_redraw();
            }
            NamedKey::ArrowRight => {
                move_piece(state, MovementType::Right);
                window.request_redraw();
            }
            NamedKey::ArrowDown => {
                move_down(state);
                window.request_redraw();
            }
            NamedKey::Space => {
                hard_drop(state);
                window.request_redraw();
            }
            NamedKey::Control => {
                rotate_piece(state, MovementType::Left);
                window.request_redraw();
            }
            NamedKey::Shift => {
                hold_piece(state);
                window.request_redraw();
            }
            _ => {}
        },
        Key::Character(x) => match x.as_str() {
            "x" => {
                rotate_piece(state, MovementType::Right);
                window.request_redraw();
            }
            "z" => {
                rotate_piece(state, MovementType::Left);
                window.request_redraw();
            }
            "c" => {
                hold_piece(state);
                window.request_redraw();
            }
            _ => {}
        },
        _ => {}
    }
}

fn hard_drop(state: &mut AppState) {
    while !move_down(state) {}
}

fn hold_piece(state: &mut AppState) {
    if state.can_hold {
        let held = match state.held.take() {
            Some(x) => x,
            None => randomize_piece(state),
        };
        state.held = Some(state.piece.piece);
        state.piece.piece = held;
        state.piece.rotation = Rotation::default();
        state.location = (3, -3);
        state.can_hold = false;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementType {
    Right,
    Left,
}
fn piece_is_legal(state: &mut AppState) -> bool {
    state
        .piece
        .to_blocks()
        .enumerate_row_major()
        .filter_map(|(el, x)| if *x { Some(el) } else { None })
        .all(|(row, col)| {
            let row = row as isize + state.location.1;
            let col = col as isize + state.location.0;

            if col < 0 || col >= COLS as isize {
                return false;
            }
            if row >= ROWS as isize {
                return false;
            }
            if row < 0 {
                return true;
            }
            match state.board.get(row as usize, col as usize) {
                Some(Some(_)) => false,
                Some(None) => true,
                None => true,
            }
        })
}
fn move_piece(state: &mut AppState, movement_type: MovementType) {
    match movement_type {
        MovementType::Left => {
            state.location.0 -= 1;
            if !piece_is_legal(state) {
                state.location.0 += 1;
            }
        }
        MovementType::Right => {
            state.location.0 += 1;
            if !piece_is_legal(state) {
                state.location.0 -= 1;
            }
        }
    }
}
fn rotate_piece(state: &mut AppState, rotation_type: MovementType) {
    let old = state.piece.rotation;
    state.piece.rotation = match rotation_type {
        MovementType::Right => match state.piece.rotation {
            Rotation::Up => Rotation::Right,
            Rotation::Right => Rotation::Down,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up,
        },
        MovementType::Left => match state.piece.rotation {
            Rotation::Up => Rotation::Left,
            Rotation::Left => Rotation::Down,
            Rotation::Down => Rotation::Right,
            Rotation::Right => Rotation::Up,
        },
    };
    if !piece_is_legal(state) {
        state.piece.rotation = old;
    }
}

#[derive(Debug, Clone)]
struct AppState {
    board: Array2D<Option<Color>>,
    piece: Tetromino,
    location: (isize, isize),
    bag: Vec<Piece>,
    game_over: bool,
    score: u64,
    rng: ThreadRng,
    held: Option<Piece>,
    can_hold: bool,
}
impl AppState {
    fn new() -> Self {
        let mut tmp = AppState {
            board: Array2D::filled_with(None, ROWS, COLS),
            piece: Tetromino {
                piece: Piece::I,
                rotation: Rotation::default(),
            },
            location: (3, -3),
            bag: vec![
                Piece::I,
                Piece::J,
                Piece::L,
                Piece::O,
                Piece::S,
                Piece::Z,
                Piece::T,
            ],
            game_over: false,
            score: 0,
            rng: thread_rng(),
            held: None,
            can_hold: true,
        };
        tmp.piece.piece = randomize_piece(&mut tmp);
        tmp
    }
}

const FONT: &[u8; 834452] = include_bytes!("font/Times New Roman.ttf");
fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop");
    let (context, gl_display, window, surface) = create_window(&event_loop);
    let mut canvas = create_canvas(gl_display, &window);

    let mut state = AppState::new();

    canvas
        .add_font_mem(FONT)
        .expect("Unable to load font from memory");

    render(&context, &surface, &window, &mut canvas, &mut state);
    wait(&event_loop);
    let mut game = Game {
        state: AppState::new(),
        window,
        context,
        surface,
        canvas,
    };
    event_loop.run_app(&mut game).unwrap();
}
trait SetControlFlow {
    fn set_control_flow(&self, control_flow: ControlFlow);
}
impl<T> SetControlFlow for EventLoop<T> {
    fn set_control_flow(&self, flow: ControlFlow) {
        self.set_control_flow(flow);
    }
}
impl SetControlFlow for ActiveEventLoop {
    fn set_control_flow(&self, flow: ControlFlow) {
        self.set_control_flow(flow);
    }
}
fn wait<T>(event_loop: &T)
where
    T: SetControlFlow,
{
    event_loop.set_control_flow(WaitUntil(Instant::now() + Duration::from_millis(333)));
}

struct Game {
    state: AppState,
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    canvas: Canvas<OpenGl>,
}
impl ApplicationHandler for Game {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::ResumeTimeReached { .. } => {
                step_game(event_loop, &mut self.state);
                self.window.request_redraw();
            }
            _ => {}
        };
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        handle_window_event(
            &mut self.state,
            event,
            &self.context,
            &self.surface,
            &self.window,
            &mut self.canvas,
            event_loop,
        )
    }
}
fn step_game(event_loop: &ActiveEventLoop, state: &mut AppState) {
    if !state.game_over {
        move_down(state);
        wait(event_loop);
    }
}
fn move_down(state: &mut AppState) -> bool {
    state.location.1 += 1;
    if !piece_is_legal(state) {
        state.location.1 -= 1;
        if lock_piece(state) {
            return true;
        }
        state.location = (3, -3);
        state.piece.piece = randomize_piece(state);
        state.piece.rotation = Rotation::default();
        return true;
    }
    false
}

fn lock_piece(state: &mut AppState) -> bool {
    let mut count = 0;
    state.can_hold = true;
    let location = state.location;
    if state
        .piece
        .to_blocks()
        .enumerate_row_major()
        .filter_map(|((row, col), x)| {
            if *x {
                Some((row as isize + location.1, col as isize + location.0))
            } else {
                None
            }
        })
        .any(|(row, col)| {
            if row < 0 {
                state.game_over = true;
                return true;
            }
            state
                .board
                .set(row as usize, col as usize, Some(state.piece.to_color()))
                .expect("Unable to set block");
            count += clear_lines(state);
            false
        })
    {
        return true;
    }
    state.score += match count {
        0 => 0,
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => unreachable!(),
    };
    false
}

fn clear_lines(state: &mut AppState) -> u8 {
    let mut count = 0;
    let rows = {
        let mut tmp = state
            .board
            .rows_iter()
            .filter_map(|x| {
                if x.clone().all(|y| y.is_some()) {
                    count += 1;
                    None
                } else {
                    Some(x.copied().collect::<Vec<_>>())
                }
            })
            .collect::<Vec<_>>();
        while tmp.len() < ROWS {
            tmp.insert(0, vec![None; COLS])
        }
        tmp
    };
    state.board = Array2D::from_rows(&rows).unwrap();
    count
}
fn randomize_piece(state: &mut AppState) -> Piece {
    match state.bag.choose(&mut state.rng) {
        Some(x) => {
            let tmp = *x;
            state.bag.retain(|x| *x != tmp);
            tmp
        }
        None => {
            state.bag = vec![
                Piece::I,
                Piece::J,
                Piece::L,
                Piece::O,
                Piece::S,
                Piece::Z,
                Piece::T,
            ];
            match state.bag.choose(&mut thread_rng()) {
                Some(x) => {
                    let tmp = *x;
                    state.bag.retain(|x| *x != tmp);
                    tmp
                }
                None => {
                    unreachable!()
                }
            }
        }
    }
}

fn handle_window_event(
    state: &mut AppState,
    event: WindowEvent,
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<OpenGl>,
    event_loop: &ActiveEventLoop,
) {
    match event {
        WindowEvent::RedrawRequested => {
            render(&context, &surface, &window, canvas, state);
        }
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::KeyboardInput {
            device_id: _,
            event,
            is_synthetic: _,
        } => handle_keyboard_input(event, state, window),
        _ => {}
    }
}
