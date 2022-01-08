

use geoutils::Location;

const METERS_THRESHOLD: f64 = 40;
const MAX_LINE_DEVIATION: f64 = 40; 
fn geo_distance (p1: (f64,f64), p2: (f64,f64)) -> f64 {
     
    let first_location = Location::new(p1.0, p1.1);
    let second_location = Location::new(p2.0, p2.1);
    let distance_meters = first_location.distance_to(&second_location).unwrap().meters();
    distance_meters
}

fn distance_from_line(point: (f64, f64), line_coeffs: (f64, f64)) -> f64 {
    let (x,y) = point;
    let (a,b) = line_coeffs;
    let perpendicular_a = -1/a;
    let perpendicular_b = y - x * perpendicular_a;
    let intersect_x = (perpendicular_b - b) / ( a - perpendicular_a)
    let intersect_y = a * intersect_x + b;
         
    geo_distance((x,y), (intersect_x, intersect_y))
}


async fn quantize_geodata
(
    conn: Connection<'_, Db>,
) -> Result<Json<Vec<GeoDataJson>>, rocket::response::Debug<sea_orm::DbErr>> {
    let db = conn.into_inner();
    let geodata_vec: Vec<entities::geodata::Model> = Geodata::find()
        .filter(
            Condition::all()
                .add(entities::geodata::Column::Uid.eq(user.sub))
                .add(
                    entities::geodata::Column::Timestamp
                        .gt(Utc::now().naive_utc() - Duration::days(7)),
                ),
        )
        .all(db)
        .await
        .expect("could not find geodata");

    let geodata_windows = geodata_vec.into_iter().windows().filter(|prev,next| {
        let distance_meters = geo_distance((prev.lat, prev.lng), (next.lat, next.lng));
        distance_meters < METERS_THRESHOLD
    }).windows();
    let mut paths = vec![];
    let mut current_path = vec![filtered_geodata.next(),filtered_geodata.next()];
    let mut current_line = 
        
    //handle only 1 point (here?)

    //CURRENT LINE SETUP HERE
    let mut line_coeffs: (f64,f64) = get_line_coeffs((prev.lat,prev.lng),(next.lat,next.lng));

    while Some((prev,next)) = geodata_windows.next(){

            let next_is_in_line: bool = MAX_LINE_DEVIATION > distance_from_line((next.lat,next.lng));
            let next_is_backtracking = //TODO; 
            if this_is_another_path {
                paths.push(current_path);
                current_path = vec![];
            }
            if !next_is_in_line || this_is_another_path{
                
                current_path.push(prev);
                line_coeffs =  get_line_coeffs((prev.lat,prev.lng),(next.lat,next.lng));
                //add prev to path as ending of the line
                // calculate delta next_prev as direction of the new line
                // 
                //CURRENT LINE SETUP HERE
            }
        }
    }
}