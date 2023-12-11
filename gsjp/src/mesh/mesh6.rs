use crate::mesh::mesh5::validate_mesh5_code;
use crate::mesh::{Coordinate, GSJPError, Mesh, Mesh1, Mesh2, Mesh3, Mesh4, Mesh5};

/// 8分の１地域メッシュの南端と北端の緯度の差
const MESH6_LAT_DIFF: f64 = 3.75 / 3600.0; // 3.75秒
/// 8分の１地域メッシュの西端と東端の経度の差
const MESH6_LON_DIFF: f64 = 5.625 / 3600.0; // 5.625秒

/// 8分の１地域メッシュ（分割地域メッシュ）
///
/// 4分の1地域メッシュを南北に2等分、東西に2等分した区画を示す。
/// 8分の1地域メッシュの辺の長さは約125mである。
pub struct Mesh6 {
    code: String,
}

impl Mesh6 {
    /// 8分の1地域メッシュを含む第1次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第1次地域区画
    pub fn mesh1(&self) -> Mesh1 {
        Mesh1::new(self.code[0..4].to_string()).unwrap()
    }

    /// 8分の1地域メッシュを含む第2次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第2次地域区画
    pub fn mesh2(&self) -> Mesh2 {
        Mesh2::new(self.code[0..6].to_string()).unwrap()
    }

    /// 8分の1地域メッシュを含む基準地域メッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 基準地域メッシュ
    pub fn mesh3(&self) -> Mesh3 {
        Mesh3::new(self.code[0..8].to_string()).unwrap()
    }

    /// 8分の1地域メッシュを含む2分の1地域メッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 2分の1地域メッシュ
    pub fn mesh4(&self) -> Mesh4 {
        Mesh4::new(self.code[0..9].to_string()).unwrap()
    }

    /// 8分の1地域メッシュを含む4分の1地域メッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 4分の1地域メッシュ
    pub fn mesh5(&self) -> Mesh5 {
        Mesh5::new(self.code[0..10].to_string()).unwrap()
    }
}

