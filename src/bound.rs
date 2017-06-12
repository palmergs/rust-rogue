pub struct Bound {
    pub min: Point,
    pub max: Point,
}

pub struct Point {
    pub col: i32,
    pub row: i32,
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

impl Point {
    pub fn offset(&self, offset: &Point) -> Point {
        Point { col: self.col + offset.col, row: self.row + offset.row }
    }
}


impl Bound {
    pub fn contains(&self, point: Point) -> Contains {
        if point.col >= self.min.col 
                && point.col <= self.max.col 
                && point.row >= self.min.row 
                && point.row <= self.max.row {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}
