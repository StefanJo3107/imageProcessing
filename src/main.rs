use std::{env, process};

use plotters::prelude::*;
use raster::Image;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let image = raster::open(format!("images/{}", config.filename).as_str()).unwrap();
    let sum_plots = vec![
        152, 156, 164, 165, 171, 176, 194, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
        207, 208, 209, 210, 211, 212, 213,
    ];

    let mut sum_points: Vec<(f32, f32)> = vec![];
    for i in 0..image.width {
        let (points, max_x) = get_points(&image, &config.color_channel, i);
        if sum_plots.contains(&i) {
            // let mut points_x: Vec<f32> = Vec::new();
            // for point in points.iter() {
            //     points_x.push(point.0);
            // }
            println!("");
            if sum_points.len() > 0 {
                for (j, point) in points.iter().enumerate() {
                    //println!("{}\t{}", point.0 / 3.7795275591, point.1);
                    sum_points[j].0 += point.0;
                    sum_points[j].1 += point.1;
                }
            } else {
                for point in points {
                    //println!("{}\t{}", point.0 / 3.7795275591, point.1);
                    sum_points.push(point);
                }
            }
        }

        // plot(points, max_x, format!("{}{}.png", config.output, i));
    }

    let mut max_y = 0.0;
    let mut max_x = 0.0;
    for i in 0..sum_points.len() {
        sum_points[i].0 /= sum_plots.len() as f32;
        sum_points[i].1 /= sum_plots.len() as f32;
        println!("{}\t{}", sum_points[i].0 / 3.7795275591, sum_points[i].1);

        if sum_points[i].1 > max_y {
            max_y = sum_points[i].1;
            max_x = sum_points[i].0;
        }
    }

    plot(sum_points, max_x, format!("{}.png", config.output));
}

pub struct Config {
    pub filename: String,
    pub color_channel: String,
    pub output: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let color_channel = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a color channel"),
        };

        let output = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config {
            filename,
            color_channel,
            output,
        })
    }
}

fn get_points(image: &Image, color_channel: &str, x_coord: i32) -> (Vec<(f32, f32)>, f32) {
    let mut points = vec![];
    for i in 0..image.height {
        let pixel = image.get_pixel(x_coord, i).unwrap();
        if color_channel.eq("r") {
            points.push((i as f32, pixel.r as f32));
        } else if color_channel.eq("g") {
            points.push((i as f32, pixel.g as f32));
        }
    }
    let mut min_y = 255.0;
    let mut max_y = 0.0;
    let mut max_x = 0.0;
    for point in points.iter() {
        if point.1 < min_y {
            min_y = point.1;
        }

        if point.1 > max_y {
            max_y = point.1;
            max_x = point.0;
        }
    }

    for i in 0..points.iter().len() {
        points[i].1 -= min_y;
        points[i].1 /= max_y - min_y;
        points[i].0 -= max_x;
    }

    (points, max_x)
}

fn plot(points: Vec<(f32, f32)>, max_x: f32, output: String) {
    let output_path = format!("images/{}", output.as_str());
    let root = BitMapBackend::new(output_path.as_str(), (1000, 300)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .margin_right(5)
        .margin_top(10)
        .build_cartesian_2d(points[0].0..points.last().unwrap().0, 0f32..1.01f32)
        .unwrap();
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(15)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()
        .unwrap();
    // chart
    //     .draw_series(PointSeries::of_element(points, 3, &RED, &|c, s, st| {
    //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //         + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
    //     }))
    //     .unwrap();
    chart.draw_series(LineSeries::new(points, &RED)).unwrap();
}
