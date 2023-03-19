#[derive(Debug)]
pub struct Figure {
    pub width: u32,
    pub height: u32,
}

impl Figure {
    pub fn build(width: u32, height: u32) -> Result<Figure, &'static str> {
        if !(width > 0 && height > 0) {
            return Err("Missing arguments.");
        }
        Ok(Figure { width, height })
    }
    pub fn plot() -> () {
        // pass
        ()
    }
}

// TODO: add other structs/enums, ie buffer, shapes, points, lines etc
// Buffer could take canvas dimensions and flatten like line does

fn line(x0: i32, x1: i32, y0: i32, y1: i32, width: u32) -> Vec<i32> {
    let adx: i32 = ((x1 - x0).abs() + 1) << 1; // add one to increase line gradient
    let ady: i32 = ((y1 - y0).abs() + 1) << 1; // also multipy by two to avoid floats
    let sign_of_x: i32 = if (x1 - x0) > 0 { 1 } else { -1 };
    let sign_of_y: i32 = if (y1 - y0) > 0 { 1 } else { -1 };
    let mut v = Vec::new();
    let (mut x, mut y): (i32, i32) = (x0, y0);
    if adx > ady {
        let mut epsilon = (ady - adx) >> 1; // bit-shift
        while x != x1 + sign_of_x {
            // add sign_of_x to include last iter
            v.push(x + y + (x * (width as i32 - 1))); // flatten to 1D array indices
            epsilon += ady;
            if epsilon << 1 >= adx {
                y += sign_of_y;
                epsilon -= adx;
            }
            x += sign_of_x;
        }
    } else {
        let mut epsilon = (adx - ady) >> 1;
        while y != y1 + sign_of_y {
            v.push(x + y + (x * (width as i32 - 1))); // flatten to 1D array indices
            epsilon += adx;
            if epsilon << 1 >= ady {
                x += sign_of_x;
                epsilon -= ady;
            }
            y += sign_of_y;
        }
    }
    v
}

fn line_gpt(x0: i32, x1: i32, y0: i32, y1: i32, width: usize) -> Vec<usize> {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let mut x = x0;
    let mut y = y0;
    let mut v = Vec::new();
    let mut err = dx - dy;
    let (sign_x, sign_y) = (if x1 > x0 { 1 } else { -1 }, if y1 > y0 { 1 } else { -1 });

    while x != x1 || y != y1 {
        v.push((y * width as i32 + x) as usize);
        let err2 = err * 2;
        if err2 > -dy {
            err -= dy;
            x += sign_x;
        }
        if err2 < dx {
            err += dx;
            y += sign_y;
        }
    }

    v.push((y * width as i32 + x) as usize);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_figure() {
        let test_figure = Figure {
            width: 20,
            height: 10,
        };
        assert!(!(test_figure.is_empty()));
    }
}