impl Mesh for Mesh6 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh6_code(&code)?;

        Ok(Self { code })
    }

    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError> {
        let mesh5 = Mesh5::from_coordinate(coord)?;
        let lat_n = ((coord.lat() - mesh5.south()) / MESH6_LAT_DIFF).floor() as u8;
        let lon_n = ((coord.lon() - mesh5.west()) / MESH6_LON_DIFF).floor() as u8;
        let num = 2 * lat_n + 1 + lon_n;
        let code = format!("{}{}", mesh5.code(), num);

        Self::new(code)
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH6_LAT_DIFF
    }

    fn east(&self) -> f64 {
        self.west() + MESH6_LON_DIFF
    }

    fn south(&self) -> f64 {
        let south = self.mesh5().south();
        match self.code.chars().nth(10).unwrap() {
            '1' | '2' => south,
            '3' | '4' => south + MESH6_LAT_DIFF,
            _ => unreachable!(),
        }
    }

    fn west(&self) -> f64 {
        let west = self.mesh5().west();
        match self.code.chars().nth(10).unwrap() {
            '1' | '3' => west,
            '2' | '4' => west + MESH6_LON_DIFF,
            _ => unreachable!(),
        }
    }

    fn north_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(10).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 2 => {
                format!("{}{}", &self.code[0..10], n + 2,)
            }
            3 | 4 => {
                let mesh5 = self.mesh5().north_mesh()?;
                format!("{}{}", mesh5.code(), n - 2,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn east_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(10).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 3 => {
                format!("{}{}", &self.code[0..10], n + 1,)
            }
            2 | 4 => {
                let mesh5 = self.mesh5().east_mesh()?;
                format!("{}{}", mesh5.code(), n - 1,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn south_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(10).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 2 => {
                let mesh5 = self.mesh5().south_mesh()?;
                format!("{}{}", mesh5.code(), n + 2,)
            }
            3 | 4 => {
                format!("{}{}", &self.code[0..10], n - 2,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn west_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(10).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 3 => {
                let mesh5 = self.mesh5().west_mesh()?;
                format!("{}{}", mesh5.code(), n + 1,)
            }
            2 | 4 => {
                format!("{}{}", &self.code[0..10], n - 1,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }
}

pub(crate) fn validate_mesh6_code(code: &str) -> Result<(), GSJPError> {
    // 上位10桁が2分の1地域メッシュのメッシュコードであることを確認
    if code.len() != 11 {
        return Err(GSJPError::InvalidMeshCode);
    }
    validate_mesh5_code(&code[0..10])?;
    // 8分の1地域メッシュのメッシュコードの8分の1地域メッシュ部分について、緯度方向の値と経度方向の値を確認
    let num = &code.chars().nth(10).unwrap();
    if !(&'1'..=&'4').contains(&num) {
        return Err(GSJPError::InvalidMeshCode);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::tests::{eq_f64, EPSILON};
    use crate::mesh::NeighborDirection;

    #[test]
    fn mesh6_new_ok() {
        let codes = vec!["53393599111", "53393599112", "53393599113", "53393599114"];
        for code in codes {
            let mesh = Mesh6::new(code.to_string());
            assert!(mesh.is_ok(), "{}", code);
            assert_eq!(code, mesh.unwrap().code(), "{}", code);
        }
    }

    #[test]
    fn mesh6_new_err() {
        let codes = vec!["53393599110", "53393599115"];
        for code in codes {
            let mesh = Mesh6::new(code.to_string());
            assert!(mesh.is_err(), "{}", code);
        }
    }

    #[test]
    fn mesh6_from_coordinate_ok() {
        let mesh5 = Mesh5::new("5339359911".to_string()).unwrap();
        let mesh5_south = mesh5.south();
        let mesh5_west = mesh5.west();
        let coord_mesh6_1 = Coordinate::new(mesh5_south, mesh5_west).unwrap();
        let coord_mesh6_2 = Coordinate::new(mesh5_south, mesh5_west + MESH6_LON_DIFF).unwrap();
        let coord_mesh6_3 =
            Coordinate::new(mesh5_south + MESH6_LAT_DIFF + EPSILON, mesh5_west).unwrap();
        let coord_mesh6_4 = Coordinate::new(
            mesh5_south + MESH6_LAT_DIFF + EPSILON,
            mesh5_west + MESH6_LON_DIFF,
        )
        .unwrap();
        let inputs = vec![
            (coord_mesh6_1, "53393599111"),
            (coord_mesh6_2, "53393599112"),
            (coord_mesh6_3, "53393599113"),
            (coord_mesh6_4, "53393599114"),
        ];

        for (coord, expected) in inputs {
            let mesh = Mesh6::from_coordinate(coord);
            assert!(mesh.is_ok(), "{}", expected);
            assert_eq!(expected, mesh.unwrap().code(), "{}", expected);
        }
    }

    #[test]
    fn mesh6_north_ok() {
        let south = 35.65833333;
        let inputs = vec![
            ("53393599111", south + MESH6_LAT_DIFF),
            ("53393599112", south + MESH6_LAT_DIFF),
            ("53393599113", south + MESH6_LAT_DIFF * 2.0),
            ("53393599114", south + MESH6_LAT_DIFF * 2.0),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            assert!(
                eq_f64(expected, mesh.north()),
                "expected: {}, actual: {}, {}",
                expected,
                mesh.north(),
                code
            );
        }
    }

    #[test]
    fn mesh6_east_ok() {
        let west = 139.7375;
        let inputs = vec![
            ("53393599111", west + MESH6_LON_DIFF),
            ("53393599112", west + MESH6_LON_DIFF * 2.0),
            ("53393599113", west + MESH6_LON_DIFF),
            ("53393599114", west + MESH6_LON_DIFF * 2.0),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            assert!(
                eq_f64(expected, mesh.east()),
                "expected: {}, actual: {}, {}",
                expected,
                mesh.east(),
                code
            );
        }
    }

    #[test]
    fn mesh6_south_ok() {
        let south = 35.65833333;
        let inputs = vec![
            ("53393599111", south),
            ("53393599112", south),
            ("53393599113", south + MESH6_LAT_DIFF),
            ("53393599114", south + MESH6_LAT_DIFF),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            assert!(
                eq_f64(expected, mesh.south()),
                "expected: {}, actual: {}, {}",
                expected,
                mesh.south(),
                code
            );
        }
    }

    #[test]
    fn mesh6_west_ok() {
        let west = 139.7375;
        let inputs = vec![
            ("53393599111", west),
            ("53393599112", west + MESH6_LON_DIFF),
            ("53393599113", west),
            ("53393599114", west + MESH6_LON_DIFF),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            assert!(
                eq_f64(expected, mesh.west()),
                "expected: {}, actual: {}, {}",
                expected,
                mesh.west(),
                code
            );
        }
    }

    #[test]
    fn mesh6_center_ok() {
        let code = String::from("53393599111");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh6::new(code).unwrap();
        let coord = mesh.center();
        let expected = south + MESH6_LAT_DIFF / 2.0;
        assert!(
            eq_f64(expected, coord.lat()),
            "expected: {}, actual: {}",
            expected,
            coord.lat()
        );
        let expected = west + MESH6_LON_DIFF / 2.0;
        assert!(
            eq_f64(expected, coord.lon()),
            "expected: {}, actual: {}",
            expected,
            coord.lon()
        );
    }

    #[test]
    fn mesh6_north_east_ok() {
        let code = String::from("53393599111");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh6::new(code).unwrap();
        let north_east = mesh.north_east();
        let expected = south + MESH6_LAT_DIFF;
        assert!(
            eq_f64(expected, north_east.lat()),
            "expected: {}, actual: {}",
            expected,
            north_east.lat()
        );
        let expected = west + MESH6_LON_DIFF;
        assert!(
            eq_f64(expected, north_east.lon()),
            "expected: {}, actual: {}",
            expected,
            north_east.lon()
        );
    }

    #[test]
    fn mesh6_south_east_ok() {
        let code = String::from("53393599111");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh6::new(code).unwrap();
        let south_east = mesh.south_east();
        let expected = south;
        assert!(
            eq_f64(expected, south_east.lat()),
            "expected: {}, actual: {}",
            expected,
            south_east.lat()
        );
        let expected = west + MESH6_LON_DIFF;
        assert!(
            eq_f64(expected, south_east.lon()),
            "expected: {}, actual: {}",
            expected,
            south_east.lon()
        );
    }

    #[test]
    fn mesh6_south_west_ok() {
        let code = String::from("53393599111");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh6::new(code).unwrap();
        let south_west = mesh.south_west();
        let expected = south;
        assert!(
            eq_f64(expected, south_west.lat()),
            "expected: {}, actual: {}",
            expected,
            south_west.lat()
        );
        let expected = west;
        assert!(
            eq_f64(expected, south_west.lon()),
            "expected: {}, actual: {}",
            expected,
            south_west.lon()
        );
    }

    #[test]
    fn mesh6_north_west_ok() {
        let code = String::from("53393599111");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh6::new(code).unwrap();
        let north_west = mesh.north_west();
        let expected = south + MESH6_LAT_DIFF;
        assert!(
            eq_f64(expected, north_west.lat()),
            "expected: {}, actual: {}",
            expected,
            north_west.lat()
        );
        let expected = west;
        assert!(
            eq_f64(expected, north_west.lon()),
            "expected: {}, actual: {}",
            expected,
            north_west.lon()
        );
    }

    #[test]
    fn mesh6_north_mesh_ok() {
        let inputs = vec![
            ("53393599111", "53393599113"),
            ("53393599112", "53393599114"),
            ("53393599113", "53393599131"),
            ("53393599114", "53393599132"),
            ("53397799333", "54390709111"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            let north_mesh = mesh.north_mesh().unwrap();
            assert_eq!(expected, north_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh6_east_mesh_ok() {
        let inputs = vec![
            ("53393599111", "53393599112"),
            ("53393599112", "53393599121"),
            ("53393599113", "53393599114"),
            ("53393599114", "53393599123"),
            ("53397799444", "53407090333"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            let east_mesh = mesh.east_mesh().unwrap();
            assert_eq!(expected, east_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh6_south_mesh_ok() {
        let inputs = vec![
            ("53393599111", "53393589333"),
            ("53393599112", "53393589334"),
            ("53393599113", "53393599111"),
            ("53393599114", "53393599112"),
            ("50390000111", "49397090333"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            let south_mesh = mesh.south_mesh().unwrap();
            assert_eq!(expected, south_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh6_west_mesh_ok() {
        let inputs = vec![
            ("53393599111", "53393598222"),
            ("53393599112", "53393599111"),
            ("53393599113", "53393598224"),
            ("53393599114", "53393599113"),
            ("53303090111", "53293799222"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh6::new(code.to_string()).unwrap();
            let west_mesh = mesh.west_mesh().unwrap();
            assert_eq!(expected, west_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh6_is_neighbor_ok() {
        let inputs = vec![
            ("51354635111", "51354635113", NeighborDirection::North),
            ("51354635111", "51354635112", NeighborDirection::East),
            ("51354635111", "51354625333", NeighborDirection::South),
            ("51354635111", "51354634222", NeighborDirection::West),
            ("51357090333", "52350000111", NeighborDirection::North),
            ("51350709222", "51360000111", NeighborDirection::East),
            ("51350000111", "50357090333", NeighborDirection::South),
            ("51350000111", "51340709222", NeighborDirection::West),
        ];
        for (code1, code2, expected) in inputs {
            let mesh1 = Mesh6::new(String::from(code1)).unwrap();
            let mesh2 = Mesh6::new(String::from(code2)).unwrap();
            assert_eq!(
                expected,
                mesh1.is_neighboring(&mesh2).unwrap(),
                "expected: {:?}, actual: {:?}, {}",
                expected,
                mesh1.is_neighboring(&mesh2).unwrap(),
                code2
            );
        }
    }

    #[test]
    fn mesh6_is_neighbor_none() {
        let inputs = vec![
            ("51354635111", "51354635114"), // 北東隣
            ("51354635111", "51354625334"), // 南東隣
            ("51354635111", "51354624444"), // 南西隣
            ("51354635111", "51354634244"), // 北西隣
            ("51354635111", "51354635131"), // 2つ北隣
            ("51354635111", "51354635121"), // 2つ東隣
            ("51354635111", "51354625131"), // 2つ南隣
            ("51354635111", "51354634121"), // 2つ西隣
        ];
        for (code1, code2) in inputs {
            let mesh1 = Mesh6::new(String::from(code1)).unwrap();
            let mesh2 = Mesh6::new(String::from(code2)).unwrap();
            assert_eq!(
                NeighborDirection::None,
                mesh1.is_neighboring(&mesh2).unwrap(),
                "actual: {:?}, {}",
                mesh1.is_neighboring(&mesh2).unwrap(),
                code2
            );
        }
    }
}
