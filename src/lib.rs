use geo_types::Coordinate;
use geo_types::Point;
use geo_types::Rect;
use std::f64::consts::PI;

pub static BASE: f64 = 2.0;

pub struct Tile {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn point_to_tile(point: Point<f64>, z: f64) -> Tile {
    let mut tile = point_to_tile_fraction(point, z);
    tile.x = tile.x.floor();
    tile.y = tile.y.floor();

    return tile;
}

pub fn point_to_tile_fraction(point: Point<f64>, z: f64) -> Tile {
    let sin: f64 = point.y().to_radians().sin();
    let z2: f64 = BASE.powf(z);
    let mut x: f64 = z2 * (point.x() / 360.0 + 0.5);
    let y: f64 = z2 * (0.5 - 0.25 * ((1.0 + sin) / (1.0 - sin)).ln() / PI);
    x = x % z2;
    if x < 0.0 {
        x = x + z2;
    }

    return Tile { x: x, y: y, z: z };
}

pub fn tile_to_quadkey(tile: Tile) -> String {
    let mut key = String::from("");
    let x = tile.x.floor() as i64;
    let y = tile.y.floor() as i64;
    let mut z = tile.z.floor() as i64;

    while z > 0 {
        let mut b = 0;
        let mask = 1 << (z - 1);
        if (x & mask) != 0 {
            b += 1;
        }
        if (y & mask) != 0 {
            b += 2;
        }
        key.push_str(&b.to_string());
        z -= 1;
    }

    return key;
}

pub fn quadkey_to_tile(key: String) -> Tile {
    let mut x = 0;
    let mut y = 0;
    let z = key.len() as usize;
    let mut i = z;
    while i > 0 {
        let mask = 1 << (i - 1);
        let q: i8 = key.chars().nth(z - i).unwrap().to_string().parse().unwrap();
        if q == 1 {
            x |= mask;
        }
        if q == 2 {
            y |= mask;
        }
        if q == 3 {
            x |= mask;
            y |= mask;
        }

        i -= 1;
    }

    return Tile {
        x: x as f64,
        y: y as f64,
        z: z as f64,
    };
}

pub fn point_to_quadkey(point: Point<f64>, z: f64) -> String {
    let tile = point_to_tile(point, z);
    let key = tile_to_quadkey(tile);
    return key;
}

pub fn tile_to_rect(tile: Tile) -> Rect<f64> {
    let e: f64 = to_lon(tile.x + 1.0, tile.z);
    let w: f64 = to_lon(tile.x, tile.z);
    let s: f64 = to_lat(tile.y + 1.0, tile.z);
    let n: f64 = to_lat(tile.y, tile.z);
    let rect = Rect::new(Coordinate { x: w, y: s }, Coordinate { x: e, y: n });
    return rect;
}

pub fn point_to_rect(point: Point<f64>, z: f64) -> Rect<f64> {
    let tile = point_to_tile(point, z);
    let rect = tile_to_rect(tile);
    return rect;
}

pub fn quadkey_to_rect(key: String) -> Rect<f64> {
    let tile = quadkey_to_tile(key);
    let rect = tile_to_rect(tile);
    return rect;
}

fn to_lon(x: f64, z: f64) -> f64 {
    return x / BASE.powf(z) * 360.0 - 180.0;
}

fn to_lat(y: f64, z: f64) -> f64 {
    let n = PI - BASE * PI * y / BASE.powf(z);
    let r = (0.5 * (n.exp() - -n.exp())).atan();
    return r.to_degrees();
}

#[cfg(test)]
mod tests {
    use super::point_to_quadkey;
    use super::point_to_rect;
    use super::point_to_tile;
    use super::point_to_tile_fraction;
    use super::quadkey_to_rect;
    use super::quadkey_to_tile;
    use super::tile_to_quadkey;
    use super::tile_to_rect;
    use super::Tile;
    use geo_types::Point;

    #[test]
    fn test_tile() {
        let tile = Tile {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(tile.x, 1.0);
        assert_eq!(tile.y, 2.0);
        assert_eq!(tile.z, 3.0);
    }

    #[test]
    fn test_point_to_tile() {
        let point = Point::new(-122.406921, 37.785232);
        let z: f64 = 7.0;
        let tile = point_to_tile(point, z);
        assert_eq!(tile.x, 20.0);
        assert_eq!(tile.y, 49.0);
        assert_eq!(tile.z, 7.0);
    }

    #[test]
    fn test_point_to_tile_fraction() {
        let point = Point::new(-122.406921, 37.785232);
        let z: f64 = 7.0;
        let tile = point_to_tile_fraction(point, z);
        assert_eq!(tile.x, 20.477539200000002);
        assert_eq!(tile.y, 49.47003194010788);
        assert_eq!(tile.z, 7.0);
    }

    #[test]
    fn test_tile_to_quadkey() {
        let tile = Tile {
            x: 20.0,
            y: 49.0,
            z: 7.0,
        };
        let key = tile_to_quadkey(tile);
        assert_eq!(key, "0230102");
    }

    #[test]
    fn test_quadkey_to_tile() {
        let key = String::from("0230102");
        let tile = quadkey_to_tile(key);
        assert_eq!(tile.x, 20.0);
        assert_eq!(tile.y, 49.0);
        assert_eq!(tile.z, 7.0);
    }

    #[test]
    fn test_point_to_quadkey() {
        let point = Point::new(-122.406921, 37.785232);
        let z: f64 = 7.0;
        let key = point_to_quadkey(point, z);
        assert_eq!(key, "0230102");
    }

    #[test]
    fn test_tile_to_rect() {
        let tile = Tile {
            x: 20.0,
            y: 49.0,
            z: 7.0,
        };
        let rect = tile_to_rect(tile);
        assert_eq!(rect.min.x, -123.75);
        assert_eq!(rect.min.y, 63.2989445665351);
        assert_eq!(rect.max.x, -120.9375);
        assert_eq!(rect.max.y, 64.41129548808857);
    }

    #[test]
    fn test_point_to_rect() {
        let point = Point::new(-122.406921, 37.785232);
        let z: f64 = 7.0;
        let rect = point_to_rect(point, z);
        assert_eq!(rect.min.x, -123.75);
        assert_eq!(rect.min.y, 63.2989445665351);
        assert_eq!(rect.max.x, -120.9375);
        assert_eq!(rect.max.y, 64.41129548808857);
    }

    #[test]
    fn test_quadkey_to_rect() {
        let key = String::from("0230102");
        let rect = quadkey_to_rect(key);
        assert_eq!(rect.min.x, -123.75);
        assert_eq!(rect.min.y, 63.2989445665351);
        assert_eq!(rect.max.x, -120.9375);
        assert_eq!(rect.max.y, 64.41129548808857);
    }
}
