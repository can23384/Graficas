mod framebuffer;
mod lines;
mod vertex;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use lines::*;
use vertex::Vertex;

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);
    framebuffer.set_background_color(Color::new(30, 30, 60, 255));
    framebuffer.clear();

    //poligono 1
    let poly1 = vec![
        Vertex::new(165, 380), Vertex::new(185, 360), Vertex::new(180, 330),
        Vertex::new(207, 345), Vertex::new(233, 330), Vertex::new(230, 360),
        Vertex::new(250, 380), Vertex::new(220, 385), Vertex::new(205, 410),
        Vertex::new(193, 383),
    ];
    framebuffer.set_current_color(Color::YELLOW);
    fill_polygon(&mut framebuffer, &poly1);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly1);

    // Polígono 2
    let poly2 = vec![
        Vertex::new(321, 335), Vertex::new(288, 286),
        Vertex::new(339, 251), Vertex::new(374, 302),
    ];
    framebuffer.set_current_color(Color::BLUE);
    fill_polygon(&mut framebuffer, &poly2);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly2);

    // Polígono 3
    let poly3 = vec![
        Vertex::new(377, 249), Vertex::new(411, 197), Vertex::new(436, 249),
    ];
    framebuffer.set_current_color(Color::RED);
    fill_polygon(&mut framebuffer, &poly3);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly3);

    // Polígono 4
    let poly4 = vec![
        Vertex::new(413, 177), Vertex::new(448, 159), Vertex::new(502, 88),
        Vertex::new(553, 53), Vertex::new(535, 36), Vertex::new(676, 37),
        Vertex::new(660, 52), Vertex::new(750, 145), Vertex::new(761, 179),
        Vertex::new(672, 192), Vertex::new(659, 214), Vertex::new(615, 214),
        Vertex::new(632, 230), Vertex::new(580, 230), Vertex::new(597, 215),
        Vertex::new(552, 214), Vertex::new(517, 144), Vertex::new(466, 180),
    ];
    framebuffer.set_current_color(Color::GREEN);
    fill_polygon(&mut framebuffer, &poly4);
    framebuffer.set_current_color(Color::WHITE);
    draw_polygon(&mut framebuffer, &poly4);

    // Polígono 5
    let poly5 = vec![
        Vertex::new(682, 175), Vertex::new(708, 120),
        Vertex::new(735, 148), Vertex::new(739, 170),
    ];
    framebuffer.set_current_color(Color::new(30, 30, 60, 255));
    fill_polygon(&mut framebuffer, &poly5);
    framebuffer.set_current_color(Color::new(30, 30, 60, 255));
    draw_polygon(&mut framebuffer, &poly5);

    framebuffer.render_to_file("out.bmp");
    framebuffer.render_to_file("out.png");
    println!("Polígonos guardados como 'out.bmp'");
}