qk
---

`qk` is a fast geographic quadtree indexer for Rust

## Test

```sh
cargo test --lib
```

## Format

```sh
cargo fmt
```

## Types

Geometry types use [geo_types](https://docs.rs/geo-types/0.4.3/geo_types/index.html) whenever available. Additional structs are exposed for types unique to `qk`, such as the Tile struct.

## API

### Tile

An X,Y,Z tile struct.

### Point

A [point geo_types coordinate](https://docs.rs/geo-types/0.4.3/geo_types/struct.Point.html).

### Rect

A [bounded rectangle geometry](https://docs.rs/geo-types/0.4.3/geo_types/struct.Rect.html), also known as a bounding box or bbox.

### point_to_quadkey

Converts a lon,lat point to a quadkey index.

### point_to_tile

Converts a lon,lat point to an X,Y,Z tile.

### point_to_tile_fraction

Converts a lon,lat point to a fractional X,Y,Z tile.

### quadkey_to_tile

Converts a quadkey index string to an X,Y,Z tile.

### tile_to_quadkey

Converts an X,Y,Z tile to a quadkey index string.

### tile_to_rect

Converts an X,Y,Z tile to a bounding box Rect.

### quadkey_to_rect

Converts a quadkey index string to a bounding box Rect.

### point_to_rect

Converts a lon,lat point to a bounding box Rect.

## Inspiration

- [Mapbox tilebelt](https://github.com/mapbox/tilebelt) (co-author)
- [Mapbox tile-cover](https://github.com/mapbox/tile-cover/) (co-author)
- [Google S2](http://s2geometry.io/) (co-author of bindings)
- [Uber H3](https://eng.uber.com/h3/)
- [JPL/NASA healpix](https://healpix.jpl.nasa.gov)
- [Bing Maps Tile System](https://docs.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system)
- [OpenStreetMap QuadTiles](https://wiki.openstreetmap.org/wiki/QuadTiles)
- [G. Niemeyer's geohash](https://www.movable-type.co.uk/scripts/geohash.html)
