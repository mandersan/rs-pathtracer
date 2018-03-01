pub mod materials;
pub mod shapes;
pub mod tracing;
pub mod util;

pub mod cameras;
pub use self::cameras::{Camera};

mod types;
pub use self::types::{Ray, Interval};
pub use self::types::{Hit, Hitable, HitableCollection};
pub use self::types::{ScatteredRay, Scatterable};