extern crate image;
extern crate cgmath;

use cgmath::{Point3, Vector3};
use image::{DynamicImage, GenericImageView};

// REF: https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub color: Color
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere
}

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point3 {x: 0.0, y: 0.0, z: -5.0},
            radius: 1.0,
            color: Color {red: 0.4, green: 1.0, blue: 0.4}
        }
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

}

// Here we implement our Ray class
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

// Prime rays are those that come from the camera, traced through the pixel, into the scene
impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

fn main() {
    println!("Hello, world!");
}