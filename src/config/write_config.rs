use super::{MagnetKind, ReadCircle, ReadRectangle};
use crate::{
    magnets::{Magnet, MagnetType2D},
    points::PointVec2,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimResult {
    pub magnets: Vec<MagnetKind>,
    pub points: PointVec2,
    pub field: PointVec2,
}

impl SimResult {
    pub fn new(magnets: Vec<MagnetKind>, points: PointVec2, field: PointVec2) -> Self {
        SimResult {
            magnets: magnets,
            points: points,
            field: field,
        }
    }
}

pub fn magnet2d_to_toml(magnet: &MagnetType2D) -> anyhow::Result<MagnetKind> {
    Ok(match magnet {
        MagnetType2D::Circle(mag) => MagnetKind::Circle(ReadCircle::new(
            mag.size(),
            [mag.center.x, mag.center.y],
            [mag.jr, mag.phi.to_degrees()],
            "degrees".to_string(),
            mag.alpha.to_degrees(),
            "degrees".to_string(),
        )),
        MagnetType2D::Rectangle(mag) => MagnetKind::Rectangle(ReadRectangle::new(
            mag.size(),
            [mag.center.x, mag.center.y],
            [mag.jr, mag.phi.to_degrees()],
            "degrees".to_string(),
            mag.alpha.to_degrees(),
            "degrees".to_string(),
        )),
    })
}

pub fn gen_magnet_toml_2d(magnets: &Vec<MagnetType2D>) -> anyhow::Result<Vec<MagnetKind>> {
    let mut magnet_list = Vec::<MagnetKind>::with_capacity(magnets.len());
    for mag in magnets {
        magnet_list.push(mag.to_toml_struct().unwrap())
    }

    Ok(magnet_list)
}

pub fn save_results(sim_result: &SimResult, outfile: &str) {
    let out_string = serde_json::to_string(sim_result).unwrap();
    println!("Outfile: {}", outfile);
    println!("{:?}", out_string);
}

// Horrible kludge, don't use.
// pub fn points_to_toml(points: &PointVec2) -> anyhow::Result<GridKind2D> {
//     Ok(GridKind2D::Custom(ReadGridCustom {
//         x: points.x.clone(),
//         y: points.y.clone(),
//     }))
// }

#[cfg(test)]
mod tests {
    use crate::{
        config::{Configure, GridKind2D},
        magnets::Rectangle,
    };

    use super::*;

    #[test]
    pub fn test_magnet_to_toml_struct() {
        let m1 = MagnetType2D::Rectangle(Rectangle::default());
        let mag_toml = m1.to_toml_struct().unwrap();
        let mag_string = toml::to_string(&mag_toml).unwrap();
        let comp_string = "kind = \"rectangle\"\nsize = [1.0, 1.0]\ncenter = [0.0, 0.0]\nmagnetisation = [1.0, 90.0]\nmagAngle = \"degrees\"\nalpha = 0.0\nalphaAngle = \"degrees\"\n".to_string();
        println!("{}", mag_string);
        assert_eq!(mag_string, comp_string);
    }

    #[test]
    pub fn test_magnet_array_to_toml_struct() {
        let mut magnet_list = Vec::<MagnetType2D>::new();

        let m1 = MagnetType2D::Rectangle(Rectangle::default());
        magnet_list.push(m1);
        let m2 = MagnetType2D::Rectangle(Rectangle::default());
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
