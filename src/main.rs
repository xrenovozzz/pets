//! Генератор изображения треугольника Серпинского
//!
//! Пример использования Rust для создания PNG-изображения

extern crate image;
extern crate rand;

use rand::Rng;

/// Точки, используемые для построения треугольника и нанесения точек на холст
pub struct Point {
    x: u32,
    y: u32,
}

/// Ширина PNG-изображения
pub const WIDTH:  u32 = 800;
/// Высота PNG-изображения
pub const HEIGHT: u32 = 600;

/// Основная программа
pub fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8]) // Чёрный цвет для начальной точки
        } else {
            image::Luma([255u8]) // Белый цвет для остальных точек
        }
    });

    let mut cnt: u32 = 1_000_000; // Количество итераций для построения точек

    let pts: [Point; 3] = [
        Point {x: WIDTH / 2, y: 0},     // Верхняя вершина треугольника
        Point {x: 0, y: HEIGHT},        // Левая нижняя вершина
        Point {x: WIDTH, y: HEIGHT},    // Правая нижняя вершина
    ];

    let mut p = Point { x: rand::thread_rng().gen_range(0..WIDTH),   // Случайная начальная точка
                        y: rand::thread_rng().gen_range(0..HEIGHT),
    };

    let pixel = img[(0, 0)]; // Цвет пикселя (изначально чёрный)
    while cnt > 0 {
        cnt = cnt - 1; // Уменьшение счётчика итераций
        let num = rand::thread_rng().gen_range(0..3); // Случайный выбор вершины (0, 1 или 2)
        p.x = (p.x + pts[num].x) / 2; // Новая x-координата — середина между текущей точкой и вершиной
        p.y = (p.y + pts[num].y) / 2; // Новая y-координата — середина между текущей точкой и вершиной
        img.put_pixel(p.x, p.y, pixel); // Установка пикселя на изображении
    }

    img.save("tri.png").unwrap(); // Сохранение изображения в файл "tri.png"
} 

