use crate::entities::GeodataModel;
use geoutils::Location;

const DEGREE_TO_METERS: f64 = 111320.;

const METERS_THRESHOLD: f64 = 40.;
const MAX_LINE_DEVIATION: f64 = 40.;

fn geo_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let first_location = Location::new(p1.0, p1.1);
    let second_location = Location::new(p2.0, p2.1);
    let distance_meters = first_location
        .distance_to(&second_location)
        .unwrap()
        .meters();
    distance_meters
}

fn filter_geodata_by_distance(geodata_vec: Vec<GeodataModel>) -> Vec<GeodataModel> {
    let mut geodata_iter = geodata_vec.into_iter();
    let initial_fold_vec: Vec<GeodataModel> = vec![geodata_iter.next().unwrap()];

    // It is worth it to first operate on pointers and then collect values in another iteration,
    // because tuple_windows() uses clone
    let filtered = geodata_iter.fold(initial_fold_vec, |mut acc, next| {
        let distance_meters = geo_distance(
            (acc.last().unwrap().lat, acc.last().unwrap().lng),
            (next.lat, next.lng),
        );
        if distance_meters > METERS_THRESHOLD {
            acc.push(next);
        }
        acc
    });
    filtered
}

fn vector_length(vec: [f64; 2]) -> f64 {
    (vec[0] * vec[0] + vec[1] * vec[1]).sqrt()
}

fn check_if_back_tracking(first: [f64; 2], prev: [f64; 2], next: [f64; 2]) -> bool {
    let first_to_prev = [first[0] + prev[0], first[1] + prev[1]];
    let first_to_next = [first[0] + next[0], first[1] + next[1]];
    vector_length(first_to_next) < vector_length(first_to_prev)
}

pub fn calculate_distance_from_line(point: [f64; 2], start: [f64; 2], end: [f64; 2]) -> f64 {
    let dx = end[0] - start[0];
    let dy = end[1] - start[1];
    let s = ((start[1] - point[1]) * dx - (start[0] - point[0]) * dy) / (dx * dx + dy * dy);
    s.abs() * (dx * dx + dy * dy).sqrt() * DEGREE_TO_METERS
}
fn remove_points_inside_lines(path: Vec<GeodataModel>) -> Vec<GeodataModel> {
    let mut path_iter = path.into_iter();
    let (first, second) = match (path_iter.next(), path_iter.next()) {
        (Some(a), Some(b)) => (a, b),
        (Some(a), _) => return vec![a],
        _ => unreachable!("x"), // Previous processing cannot put empty vec as an argument
    };
    let mut first_in_line_xy = [first.lat, first.lng];
    let mut second_in_line_xy = [second.lat, second.lng];
    let mut filtered_path = vec![first];

    let mut prev = second;
    for next in path_iter {
        let next_xy = [next.lat, next.lng];
        let prev_xy = [prev.lat, prev.lng];
        let next_is_not_in_line =
            calculate_distance_from_line(next_xy, first_in_line_xy, second_in_line_xy)
                > MAX_LINE_DEVIATION; //TODO the function
        let next_is_back_tracking = check_if_back_tracking(first_in_line_xy, prev_xy, next_xy);

        if next_is_not_in_line || next_is_back_tracking {
            filtered_path.push(prev);
            first_in_line_xy = prev_xy;
            second_in_line_xy = next_xy;
        }
        prev = next;
    }
    // prev is now the last element of the path,
    // which should always be added at the end:
    // it is either the end of current line
    // or it breaks the current line and is the end of a new one
    filtered_path.push(prev);
    filtered_path
}

pub fn quantize_geodata(geodata_vec: Vec<GeodataModel>) -> Vec<GeodataModel> {
    if geodata_vec.is_empty() {
        return vec![];
    }
    let filtered_geodata: Vec<GeodataModel> = filter_geodata_by_distance(geodata_vec);

    let quantized_path = remove_points_inside_lines(filtered_geodata);

    quantized_path
}
