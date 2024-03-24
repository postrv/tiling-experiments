use std::f64::consts::PI;
use rand::Rng;

// Step 1: Define constants for angles and ratios
const SQRT_2: f64 = 1.4142135623730950;
const RHOMBUS_ANGLE: f64 = PI / 4.0;
const SQUARE_SIZE_RATIO: f64 = 1.0 / SQRT_2;

fn gradient_color(center: [f64; 2], img_center: [f64; 2], img_size: f64) -> String {
    let distance = ((center[0] - img_center[0]).powi(2) + (center[1] - img_center[1]).powi(2)).sqrt();
    let ratio = distance / (img_size / 2.0);
    let hue = ratio * 360.0;
    format!("hsl({}, 50%, 50%)", hue)
}

// Step 2: Define the Tile enum

// Update the Tile enum to include a color field
enum Tile {
    Rhombus {
        center: [f64; 2],
        size: f64,
        angle: f64,
        color: String,
    },
    Square {
        center: [f64; 2],
        size: f64,
        angle: f64,
        color: String,
    },
}

// Step 3: Implement methods for the Tile enum
impl Tile {
    fn vertices(&self) -> Vec<[f64; 2]> {
        match *self {
            Tile::Rhombus { center, size, angle, .. } => {
                let angle1 = angle;
                let angle2 = angle + RHOMBUS_ANGLE;
                let angle3 = angle2 + RHOMBUS_ANGLE;
                let angle4 = angle3 + RHOMBUS_ANGLE;

                let point1 = point_at_angle_and_distance(center, angle1, size);
                let point2 = point_at_angle_and_distance(center, angle2, size);
                let point3 = point_at_angle_and_distance(center, angle3, size);
                let point4 = point_at_angle_and_distance(center, angle4, size);

                vec![point1, point2, point3, point4]
            }
            Tile::Square { center, size, angle, .. } => {
                let angle1 = angle;
                let angle2 = angle + PI / 2.0;
                let angle3 = angle2 + PI / 2.0;
                let angle4 = angle3 + PI / 2.0;

                let point1 = point_at_angle_and_distance(center, angle1, size / SQRT_2);
                let point2 = point_at_angle_and_distance(center, angle2, size / SQRT_2);
                let point3 = point_at_angle_and_distance(center, angle3, size / SQRT_2);
                let point4 = point_at_angle_and_distance(center, angle4, size / SQRT_2);

                vec![point1, point2, point3, point4]
            }
        }
    }

    fn subdivide(&self, img_center: [f64; 2], img_size: f64) -> Vec<Tile> {
        match *self {
            Tile::Rhombus { center, size, angle, .. } => {
                let new_size = size / SQRT_2;

                let new_center1 = point_at_angle_and_distance(center, angle, size / 2.0);
                let new_center2 = point_at_angle_and_distance(center, angle + PI, size / 2.0);
                let square_center = point_at_angle_and_distance(center, angle + RHOMBUS_ANGLE, size / 2.0);

                let new_angle1 = angle + RHOMBUS_ANGLE;
                let new_angle2 = angle + RHOMBUS_ANGLE + PI;
                let square_angle = angle;

                vec![
                    Tile::Rhombus {
                        center: new_center1,
                        size: new_size,
                        angle: new_angle1,
                        color: gradient_color(new_center1, img_center, img_size),
                    },
                    Tile::Rhombus {
                        center: new_center2,
                        size: new_size,
                        angle: new_angle2,
                        color: gradient_color(new_center2, img_center, img_size),
                    },
                    Tile::Square {
                        center: square_center,
                        size: new_size * SQUARE_SIZE_RATIO,
                        angle: square_angle,
                        color: gradient_color(square_center, img_center, img_size),
                    },
                ]
            }
            Tile::Square { center, size, angle, .. } => {
                let new_size = size * SQRT_2;

                let rhombus_center1 = point_at_angle_and_distance(center, angle, size / SQRT_2);
                let rhombus_center2 = point_at_angle_and_distance(center, angle + PI / 2.0, size / SQRT_2);
                let rhombus_center3 = point_at_angle_and_distance(center, angle + PI, size / SQRT_2);
                let rhombus_center4 = point_at_angle_and_distance(center, angle + 3.0 * PI / 2.0, size / SQRT_2);

                let rhombus_angle1 = angle;
                let rhombus_angle2 = angle + PI / 2.0;
                let rhombus_angle3 = angle + PI;
                let rhombus_angle4 = angle + 3.0 * PI / 2.0;

                vec![
                    Tile::Rhombus {
                        center: rhombus_center1,
                        size: new_size,
                        angle: rhombus_angle1,
                        color: gradient_color(rhombus_center1, img_center, img_size),
                    },
                    Tile::Rhombus {
                        center: rhombus_center2,
                        size: new_size,
                        angle: rhombus_angle2,
                        color: gradient_color(rhombus_center2, img_center, img_size),
                    },
                    Tile::Rhombus {
                        center: rhombus_center3,
                        size: new_size,
                        angle: rhombus_angle3,
                        color: gradient_color(rhombus_center3, img_center, img_size),
                    },
                    Tile::Rhombus {
                        center: rhombus_center4,
                        size: new_size,
                        angle: rhombus_angle4,
                        color: gradient_color(rhombus_center4, img_center, img_size),
                    },
                ]
            }
        }
    }

