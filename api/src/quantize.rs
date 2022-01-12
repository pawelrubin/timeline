use geoutils::Location;

use crate::auth;
use crate::db::pool::Db;
use crate::entities;
use crate::entities::{GeodataColumn, GeodataEntity, GeodataJson, GeodataModel};
use itertools::Itertools;

use chrono::{Duration, Utc};

use sea_orm::query::*;
use sea_orm_rocket::Connection;

use rocket::serde::json::Json;
use sea_orm::{entity::*, query::*, ConnectionTrait, DatabaseBackend, EntityTrait, Set, Statement};

const METERS_THRESHOLD: f64 = 40.;
const MAX_LINE_DEVIATION: f64 = 40.;
const NEW_PATH_TIME_MINUTES: i64 = 60;

fn calculate_dir_vec(v1: [f64;2], v2: [f64;2]) -> [f64;2] {
    [v2[0] - v1[0], v2[1] - v1[1]]
}

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
    let initial_fold_vec: Vec<GeodataModel> = vec![geodata_vec[0].to_owned()];

    // It is worth it to first operate on pointers and then collect values in another iteration,
    // because tuple_windows() uses clone
    let filtered = geodata_vec
        .iter()
        .tuple_windows()
        .fold(initial_fold_vec, |mut acc, (prev, next)| {
            let distance_meters = geo_distance((prev.lat, prev.lng), (next.lat, next.lng));
            if distance_meters > METERS_THRESHOLD {
                acc.push(*next);
            }
            acc
        });
    filtered
}

fn split_geodata_by_time<'a>(geodata_vec: Vec<GeodataModel>) -> Vec<Vec<GeodataModel>> {
    let initial_fold_vec: Vec<Vec<GeodataModel>> = vec![vec![geodata_vec[0].to_owned()]];
    let split = geodata_vec
        .iter()
        .tuple_windows()
        .fold(initial_fold_vec, |mut acc, (prev, next)| {
            if next.timestamp - prev.timestamp > Duration::minutes(NEW_PATH_TIME_MINUTES) {
                //TODO
                acc.push(vec![]);
            }
            acc.last_mut().unwrap().push(*next);
            acc
        });
    split
    
}

fn vector_length(vec: [f64; 2]) -> f64 {
    (vec[0]*vec[0]+vec[1]*vec[1]).sqrt()
}

fn check_if_back_tracking(first: [f64;2], prev: [f64;2], next: [f64;2]) -> bool {
    let first_to_prev = [first[0] + prev[0], first[1] + prev[1]];
    let first_to_next =  [first[0] + next[0], first[1] +next[1]];
    vector_length(first_to_next) < vector_length(first_to_prev)

}

fn calculate_distance_from_line(point: [f64;2],start: [f64;2],end: [f64;2]) -> f64 {
    let dx = end[0] - start[0];
    let dy = end[1] - start[1];
    let s = ((start[1] - point[1]) * dx - (start[0] - point[0]) * dy) /
        (dx * dx + dy * dy);
    s.abs() * ((dx * dx + dy * dy)).sqrt()
}

//TODO last iteration
fn remove_points_inside_lines(path: Vec<GeodataModel>) -> Vec<GeodataModel> {
    let mut path_windows = path.iter().tuple_windows();
    let (first, second) = match path_windows.next() {
        Some((a, b)) => (a, b),
        None => return vec![path[0]],
    };
    let initial_path_vec = vec![*first];
    let initial_start = [first.lat, first.lng];
    let initial_end = [second.lat, second.lng];
    //TODO check if lat and lng should be swithced
    let (filtered_path, _, _) = path_windows.fold(
        (initial_path_vec, initial_start, initial_end),
        |(mut filtered_path, mut start, mut end), (prev, next)| {
            let next_xy = [next.lat, next.lng]; 
            let prev_xy = [prev.lat, prev.lng];
            let next_is_not_in_line = calculate_distance_from_line(next_xy, start, end) > MAX_LINE_DEVIATION; //TODO the function
            let next_is_back_tracking = check_if_back_tracking(start, prev_xy, next_xy);

            if next_is_not_in_line || next_is_back_tracking {
                filtered_path.push(*prev);
                start = prev_xy;
                end = next_xy
            }
            (filtered_path, start, end)
        },
    );
    filtered_path
}
async fn quantize_geodata(
    geodata_vec: Vec<GeodataModel>
) -> Vec<Vec<GeodataModel>> {
    let filtered_geodata: Vec<GeodataModel> = filter_geodata_by_distance(geodata_vec);
    let paths: Vec<Vec<GeodataModel>> = split_geodata_by_time(filtered_geodata);

    let quantized_paths = paths
        .into_iter()
        .map(|path| remove_points_inside_lines(path))
        .collect::<Vec<Vec<_>>>();

    quantized_paths
}

