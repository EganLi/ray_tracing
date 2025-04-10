use vec3::write_color;

mod ray;
mod vec3;

fn main() {
    let image_width = 256 as u32;
    let image_height = 256 as u32;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            /*
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {} ", ir, ig, ib);
            */
            let pixel_color = vec3::Color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0,
            );
            write_color(&pixel_color);
        }
    }
}