    fn to_svg_polygon(&self) -> String {
        let vertices = self.vertices();
        let points: String = vertices
            .iter()
            .map(|&[x, y]| format!("{},{}", x, y))
            .collect::<Vec<String>>()
            .join(" ");
        match self {
            Tile::Rhombus { color, .. } | Tile::Square { color, .. } => {
                format!("<polygon points=\"{}\" fill=\"{}\" stroke=\"black\" stroke-width=\"1\" />", points, color)
            }
        }
    }
}


// Step 4: Implement utility functions
fn point_at_angle_and_distance(origin: [f64; 2], angle: f64, distance: f64) -> [f64; 2] {
    [
        origin[0] + distance * angle.cos(),
        origin[1] + distance * angle.sin(),
    ]
}

fn rotate_point_about_center(point: [f64; 2], center: [f64; 2], angle: f64) -> [f64; 2] {
    let translated_point = [point[0] - center[0], point[1] - center[1]];
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    let rotated_point = [
        cos_theta * translated_point[0] - sin_theta * translated_point[1],
        sin_theta * translated_point[0] + cos_theta * translated_point[1],
    ];
    [rotated_point[0] + center[0], rotated_point[1] + center[1]]
}

// Step 5: Implement the subdivide_recursively function
fn subdivide_recursively(tile: Tile, depth: usize, img_center: [f64; 2], img_size: f64) -> Vec<Tile> {
    if depth == 0 {
        vec![tile]
    } else {
        let mut result = Vec::new();
        for subtile in tile.subdivide(img_center, img_size) {
            result.append(&mut subdivide_recursively(subtile, depth - 1, img_center, img_size));
        }
        result
    }
}

// Step 6: Implement the main function
fn main() {
    let img_width: f64 = 800.0;
    let img_height: f64 = 800.0;

    let base_center = [img_width / 2.0, img_height / 2.0];
    let initial_size = img_width / 4.0;

    let initial_tiles = vec![
        Tile::Rhombus {
            center: base_center,
            size: initial_size,
            angle: 0.0,
            color: "".to_string(),
        },
        Tile::Square {
            center: base_center,
            size: initial_size * SQUARE_SIZE_RATIO,
            angle: RHOMBUS_ANGLE,
            color: "".to_string(),
        },
    ];

    let depth = 5;
    let mut tiles = Vec::new();
    for tile in initial_tiles {
        tiles.append(&mut subdivide_recursively(tile, depth, base_center, img_width));
    }

    let mut svg_content = String::new();
    for tile in tiles {
        svg_content.push_str(&tile.to_svg_polygon());
    }

    let svg_data = format!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>", img_width, img_height, svg_content);

    std::fs::write("amman_beenker_tiling2.svg", svg_data).unwrap();
}