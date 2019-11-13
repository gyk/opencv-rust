pub mod core;
#[cfg(feature = "contrib")]
pub mod aruco;
#[cfg(feature = "contrib")]
pub mod bgsegm;
#[cfg(feature = "contrib")]
pub mod bioinspired;
pub mod calib3d;
#[cfg(feature = "contrib")]
pub mod ccalib;
#[cfg(feature = "contrib")]
pub mod cvv;
pub mod dnn;
#[cfg(feature = "contrib")]
pub mod dpm;
#[cfg(feature = "contrib")]
pub mod face;
pub mod features2d;
#[cfg(feature = "contrib")]
pub mod freetype;
#[cfg(feature = "contrib")]
pub mod fuzzy;
#[cfg(feature = "contrib")]
pub mod hdf;
pub mod highgui;
#[cfg(feature = "contrib")]
pub mod img_hash;
pub mod imgcodecs;
pub mod imgproc;
#[cfg(feature = "contrib")]
pub mod line_descriptor;
pub mod ml;
pub mod objdetect;
#[cfg(feature = "contrib")]
pub mod phase_unwrapping;
pub mod photo;
#[cfg(feature = "contrib")]
pub mod plot;
#[cfg(feature = "contrib")]
pub mod sfm;
pub mod shape;
pub mod stitching;
#[cfg(feature = "contrib")]
pub mod structured_light;
pub mod superres;
pub mod video;
pub mod videoio;
pub mod videostab;
pub mod viz;
#[cfg(feature = "contrib")]
pub mod xfeatures2d;
#[cfg(feature = "contrib")]
pub mod xobjdetect;
#[cfg(feature = "contrib")]
pub mod xphoto;
pub mod types;
#[doc(hidden)]
pub mod sys;