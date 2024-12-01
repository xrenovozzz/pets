extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;

// Точка для построения треугольника
pub struct Point {
    x: u32,
    y: u32,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    // Создаем буфер изображения
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let mut cnt: u32 = 10_000;

    // Определяем точки треугольника
    let pts: [Point; 3] = [
        Point { x: WIDTH / 2, y: 0 },
        Point { x: 0, y: HEIGHT },
        Point { x: WIDTH, y: HEIGHT },
    ];

    let mut num: usize;
    let mut p = Point { x: 350, y: 350 };
    let pixel = image::Luma([0u8]);

    // Генерация точек по методу хаотических треугольников
    while cnt > 0 {
        cnt -= 1;
        num = rand::thread_rng().gen_range(0..3);
        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.y + pts[num].y) / 2;
        img.put_pixel(p.x, p.y, pixel);
    }

    // Сохраняем изображение
    let ref mut fout = File::create(&Path::new("tri.png")).unwrap();
    img.save(fout).unwrap();
}
