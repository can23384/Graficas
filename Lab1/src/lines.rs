use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vertex,
    end: Vertex,
) {
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as u32) < framebuffer.width && (y0 as u32) < framebuffer.height {
            framebuffer.set_pixel(x0 as u32, y0 as u32);
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}


pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vertex]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() - 1 {
        line(framebuffer, points[i], points[i + 1]);
    }

    line(framebuffer, points[points.len() - 1], points[0]);
}


pub fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vertex]) {
    if vertices.len() < 3 {
        return; 
    }

    let min_y = vertices.iter().map(|v| v.y).min().unwrap();
    let max_y = vertices.iter().map(|v| v.y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()]; 

            if v1.y == v2.y {
                continue;
            }

            let (v_top, v_bottom) = if v1.y < v2.y { (v1, v2) } else { (v2, v1) };
            if y >= v_top.y && y < v_bottom.y {
                let dx = v_bottom.x - v_top.x;
                let dy = v_bottom.y - v_top.y;

                let x = v_top.x + dx * (y - v_top.y) / dy;
                intersections.push(x);
            }
        }

        intersections.sort();

        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i];
            let x_end = intersections[i + 1];

            for x in x_start..=x_end {
                if x >= 0 && y >= 0 && (x as u32) < framebuffer.width && (y as u32) < framebuffer.height {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }

            i += 2;
        }
    }
}