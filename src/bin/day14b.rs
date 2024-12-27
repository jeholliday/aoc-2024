use regex::Regex;

#[derive(Clone, Eq, PartialEq)]
struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)"
    )
    .unwrap();
}

fn solve(input: &str, width: usize, height: usize) -> usize {
    let init_robots: Vec<Robot> = REGEX
        .captures_iter(input)
        .map(|c| {
            let (_, [x, y, vx, vy]) = c.extract();
            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect();

    let width = width as isize;
    let height = height as isize;

    let mut robots = init_robots.clone();
    let mut iter = 0;
    loop {
        let mut map = vec![vec![0isize; width as usize]; height as usize];
        iter += 1;
        for robot in robots.iter_mut() {
            robot.x += robot.vx;
            if robot.x < 0 {
                robot.x += width;
            } else if robot.x >= width {
                robot.x -= width;
            }

            robot.y += robot.vy;
            if robot.y < 0 {
                robot.y += height;
            } else if robot.y >= height {
                robot.y -= height;
            }
            map[robot.y as usize][robot.x as usize] += 1;
        }

        let mut max_line_length = 0;
        for row in map.iter() {
            let mut line_length = 0;
            for cell in row {
                if *cell == 0 {
                    if line_length > max_line_length {
                        max_line_length = line_length;
                    }
                    line_length = 0;
                } else {
                    line_length += 1;
                }
            }
            if line_length > max_line_length {
                max_line_length = line_length;
            }
        }

        if max_line_length > 10 {
            for row in map.iter() {
                for i in row.iter() {
                    if *i > 9 {
                        print!("9");
                    } else if *i == 0 {
                        print!(" ");
                    } else {
                        print!("{}", i);
                    }
                }
                println!();
            }
            break;
        }
    }

    iter
}

fn main() {
    let input = include_str!("../../inputs/day14.txt");
    let ans = solve(input, 101, 103);
    println!("{}", ans);
}
