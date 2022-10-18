fn main1() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
    let kcle_latitude_degrees: f64 = 41.475;
    let kcle_longitude_degrees: f64 = -81.8511;

    let kslc_latitude_degrees: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
    let kcle_longitude_radians = kcle_longitude_degrees.to_radians();

    let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
    let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
        + kcle_latitude_radians.cos()
        + kslc_latitude_degrees.cos() * f64::powi((delta_longitude / 2.0).sin(), 2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

    println!(
        "The distance between the two points is {:.1} kilometers",
        distance
    );
}


fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
    let route = [
        ("KCLE", 40.4075, -81.85111),
        ("CLE", 40.4075, -81.85111),
        ("KCE", 40.4075, -81.85111),
        ("KCLE", 40.4075, -81.85111),
        ("KCE", 40.4075, -81.85111),
        ("KLE", 40.4075, -81.85111),
        ("KE", 40.4075, -81.85111),
        ("KLE", 40.4075, -81.85111),
        ("KLE", 40.4075, -81.85111),
        ("KE", 40.4075, -81.85111),
        ("LE", 40.4075, -81.85111),
        ("E", 40.4075, -81.85111)
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();
            
                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos() + waypoint_radians.cos() 
                * f64::powi((delta_longitude / 2.0).sin(), 2);
            
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());

                println!("The distance, between {} and {} is {:.1} km", previous_waypoint_value.0, waypoint.0, distance);
            }
        }
    }
    
}


main();
