use crate::config::MagnetKind;

use super::{magnet3d::Magnet3D, Magnet2D};

/// Convenience type alias for a Vec of MagnetType enum. It is more general than
/// `MagnetVec2D` and `MagnetVec3D`, but involves more boilerplate code.
pub type MagnetVec = Vec<Magnet>;

/// Convenience type alias for a Vec of MagnetType2D enum
pub type MagnetVec2D = Vec<Magnet2D>;

/// Convenience type alias for a Vec of MagnetType2D enum
pub type MagnetVec3D = Vec<Magnet2D>;

/// Convenience type alias for a Vec of MagnetKind enum used for serializing/deserializing.
pub type MagnetList = Vec<MagnetKind>;

/// Convenience enum containing 2D and 3D magnet types.
#[derive(Debug, PartialEq)]
pub enum Magnet {
    /// 2D Magnet
    Magnet2D(Magnet2D),
    /// 3D Magnet
    Magnet3D(Magnet3D),
}

/// Return center trait. It must implement the `center()` method
pub trait GetCenter<T> {
    /// Returns center method for any type
    fn center(&self) -> T;
}

/// Generic trait for returning the magnetic field due to an input point. This
/// must be implemented for each magnet type and each input type.
pub trait GetField<INPUT, OUTPUT> {
    /// Returns the magnetic field due to a generic input which must contain an
    /// x and y coordinate.
    fn field(&self, point: INPUT) -> OUTPUT;
}

/// Magnet Trait for standard methods for all magnet types
pub trait MagnetTrait<POINT, CENTER, SIZE, MAG> {
    /// Returns the magnetic field at a point
    // fn field(&self, point: &POINT) -> anyhow::Result<POINT, MagnetError>;

    /// Returns the magnet center
    fn center(&self) -> CENTER;

    /// Returns the magnet dimensions.
    ///
    /// Note: This returns a generic, an array `[f64;2]` for Rectangles,
    /// and f64 for Circles
    fn size(&self) -> SIZE;

    /// Returns the magnetisation vector
    fn magnetisation(self) -> MAG;

    /// Sets the magnet center to a point
    fn set_center(&mut self, point: CENTER);

    /// Sets the size the of the magnet.
    /// Generic method which can also change internal struct values
    fn set_size(&mut self, point: SIZE);

    /// Set the magnetisation  of the magnet using a Polar vector.
    /// i.e. magnitude and angle phi.
    ///
    /// This method also updates self.jx and self.jy
    fn set_magnetisation(&mut self, magnetisation: MAG);
}
