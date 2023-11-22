use super::{bounds::Bounds, point::Point, shape::Shape};

//drafted from the bottom left of the shape
const FLAT_SHAPE: [Point; 4] = [
    Point::new_point(0, 0),
    Point::new_point(1, 0),
    Point::new_point(2, 0),
    Point::new_point(3, 0),
];

const PLUS_SHAPE: [Point; 5] = [
    Point::new_point(0, 1),
    Point::new_point(1, 0),
    Point::new_point(1, 1),
    Point::new_point(1, 2),
    Point::new_point(2, 1),
];

const L_SHAPE: [Point; 5] = [
    Point::new_point(0, 0),
    Point::new_point(1, 0),
    Point::new_point(2, 0),
    Point::new_point(2, 1),
    Point::new_point(2, 2),
];

const VERT_SHAPE: [Point; 4] = [
    Point::new_point(0, 0),
    Point::new_point(0, 1),
    Point::new_point(0, 2),
    Point::new_point(0, 3),
];

const SQUARE_SHAPE: [Point; 4] = [
    Point::new_point(0, 0),
    Point::new_point(1, 0),
    Point::new_point(0, 1),
    Point::new_point(1, 1),
];

#[derive(Clone, Copy)]
pub enum Piece {
    Flat,
    Plus,
    L,
    Vert,
    Square,
}

impl Piece {
    pub fn points(&self) -> &[Point] {
        match self {
            Piece::Flat => &FLAT_SHAPE,
            Piece::Plus => &PLUS_SHAPE,
            Piece::L => &L_SHAPE,
            Piece::Vert => &VERT_SHAPE,
            Piece::Square => &SQUARE_SHAPE,
        }
    }

    pub fn bounds(&self) -> Bounds {
        match self {
            Piece::Flat => Bounds::zero_based(4, 1),
            Piece::Plus => Bounds::zero_based(3, 3),
            Piece::L => Bounds::zero_based(3, 3),
            Piece::Vert => Bounds::zero_based(1, 4),
            Piece::Square => Bounds::zero_based(2, 2),
        }
    }
}

pub struct PieceSpawner {
    current: Piece,
}

impl PieceSpawner {
    pub fn new() -> PieceSpawner {
        PieceSpawner {
            current: Piece::Flat,
        }
    }
}

impl Iterator for PieceSpawner {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.current {
            Piece::Flat => Piece::Plus,
            Piece::Plus => Piece::L,
            Piece::L => Piece::Vert,
            Piece::Vert => Piece::Square,
            Piece::Square => Piece::Flat,
        };

        let out = self.current;
        self.current = next;
        Some(out)
    }
}
