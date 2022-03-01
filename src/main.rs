mod vec3;
use std::{
    borrow::BorrowMut,
    thread,
    time::{self, Duration},
};

use lazy_static::lazy_static;
use vec3::Vec3;

lazy_static! {
    static ref PIXELS: Vec<char> = " .:+|0#".chars().collect();
}
const WEIGHT: u32 = 80;
const HEIGHT: u32 = 40;

// 模拟光线行进
pub fn raymarch(framebuffer: &mut Vec<Vec<char>>) {
    let mut pxl = PIXELS.get(1).unwrap().to_owned();
    // 光线从一个像素点射出
    for y in 0..HEIGHT {
        for x in 0..WEIGHT {
            // 摄像机(光源)位置开始时位于0,0,-3
            let mut pos = Vec3::new(0.0, 0.0, -3.0);
            // 目标点(起始就是屏幕上每个像素的坐标，不过需要经过变换), x, y分别归一化之后减去0.5做到将坐标轴原点偏移到
            let target = Vec3::new(
                (x as f64 / WEIGHT as f64) - 0.5,
                ((y as f64 / HEIGHT as f64) - 0.5) * (HEIGHT as f64 / WEIGHT as f64) * 1.5,
                -1.5,
            );
            // 光线的向量
            let mut ray = target.subtrate(&pos);
            // 归一化，得到光线的法向量
            ray.normalize();

            // 光线走到的最远距离
            let max = 9999_f64;
            // 光线开始行进
            pxl = ' ';
            for _ in 0..15000 {
                if f64::abs(pos.x) > max || f64::abs(pos.y) > max || f64::abs(pos.z) > max {
                    break;
                }
                // 计算光线距离最近物体的距离
                let dist = sdf(&pos);
                if dist < 1e-6 {
                    pxl = shade(&pos);
                    break;
                }
                pos = pos.add(&ray.multiply(dist));
            }
            framebuffer[y as usize][x as usize] = pxl;
        }
    }
}

pub fn sdf(pos: &Vec3) -> f64 {
    // 这里只是模拟了一个半径为0.2的球
    let center = Vec3::new(0.0, 0.0, 0.0);
    pos.subtrate(&center).length() - 0.2
}

pub fn shade(pos: &Vec3) -> char {
    // 随时间变换位置的光源
    let t = chrono::Local::now().timestamp() as f64;

    let mut l = Vec3::new(50.0 * t.sin(), 20.0, 50.0 * t.sin());
    l.normalize();

    //
    let dt = 1e-6;
    let current_val = sdf(pos);
    let x = Vec3::new(pos.x + dt, pos.y, pos.z);
    let y = Vec3::new(pos.x, pos.y + dt, pos.z);
    let z = Vec3::new(pos.x, pos.y, pos.z + dt);
    let dx = sdf(&x) - current_val;
    let dy = sdf(&y) - current_val;
    let dz = sdf(&z) - current_val;
    let mut n = Vec3::new((dx - pos.x) / dt, (dy - pos.y) / dt, (dz - pos.z) / dt);
    if n.length() < 1e-9 {
        return PIXELS.first().unwrap().to_owned();
    }
    n.normalize();
    let mut diffuse = l.x * n.x + l.y * n.y + l.z * n.z;
    diffuse = (diffuse + 1.0) / 2.0 * PIXELS.len() as f64;
    PIXELS[diffuse.floor() as usize % PIXELS.len()]
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn printfb(framebuffer: &Vec<Vec<char>>) {
    for y in 0..HEIGHT {
        let row = framebuffer
            .get(y as usize)
            .unwrap()
            .into_iter()
            .collect::<String>();
        println!("{row}");
    }
}

fn main() {
    let mut framebuffer = vec![vec![' '; WEIGHT as usize]; HEIGHT as usize];
    loop {
        thread::sleep(Duration::from_millis(2));
        clear_screen();
        raymarch(&mut framebuffer);
        printfb(&framebuffer);
    }
}
