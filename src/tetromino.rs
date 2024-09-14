use array2d::Array2D;
use femtovg::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Tetromino {
    pub(crate) piece: Piece,
    pub(crate) rotation: Rotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rotation {
    Up,
    Right,
    Down,
    Left,
}
impl Default for Rotation {
    fn default() -> Self {
        Self::Down
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Piece {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}
impl Piece {
    pub(crate) fn to_color(&self) -> Color {
        match *self {
            Piece::I => Color::rgb(0, 255, 255),
            Piece::O => Color::rgb(255, 255, 0),
            Piece::T => Color::rgb(128, 0, 128),
            Piece::S => Color::rgb(0, 255, 0),
            Piece::Z => Color::rgb(255, 0, 0),
            Piece::J => Color::rgb(0, 0, 255),
            Piece::L => Color::rgb(255, 165, 0),
        }
    }
}
impl Tetromino {
    pub(crate) fn to_color(&self) -> Color {
        self.piece.to_color()
    }
    pub(crate) fn to_blocks(&self) -> Array2D<bool> {
        match *self {
            // I Pieces
            Tetromino {
                rotation: Rotation::Up,
                piece: Piece::I,
            } => Array2D::from_rows(&[
                vec![false, false, false, false],
                vec![true, true, true, true],
                vec![false, false, false, false],
                vec![false, false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Right,
                piece: Piece::I,
            } => Array2D::from_rows(&[
                vec![false, false, true, false],
                vec![false, false, true, false],
                vec![false, false, true, false],
                vec![false, false, true, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Down,
                piece: Piece::I,
            } => Array2D::from_rows(&[
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, true, true, true],
                vec![false, false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Left,
                piece: Piece::I,
            } => Array2D::from_rows(&[
                vec![false, true, false, false],
                vec![false, true, false, false],
                vec![false, true, false, false],
                vec![false, true, false, false],
            ])
            .unwrap(),

            // J Pieces
            Tetromino {
                rotation: Rotation::Up,
                piece: Piece::J,
            } => Array2D::from_rows(&[
                vec![true, false, false],
                vec![true, true, true],
                vec![false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Right,
                piece: Piece::J,
            } => Array2D::from_rows(&[
                vec![false, true, true],
                vec![false, true, false],
                vec![false, true, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Down,
                piece: Piece::J,
            } => Array2D::from_rows(&[
                vec![false, false, false],
                vec![true, true, true],
                vec![false, false, true],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Left,
                piece: Piece::J,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![false, true, false],
                vec![true, true, false],
            ])
            .unwrap(),
            // L Pieces
            Tetromino {
                rotation: Rotation::Up,
                piece: Piece::L,
            } => Array2D::from_rows(&[
                vec![false, false, true],
                vec![true, true, true],
                vec![false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Right,
                piece: Piece::L,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![false, true, false],
                vec![false, true, true],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Down,
                piece: Piece::L,
            } => Array2D::from_rows(&[
                vec![false, false, false],
                vec![true, true, true],
                vec![true, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Left,
                piece: Piece::L,
            } => Array2D::from_rows(&[
                vec![true, true, false],
                vec![false, true, false],
                vec![false, true, false],
            ])
            .unwrap(),
            // S Pieces
            Tetromino {
                rotation: Rotation::Up,
                piece: Piece::S,
            } => Array2D::from_rows(&[
                vec![false, true, true],
                vec![true, true, false],
                vec![false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Right,
                piece: Piece::S,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![false, true, true],
                vec![false, false, true],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Down,
                piece: Piece::S,
            } => Array2D::from_rows(&[
                vec![false, false, false],
                vec![false, true, true],
                vec![true, true, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Left,
                piece: Piece::S,
            } => Array2D::from_rows(&[
                vec![true, false, false],
                vec![true, true, false],
                vec![false, true, false],
            ])
            .unwrap(),

            // T Pieces
            Tetromino {
                rotation: Rotation::Up,
                piece: Piece::T,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![true, true, true],
                vec![false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Right,
                piece: Piece::T,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![false, true, true],
                vec![false, true, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Down,
                piece: Piece::T,
            } => Array2D::from_rows(&[
                vec![false, false, false],
                vec![true, true, true],
                vec![false, true, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Left,
                piece: Piece::T,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![true, true, false],
                vec![false, true, false],
            ])
            .unwrap(),

            // Z Pieces
            Tetromino {
                rotation: Rotation::Up,
                piece: Piece::Z,
            } => Array2D::from_rows(&[
                vec![true, true, false],
                vec![false, true, true],
                vec![false, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Right,
                piece: Piece::Z,
            } => Array2D::from_rows(&[
                vec![false, false, true],
                vec![false, true, true],
                vec![false, true, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Down,
                piece: Piece::Z,
            } => Array2D::from_rows(&[
                vec![false, false, false],
                vec![true, true, false],
                vec![false, true, true],
            ])
            .unwrap(),
            Tetromino {
                rotation: Rotation::Left,
                piece: Piece::Z,
            } => Array2D::from_rows(&[
                vec![false, true, false],
                vec![true, true, false],
                vec![true, false, false],
            ])
            .unwrap(),
            Tetromino {
                rotation: _,
                piece: Piece::O,
            } => Array2D::from_rows(&[vec![true, true], vec![true, true]]).unwrap(),
        }
    }
}
