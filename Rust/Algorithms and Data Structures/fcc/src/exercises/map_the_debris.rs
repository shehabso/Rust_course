// According to Kepler's Third Law, the orbital period T of two point masses orbiting
// each other in a circular or elliptic orbit is:
//
// T=2π√a**3/μ
//
// - a is the orbit's semi-major axis
// - μ=GM is the standard gravitational parameter
// - G is the gravitational constant,
// - M is the mass of the more massive body.
//
// Return a new array that transforms the elements' average altitude into their orbital periods (in seconds).

// The array will contain objects in the format {name: 'name', avgAlt: avgAlt}.
//
// The values should be rounded to the nearest whole number. The body being orbited is Earth.
//
// The radius of the earth is 6367.4447 kilometers, and the GM value of earth is 398600.4418 km**3s**-2.

use std::f64::consts::PI;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Satellite {
    name: String,
    avg_alt: f64,
}

#[allow(dead_code)]
fn orbital_period(arr: Vec<Satellite>) -> Vec<(String, i32)> {
    const GM: f64 = 398600.4418;
    const EARTH_RADIUS: f64 = 6367.4447;

    arr.iter()
        .map(|sat| {
            let earth = EARTH_RADIUS + sat.avg_alt;
            let orbital_period = ((2.0 * PI * (earth.powi(3) / GM)).sqrt()).round() as i32;
            (sat.name.clone(), orbital_period)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1() {
        let sputnik = Satellite {
            name: "sputnik".to_string(),
            avg_alt: 35873.5553,
        };

        assert_eq!(
            orbital_period(vec![sputnik]),
            vec![("sputnik".to_string(), 86400)]
        );
    }
}
