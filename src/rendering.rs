use crate::tetromino::Tetromino;
use crate::{piece_is_legal, AppState, Rotation, COLS, ROWS};
use femtovg::{Align, Baseline, Canvas, Color, Paint, Path, Renderer};
use glutin::{context::PossiblyCurrentContext, surface::{Surface, WindowSurface, GlSurface}};
use num::NumCast;
use std::cmp::min_by;
use winit::{window::Window, dpi::PhysicalSize};

pub(crate) fn cell_size(size: PhysicalSize<u32>) -> f32 {
    min_by(
        size.height as f32 / (ROWS + 1) as f32,
        size.width as f32 / (COLS + 1) as f32,
        |x, y| x.partial_cmp(y).expect("NaN in cell size calculation"),
    ) * 0.95
}

#[inline]
pub(crate) fn index_to_grid<T: NumCast>(row: T, col: T, board_info: BoardInfo) -> (f32, f32) {
    let BoardInfo {
        cell_size,
        board_left,
        board_top,
        ..
    } = board_info;
    (
        <f32 as NumCast>::from(col).unwrap() * cell_size + board_left,
        <f32 as NumCast>::from(row).unwrap() * cell_size + board_top,
    )
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BoardInfo {
    cell_size: f32,
    board_left: f32,
    board_top: f32,
    line_width: f32,
    font_size: f32,
    board_size: PhysicalSize<u32>,
}
pub(crate) fn board_location(size: PhysicalSize<u32>, cell_size: f32) -> (f32, f32) {
    let board_left = (size.width as f32 - cell_size * COLS as f32) / 2.0;
    let board_top = (size.height as f32 - cell_size * ROWS as f32) / 1.5;
    (board_left, board_top)
}
pub(crate) fn render<T: Renderer>(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<T>,
    state: &mut AppState,
) {
    let size = window.inner_size();
    let cell_size = cell_size(size);
    let (board_left, board_top) = board_location(size, cell_size);
    let line_width = cell_size / 20.0;
    let board_info = BoardInfo {
        cell_size,
        board_left,
        board_top,
        line_width,
        font_size: cell_size,
        board_size: size,
    };

    canvas.set_size(size.width, size.height, window.scale_factor() as f32);
    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    // Draw Board
    draw_board(state, board_info, canvas);

    // Draw Piece
    let paint = Paint::color(state.piece.to_color());
    draw_piece(
        state.piece,
        state.location,
        board_info,
        canvas,
        &paint,
        PieceType::Normal,
    );

    // Draw Ghost Piece
    let paint = &Paint::color(state.piece.to_color()).with_line_width(line_width * 2.0);
    let row = ghost_location(state);
    draw_piece(
        state.piece,
        (state.location.0, row),
        board_info,
        canvas,
        paint,
        PieceType::Ghost,
    );
    // Draw Held Piece
    if let Some(held) = state.held {
        let paint = Paint::color(held.to_color());
        draw_piece(
            Tetromino {
                piece: held,
                rotation: Rotation::Up,
            },
            (-5, 0),
            board_info,
            canvas,
            &paint,
            PieceType::Held,
        );
        draw_held_text(board_info, canvas);
    }

    draw_grid(board_info, canvas);

    if state.game_over {
        draw_game_over(board_info, canvas, state);
    } else {
        draw_score_text(board_info, state, canvas);
    }

    // Display to screen
    canvas.flush();
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}

fn draw_game_over<T: Renderer>(
    board_info: BoardInfo,
    canvas: &mut Canvas<T>,
    state: &mut AppState,
) {
    let size = board_info.board_size;
    let (width, height) = (size.width as f32, size.height as f32);
    let mut path = Path::new();
    path.rect(width / 3.0, height / 3.0, width / 3.0, height / 3.0);

    canvas.fill_path(&path, &Paint::color(Color::black()));
    canvas.stroke_path(&path, &Paint::color(Color::white()));

    let mut paint = Paint::color(Color::white())
        .with_font_size(board_info.font_size)
        .with_text_align(Align::Center)
        .with_text_baseline(Baseline::Bottom);

    let score = format!("Score: {}", state.score);

    canvas
        .fill_text(width / 2.0, height / 2.0, "Game Over", &paint)
        .expect("Unable to display game over top text");
    
    paint.set_text_baseline(Baseline::Top);

    canvas
        .fill_text(width / 2.0, height / 2.0, score, &paint)
        .expect("Unable to display game over score text");
}
fn ghost_location(state: &mut AppState) -> isize {
    let location = state.location;
    while piece_is_legal(state) {
        state.location.1 += 1;
    }
    let ret = state.location.1 - 1;
    state.location = location;
    ret
}

fn draw_board<T: Renderer>(state: &mut AppState, board_info: BoardInfo, canvas: &mut Canvas<T>) {
    let BoardInfo {
        cell_size,
        line_width,
        ..
    } = board_info;
    state
        .board
        .enumerate_row_major()
        .for_each(|((row, col), el)| {
            if let Some(color) = el {
                let (x, y) = index_to_grid(row, col, board_info);
                let mut path = Path::new();
                path.rect(
                    x + line_width / 2.0,
                    y + line_width / 2.0,
                    cell_size - line_width,
                    cell_size - line_width,
                );
                canvas.fill_path(&path, &Paint::color(*color));
            }
        });
}

fn draw_score_text<T: Renderer>(board_info: BoardInfo, state: &mut AppState, canvas: &mut Canvas<T>) {
    let (a, b) = index_to_grid(0, COLS / 2, board_info);
    let paint = &Paint::color(Color::white())
        .with_font_size(board_info.font_size)
        .with_text_align(Align::Center)
        .with_text_baseline(Baseline::Bottom);
    let text = format!("Score: {}", state.score);

    canvas
        .fill_text(a, b, text, paint)
        .expect("Could not display score");
}
fn draw_held_text<T: Renderer>(board_info: BoardInfo, canvas: &mut Canvas<T>) {
    let (a, b) = index_to_grid(0, -3, board_info);
    let paint = &Paint::color(Color::white())
        .with_font_size(board_info.font_size)
        .with_text_align(Align::Center)
        .with_text_baseline(Baseline::Bottom);

    canvas
        .fill_text(a, b, "Held Piece:", paint)
        .expect("Could not display score");
}


fn draw_grid<T: Renderer>(board_info: BoardInfo, canvas: &mut Canvas<T>) {
    let BoardInfo { cell_size, .. } = board_info;
    let line_paint = Paint::color(Color::rgb(127, 127, 127)).with_line_width(cell_size / 20.0);
    for x in 0..=ROWS {
        let mut path = Path::new();
        let (a, b) = index_to_grid(x, 0, board_info);
        let (c, d) = index_to_grid(x, COLS, board_info);
        path.move_to(a, b);
        path.line_to(c, d);
        canvas.stroke_path(&path, &line_paint);
    }
    for x in 0..=COLS {
        let mut path = Path::new();
        let (a, b) = index_to_grid(0, x, board_info);
        let (c, d) = index_to_grid(ROWS, x, board_info);
        path.move_to(a, b);
        path.line_to(c, d);
        canvas.stroke_path(&path, &line_paint);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceType {
    Normal,
    Held,
    Ghost,
}
fn draw_piece<T: Renderer>(
    piece: Tetromino,
    location: (isize, isize),
    board_info: BoardInfo,
    canvas: &mut Canvas<T>,
    paint: &Paint,
    piece_type: PieceType,
) {
    let BoardInfo {
        cell_size,
        line_width,
        ..
    } = board_info;
    let line_width = match piece_type {
        PieceType::Held => line_width / 2.0,
        _ => line_width,
    };
    let (piece_col, piece_row) = location;
    piece
        .to_blocks()
        .enumerate_row_major()
        .filter_map(|((row, col), x)| match *x {
            true => Some((row as isize + piece_row, col as isize + piece_col)),
            false => None,
        })
        .for_each(|(row, col)| {
            let mut path = Path::new();
            if row < 0 || row >= ROWS as isize {
                return;
            }
            let (x, y) = index_to_grid(row, col, board_info);

            path.rect(
                x + line_width / 2.0,
                y + line_width / 2.0,
                cell_size - line_width,
                cell_size - line_width,
            );
            match piece_type {
                PieceType::Ghost => canvas.stroke_path(&path, paint),
                _ => canvas.fill_path(&path, paint),
            }
        });
}
