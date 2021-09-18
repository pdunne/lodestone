use crate::{
    config::{gen_magnet_toml_2d, save_results, SimResult},
    magnets::{Magnet2D, Rectangle},
    points::{cart_prod_2d_vec, Point2},
    utils::conversions::Angle,
    MagnetError,
};

pub fn run_demo() -> Result<(), MagnetError> {
    println!("Running Demo Calculation, two square magnets over a grid of 100 points");

    let outfile = "example_out.json";
    let points = cart_prod_2d_vec(&Point2::new(-2.0, -2.0), &Point2::new(2.0, 2.0), &100);
    let mut magnet_list = Vec::<Magnet2D>::new();

    // Create Magnets
    let m1 = Rectangle::new(
        1.0,
        1.0,
        (-1.0, -0.5),
        Angle::Degrees(0.0),
        1.0,
        Angle::Degrees(90.0),
    );
    magnet_list.push(Magnet2D::Rectangle(m1));

    let m2 = Rectangle::new(
        1.0,
        1.0,
        (1.0, -0.5),
        Angle::Degrees(0.0),
        -1.0,
        Angle::Degrees(90.0),
    );
    magnet_list.push(Magnet2D::Rectangle(m2));

    // Calculate the magnetic field
    let field = points.get_field(&magnet_list);
    let units = "mm".to_string();

    let mag_toml = gen_magnet_toml_2d(&magnet_list)?;

    let sim_res = SimResult::new(mag_toml, points, units, field);
    println!("Saving to {:#?}", outfile);
    save_results(&sim_res, &outfile)?;
    println!("Done");

    Ok(())
}
