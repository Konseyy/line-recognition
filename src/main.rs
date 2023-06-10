use image::{DynamicImage, GenericImageView, Rgba};
use rand::Rng;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
#[derive(Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn process_image(input_path: &str) -> Option<(u32, u32, DynamicImage)> {
    let img = image::open(&Path::new(input_path));
    if img.is_err() {
        println!("Error: {}", img.err().unwrap());
        return None;
    }

    let width = img.as_ref().unwrap().width();
    let height = img.as_ref().unwrap().height();

    return Some((width, height, img.as_ref().unwrap().clone()));
}

fn get_neighbors(x: u32, y: u32, width: u32, height: u32) -> Vec<(u32, u32)> {
    let ix = x as i32;
    let iy = y as i32;
    let mut neighbor_coordinates = Vec::new();
    let neighbors = [(ix - 1, iy), (ix + 1, iy), (ix, iy - 1), (ix, iy + 1)];
    neighbors.iter().for_each(|coord| {
        if coord.0 < 0 || coord.0 >= width as i32 {
            return;
        }
        if coord.1 < 0 || coord.1 >= height as i32 {
            return;
        }
        neighbor_coordinates.push((coord.0 as u32, coord.1 as u32));
    });
    return neighbor_coordinates;
}
fn main() {
    let mut src_img_path = String::new();

    print!("Please enter image path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut src_img_path)
        .expect("Did not enter a correct string");
    if let Some('\n') = src_img_path.chars().next_back() {
        src_img_path.pop();
    }
    if let Some('\r') = src_img_path.chars().next_back() {
        src_img_path.pop();
    }
    let img_info_result = process_image(&src_img_path);
    if img_info_result.is_none() {
        println!("Could not process image {}", src_img_path);
        return;
    }
    let img_info = img_info_result.unwrap();
    let mut new_img = image::ImageBuffer::new(img_info.0 * 2, img_info.1);
    for x in 0..img_info.0 * 2 {
        for y in 0..img_info.1 {
            new_img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }
    let mut black_pixels = Vec::new();
    for pixel in img_info.2.pixels() {
        if pixel.2[0] == 0 && pixel.2[1] == 0 && pixel.2[2] == 0 {
            black_pixels.push(Point {
                x: pixel.0,
                y: pixel.1,
            });
        }
        new_img.put_pixel(pixel.0, pixel.1, pixel.2);
        new_img.put_pixel(pixel.0 + img_info.0, pixel.1, pixel.2);
    }
    while black_pixels.len() != 0 {
        let line_r: u8 = rand::thread_rng().gen_range(20..=240);
        let line_g: u8 = rand::thread_rng().gen_range(20..=240);
        let line_b: u8 = rand::thread_rng().gen_range(20..=240);
        let mut line_points = Vec::new();
        let p_optional = black_pixels.iter().position(|&p| {
            let neighbors = get_neighbors(p.x, p.y, img_info.0, img_info.1);
            let black_neighbors = neighbors
                .iter()
                .filter(|&n| {
                    let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                    return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
                })
                .collect::<Vec<&(u32, u32)>>();
            if black_neighbors.len() <= 2 {
                return true;
            }
            return false;
        });
        if p_optional.is_none() {
            break;
        }
        let p = black_pixels.remove(p_optional.unwrap());
        new_img.put_pixel(p.x + img_info.0, p.y, Rgba([255, 255, 255, 255]));
        line_points.push(p);
        let p_neighbors = get_neighbors(p.x, p.y, img_info.0, img_info.1);
        let p_black_neighbors = p_neighbors
            .iter()
            .filter(|&n| {
                let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
            })
            .collect::<Vec<&(u32, u32)>>();
        if p_black_neighbors.len() == 0 {
            line_points.iter().for_each(|p| {
                new_img.put_pixel(p.x + img_info.0, p.y, Rgba([line_r, line_g, line_b, 255]));
            });
            continue;
        }
        let mut p1 = *p_black_neighbors[0];
        line_points.push(Point { x: p1.0, y: p1.1 });
        new_img.put_pixel(p1.0 + img_info.0, p1.1, Rgba([255, 255, 255, 255]));
        black_pixels.remove(
            black_pixels
                .iter()
                .position(|&p| p.x == p1.0 && p.y == p1.1)
                .unwrap(),
        );
        let p1_neighbors = &get_neighbors(p1.0, p1.1, img_info.0, img_info.1);
        let p1_black_neighbors = p1_neighbors
            .iter()
            .filter(|&n| {
                let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
            })
            .collect::<Vec<&(u32, u32)>>();
        let mut p1_black_neighbor_count = p1_black_neighbors.len();
        while p1_black_neighbor_count == 1 {
            let temp_p1_neighbors = get_neighbors(p1.0, p1.1, img_info.0, img_info.1);
            let temp_p1_black_neighbors = temp_p1_neighbors
                .iter()
                .filter(|&n| {
                    let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                    return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
                })
                .collect::<Vec<&(u32, u32)>>();
            let qx = temp_p1_black_neighbors[0].0;
            let qy = temp_p1_black_neighbors[0].1;
            line_points.push(Point { x: qx, y: qy });
            black_pixels.remove(
                black_pixels
                    .iter()
                    .position(|&p| p.x == qx && p.y == qy)
                    .unwrap(),
            );
            new_img.put_pixel(qx + img_info.0, qy, Rgba([255, 255, 255, 255]));
            p1 = (qx, qy);
            let new_p1_neighbors = &get_neighbors(p1.0, p1.1, img_info.0, img_info.1);
            let new_p1_black_neighbors = new_p1_neighbors
                .iter()
                .filter(|&n| {
                    let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                    return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
                })
                .collect::<Vec<&(u32, u32)>>();
            p1_black_neighbor_count = new_p1_black_neighbors.len();
        }
        let p_coords = &(p.x, p.y);
        p1 = *p_coords;
        let p1_neighbors = &get_neighbors(p1.0, p1.1, img_info.0, img_info.1);
        let p1_black_neighbors = p1_neighbors
            .iter()
            .filter(|&n| {
                let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
            })
            .collect::<Vec<&(u32, u32)>>();
        let mut p1_black_neighbor_count = p1_black_neighbors.len();
        while p1_black_neighbor_count == 1 {
            let temp_p1_neighbors = get_neighbors(p1.0, p1.1, img_info.0, img_info.1);
            let temp_p1_black_neighbors = temp_p1_neighbors
                .iter()
                .filter(|&n| {
                    let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                    return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
                })
                .collect::<Vec<&(u32, u32)>>();
            let qx = temp_p1_black_neighbors[0].0;
            let qy = temp_p1_black_neighbors[0].1;
            line_points.push(Point { x: qx, y: qy });
            black_pixels.remove(
                black_pixels
                    .iter()
                    .position(|&p| p.x == qx && p.y == qy)
                    .unwrap(),
            );
            new_img.put_pixel(qx + img_info.0, qy, Rgba([255, 255, 255, 255]));
            p1 = (qx, qy);
            let new_p1_neighbors = &get_neighbors(p1.0, p1.1, img_info.0, img_info.1);
            let new_p1_black_neighbors = new_p1_neighbors
                .iter()
                .filter(|&n| {
                    let pixel = new_img.get_pixel(n.0 + img_info.0, n.1);
                    return pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0;
                })
                .collect::<Vec<&(u32, u32)>>();
            p1_black_neighbor_count = new_p1_black_neighbors.len();
        }
        line_points.iter().for_each(|p| {
            new_img.put_pixel(p.x + img_info.0, p.y, Rgba([line_r, line_g, line_b, 255]));
        });
    }
    fs::create_dir_all("images").unwrap();
    new_img.save("images/comparison.png").unwrap();
}
