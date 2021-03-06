//! Mirage is a library for describing vector graphics.
//! Rather than actually rendering the graphics itself,
//! it is intended to provide a usable definition that can be used
//! by external rendering programs themselves.

#[macro_use]
pub mod object;
pub mod context;
pub mod convert;
#[cfg(feature = "raster")]
pub mod raster;
pub mod surface;
