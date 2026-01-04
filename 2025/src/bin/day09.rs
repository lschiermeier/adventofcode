use std::collections::BTreeSet;

use aoc_2025::*;
use itertools::Itertools;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[allow(unused)]

fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let starting_table = read_table(input_path, ',').expect("File could not be opened.");
    let parsed_points = parse_table::<i64>(starting_table);
    let red_points: Vec<Point2d> = parsed_points
        .iter()
        .map(|x| x.try_into().unwrap())
        .collect();

    let areas = red_points
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(x, y)| (x.calc_area(*y), x, y))
        .sorted()
        .rev()
        .collect_vec();

    debug!(
        "calculated alls areas first ten = {:?}",
        areas.first_chunk::<10>()
    );
    info!("Result p1: {:?}", areas.first().unwrap());

    let green_border_points = red_points
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(point_a, point_b)| point_a.gen_between(*point_b))
        .concat();
    debug!(
        "generated border green points: {}",
        green_border_points.len()
    );

    let border_set: BTreeSet<Point2d> = BTreeSet::from_iter(
        green_border_points
            .iter()
            .cloned()
            .chain(red_points.iter().cloned()),
    );
    debug!("generated full border set: {}", border_set.len());
    let first_corner = red_points.first().unwrap();
    let forward_corner = red_points.get(1).unwrap();
    let backward_corner = red_points.last().unwrap();
    debug!(
        "first corner: {:?}, forward corner: {:?}, backward corner: {:?}",
        first_corner, forward_corner, backward_corner
    );

    let starting_dir = first_corner
        .direction_to(*forward_corner)
        .unwrap()
        .get_between(first_corner.direction_to(*backward_corner).unwrap())
        .expect("These points should be a corner");
    let starting_point = starting_dir.as_offset() + *first_corner;
    debug!(
        "starting dir: {:?}, starting point: {:?}",
        starting_dir, starting_point
    );

    for area in areas {
        if check_area(area.1, area.2, &border_set) {
            info!("area: {area:?} is fully contained in border!");
            break;
        } else {
            debug!("area: {area:?} is not contained in border searching...");
        }
    }
}

pub fn flood_fill(
    starting_point: Point2d,
    border_set: BTreeSet<Point2d>,
) -> Result<BTreeSet<Point2d>, String> {
    if border_set.contains(&starting_point) {
        Err("Starting point must not be on border.".to_string())
    } else {
        trace!("starting flood_fill");
        let mut out_set = BTreeSet::from([starting_point]);
        let mut points_to_check: BTreeSet<Point2d> = BTreeSet::from([starting_point]);
        while points_to_check.len() > 0 {
            let point = points_to_check
                .pop_first()
                .ok_or(String::from("Length of working points was just checked."))?;
            for dir in Direction::iterator() {
                let dir_point = point + dir.as_offset();
                if !border_set.contains(&dir_point)
                    && !out_set.contains(&dir_point)
                    && !points_to_check.contains(&dir_point)
                {
                    out_set.insert(dir_point);
                    points_to_check.insert(dir_point);
                }
            }
            if out_set.len() % 10000 == 0 {
                trace!(
                    "flood_fill loop in cycle {}, points_to_check.len() = {}",
                    out_set.len(),
                    points_to_check.len()
                );
            }
        }
        Ok(out_set)
    }
}

pub fn check_area(corner_a: &Point2d, corner_c: &Point2d, border_set: &BTreeSet<Point2d>) -> bool {
    let corner_b = Point2d {
        x: corner_a.x,
        y: corner_c.y,
    };
    let corner_d = Point2d {
        x: corner_c.x,
        y: corner_a.y,
    };
    let edges = vec![
        (
            corner_a.direction_to(corner_d).unwrap().get_opposite(),
            corner_a.gen_between(corner_b),
        ),
        (
            corner_b.direction_to(*corner_a).unwrap().get_opposite(),
            corner_b.gen_between(*corner_c),
        ),
        (
            corner_c.direction_to(corner_b).unwrap().get_opposite(),
            corner_c.gen_between(corner_d),
        ),
        (
            corner_d.direction_to(*corner_c).unwrap().get_opposite(),
            corner_d.gen_between(*corner_a),
        ),
    ];

    for (dir, edge) in edges {
        for mut point in edge {
            while !border_set.contains(&point) {
                if point.x < 0 || point.x > 100_000 || point.x < 0 || point.x > 100_000 {
                    trace!("checked dir of edge {dir:?} for points {corner_a:?} {corner_c:?}");
                    return false;
                } else {
                    point = point + dir.as_offset();
                }
            }
        }
        trace!("checked dir of edge {dir:?} for points {corner_a:?} {corner_c:?}");
    }
    true
}
