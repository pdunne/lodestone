use super::{MagnetKind, ReadCircle, ReadCustomPolygon, ReadRectangle};
use crate::{
    magnets::{Magnet2D, MagnetTrait},
    points::PointVec2,
    MagnetError,
};
use serde_derive::{Deserialize, Serialize};

use std::fs::File;
// use std::sync::Mutex;

/// Struct containing the results of the calculation,
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimResult {
    /// Array of magnets
    pub magnets: Vec<MagnetKind>,
    /// Array of points
    pub points: PointVec2,
    /// Unit of length for points
    pub units: String,
    /// Array of calculated magnetic field
    pub field: PointVec2,
}

impl SimResult {
    /// Generates new SimResult Struct
    pub fn new(
        magnets: Vec<MagnetKind>,
        points: PointVec2,
        units: String,
        field: PointVec2,
    ) -> Self {
        SimResult {
            magnets,
            points,
            units,
            field,
        }
    }
}

/// Converts magnet to serializable struct `MagnetKind`
pub fn magnet2d_to_toml(magnet: &Magnet2D) -> Result<MagnetKind, MagnetError> {
    Ok(match magnet {
        Magnet2D::Circle(mag) => MagnetKind::Circle(ReadCircle::new(
            mag.size(),
            [mag.center.x, mag.center.y],
            [mag.jr, mag.phi.to_degrees()],
            "degrees".to_string(),
            mag.alpha.to_degrees(),
            "degrees".to_string(),
        )),
        Magnet2D::Rectangle(mag) => MagnetKind::Rectangle(ReadRectangle::new(
            mag.size(),
            [mag.center.x, mag.center.y],
            [mag.jr, mag.phi.to_degrees()],
            "degrees".to_string(),
            mag.alpha.to_degrees(),
            "degrees".to_string(),
        )),

        Magnet2D::Polygon(mag) => MagnetKind::CustomPolygon(ReadCustomPolygon::new(
            mag.center.as_array(),
            [mag.jr, mag.phi.to_degrees()],
            "Degrees".to_string(),
            mag.alpha.to_degrees(),
            "Degrees".to_string(),
            mag.vertices.clone(), //TODO: Fix clone ownership
        )),
    })
}

/// Converts magnet vector to vector of serializable `Magnetkind`
pub fn gen_magnet_toml_2d(magnets: &[Magnet2D]) -> Result<Vec<MagnetKind>, MagnetError> {
    let mut magnet_list = Vec::<MagnetKind>::with_capacity(magnets.len());
    for mag in magnets {
        magnet_list.push(mag.to_toml_struct()?)
    }

    Ok(magnet_list)
}

/// Writes `SimResult` struct to file
pub fn save_results(sim_result: &SimResult, outfile: &str) -> Result<(), MagnetError> {
    let file = File::create(outfile)?;

    serde_json::to_writer(file, sim_result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{Configure, GridKind2D},
        magnets::Rectangle,
    };

    use super::*;

    #[test]
    pub fn test_magnet_to_toml_struct() {
        let m1 = Magnet2D::Rectangle(Rectangle::default());
        let mag_toml = m1.to_toml_struct().unwrap();
        let mag_string = toml::to_string(&mag_toml).unwrap();
        let comp_string = "kind = \"rectangle\"\nsize = [1.0, 1.0]\ncenter = [0.0, 0.0]\nmagnetisation = [1.0, 90.0]\nmagAngle = \"degrees\"\nalpha = 0.0\nalphaAngle = \"degrees\"\n".to_string();
        println!("{}", mag_string);
        assert_eq!(mag_string, comp_string);
    }

    #[test]
    pub fn test_magnet_array_to_toml_struct() {
        let mut magnet_list = Vec::<Magnet2D>::new();

        let m1 = Magnet2D::Rectangle(Rectangle::default());
        magnet_list.push(m1);
        let m2 = Magnet2D::Rectangle(Rectangle::default());
        magnet_list.push(m2);

        let mag_toml = gen_magnet_toml_2d(&magnet_list).unwrap();

        let config = Configure {
            magnet: mag_toml,
            grid: GridKind2D::None,
        };

        let config_string = toml::to_string(&config).unwrap();

        let comp_string = "[grid]\nkind = \"none\"\n\n[[magnet]]\nkind = \"rectangle\"\nsize = [1.0, 1.0]\ncenter = [0.0, 0.0]\nmagnetisation = [1.0, 90.0]\nmagAngle = \"degrees\"\nalpha = 0.0\nalphaAngle = \"degrees\"\n\n[[magnet]]\nkind = \"rectangle\"\nsize = [1.0, 1.0]\ncenter = [0.0, 0.0]\nmagnetisation = [1.0, 90.0]\nmagAngle = \"degrees\"\nalpha = 0.0\nalphaAngle = \"degrees\"\n".to_string();
        assert_eq!(config_string, comp_string);
    }
}
