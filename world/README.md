# rad-tools-world

A Rust library for working with 3D grids of numeric data with coordinate transformations.

## Features

- **3D Grid**: Store numeric data in a three-dimensional grid
- **Coordinate Transformations**: Convert between grid indices and world coordinates
- **Invertible Transforms**: All transforms are guaranteed to be invertible
- **Generic Numeric Types**: Works with any type that implements `Copy` and `Debug`

## Overview

The library provides a `Grid3D` struct for storing numeric data in a three-dimensional grid, and a `Transform` struct for converting between grid indices and world coordinates.

### Grid3D

The `Grid3D` struct allows you to:
- Create a grid with specific dimensions and a default value
- Get and set values at specific indices
- Get and set values at specific world coordinates
- Convert between indices and coordinates

### Transform

The `Transform` struct allows you to:
- Create a transform with a custom 4x4 matrix
- Create identity, translation, and scale transforms
- Convert between grid indices and world coordinates

## Error Handling

The library uses the `thiserror` crate for error handling. The main error types are:

- `GridError::IndexOutOfBounds`: When trying to access indices outside the grid dimensions
- `GridError::NonInvertibleTransform`: When trying to create a transform with a non-invertible matrix
- `GridError::InvalidCoordinateConversion`: When coordinates cannot be converted to valid indices

## Implementation Details

The 3D grid is implemented as a 1D vector with a mapping function to convert 3D indices to a 1D index. The transform is implemented using a 4x4 matrix and its inverse for efficient coordinate conversions.