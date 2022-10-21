#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);

#[derive(Debug, PartialEq)]
struct Rectangle {
    corners: [Point; 4],
}

impl Rectangle {
    fn try_from(corners: [Point; 4]) -> Result<Self, ()> {
        // sort corners so that it contain points in order
        // first horizontal line then next horizontal line
        let mut corners = corners;
        corners.sort_unstable();
        // if contain duplicate corners then reject
        if (1..corners.len()).any(|i| corners[i..].contains(&corners[i - 1])) {
            return Err(());
        }
        // check if points construct a rectangle
        if !(corners[0].0 == corners[1].0
            && corners[2].0 == corners[3].0
            && (corners[0].1, corners[1].1) == (corners[2].1, corners[3].1))
        {
            return Err(());
        }

        Ok(Self { corners })
    }
}

#[derive(Debug)]
struct Diagram<'a> {
    input_lines: &'a [&'a str],
    points: Vec<Point>,
    rectangles: Vec<Rectangle>,
}

impl<'a> Diagram<'a> {
    fn new(input_lines: &'a [&str]) -> Self {
        let mut points = vec![];
        input_lines.iter().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, ch)| {
                if ch == '+' {
                    points.push(Point(i, j));
                }
            });
        });

        Self {
            input_lines,
            points,
            rectangles: vec![],
        }
    }

    fn is_connected(&self, p1: Point, p2: Point) -> bool {
        // horizontal line
        if p1.0 == p2.0 {
            if self.input_lines[p1.0][p1.1..p2.1 + 1]
                .chars()
                .all(|ch| ch == '-' || ch == '+')
            {
                return true;
            }
        }
        // vertical line
        if p1.1 == p2.1 {
            if self.input_lines[p1.0..p2.0 + 1].iter().all(|line| {
                let l = &line[p2.1..p2.1 + 1];
                l == "|" || l == "+"
            }) {
                return true;
            }
        }
        false
    }

    fn add_rectangle(&mut self, corners: [Point; 4]) {
        let rect = Rectangle::try_from(corners);
        if rect.is_err() {
            return;
        }
        let rect = rect.unwrap();
        // reject the rectangle if not connected
        if !(self.is_connected(rect.corners[0], rect.corners[1])
            && self.is_connected(rect.corners[2], rect.corners[3])
            && self.is_connected(rect.corners[0], rect.corners[2])
            && self.is_connected(rect.corners[1], rect.corners[3]))
        {
            return;
        }

        // if the rectangle is already counted then reject
        if self.rectangles.contains(&rect) {
            return;
        }
        self.rectangles.push(rect);
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let mut diagram = Diagram::new(lines);
    let len = diagram.points.len();
    (0..len).for_each(|x| {
        (x + 1..len).for_each(|y| {
            (y + 1..len).for_each(|z| {
                (z + 1..len).for_each(|w| {
                    let p1 = diagram.points[x];
                    let p2 = diagram.points[y];
                    let p3 = diagram.points[z];
                    let p4 = diagram.points[w];
                    diagram.add_rectangle([p1, p2, p3, p4]);
                });
            });
        });
    });

    diagram.rectangles.len() as u32
}
