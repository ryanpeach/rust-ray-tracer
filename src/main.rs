extern crate image;
extern crate cgmath;

use cgmath::{Point3, Vector3, InnerSpace};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba, Pixel};

// REF: https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

impl Color {
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        let rgba = rgba.0;

        Color {
            red: gamma_decode((rgba[0] as f32) / 255.0),
            green: gamma_decode((rgba[1] as f32) / 255.0),
            blue: gamma_decode((rgba[2] as f32) / 255.0),
        }
    }
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
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color.to_rgba())
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
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
        // Camera origin is (0, 0, 0) and sensors are located -1 z away.

        // This describes how the ray direction is calculated
        // First the pixel center is calculated as it's starting value + half a pixel
        // Then it's normalized to the width of the scene
        // Then it's adjusted from coordinates (0..1) to (-1..1) via *2
        fn sensor(scene: &Scene, v: u32) -> f64 {
            let pixel_center = v as f64 + 0.5;
            let normalized_to_width = pixel_center / scene.width as f64;
            let adjusted_screen_pos = (normalized_to_width * 2.0) - 1.0;
            adjusted_screen_pos
        }

        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =  sensor(scene, x) * fov_adjustment * aspect_ratio;
        let sensor_y = -sensor(scene, y) * fov_adjustment;  // y is positive in the down direction

        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0       // z is -1.0 because all of our prime rays should go forward from the camera
            }.normalize()
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // Create a line segment between the ray origin and the center of the sphere
        let l: Vector3<f64> = self.center - ray.origin;
        // Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.direction);
        // Find the length-squared of the opposite side
        // This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(l) - (adj2 * adj2);
        // If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}

fn main() { 
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

    img.save("image.png");

}
