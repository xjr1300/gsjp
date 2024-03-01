use crate::mesh::mesh4::validate_mesh4_code;
use crate::mesh::{Coordinate, GSJPError, Mesh, Mesh1, Mesh2, Mesh3, Mesh4};

/// 4分の１地域メッシュの南端と北端の緯度の差
const MESH5_LAT_DIFF: f64 = 7.5 / 3600.0; // 7.5秒
/// 4分の１地域メッシュの西端と東端の経度の差
const MESH5_LON_DIFF: f64 = 11.25 / 3600.0; // 11.25秒

/// 4分の１地域メッシュ（分割地域メッシュ）
///
/// 2分の１地域メッシュを南北に2等分、東西に2等分した区画を示す。
/// 4分の1地域メッシュの辺の長さは約250mである。
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh5 {
    code: String,
}

impl Mesh5 {
    /// 4分の1地域メッシュを含む第1次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第1次地域区画
    pub fn mesh1(&self) -> Mesh1 {
        Mesh1::new(self.code[0..4].to_string()).unwrap()
    }

    /// 4分の1地域メッシュを含む第2次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第2次地域区画
    pub fn mesh2(&self) -> Mesh2 {
        Mesh2::new(self.code[0..6].to_string()).unwrap()
    }

    /// 4分の1地域メッシュを含む基準地域メッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 基準地域メッシュ
    pub fn mesh3(&self) -> Mesh3 {
        Mesh3::new(self.code[0..8].to_string()).unwrap()
    }

    /// 4分の1地域メッシュを含む2分の1地域メッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 2分の1地域メッシュ
    pub fn mesh4(&self) -> Mesh4 {
        Mesh4::new(self.code[0..9].to_string()).unwrap()
    }
}

impl Mesh for Mesh5 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh5_code(&code)?;

        Ok(Self { code })
    }

    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError> {
        let mesh4 = Mesh4::from_coordinate(coord)?;
        let lat_n = ((coord.lat() - mesh4.south()) / MESH5_LAT_DIFF).floor() as u8;
        let lon_n = ((coord.lon() - mesh4.west()) / MESH5_LON_DIFF).floor() as u8;
        let num = 2 * lat_n + 1 + lon_n;
        let code = format!("{}{}", mesh4.code(), num);

        Self::new(code)
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH5_LAT_DIFF
    }

    fn east(&self) -> f64 {
        self.west() + MESH5_LON_DIFF
    }

    fn south(&self) -> f64 {
        let south = self.mesh4().south();
        match self.code.chars().nth(9).unwrap() {
            '1' | '2' => south,
            '3' | '4' => south + MESH5_LAT_DIFF,
            _ => unreachable!(),
        }
    }

    fn west(&self) -> f64 {
        let west = self.mesh4().west();
        match self.code.chars().nth(9).unwrap() {
            '1' | '3' => west,
            '2' | '4' => west + MESH5_LON_DIFF,
            _ => unreachable!(),
        }
    }

    fn north_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(9).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 2 => {
                format!("{}{}", &self.code[0..9], n + 2,)
            }
            3 | 4 => {
                let mesh4 = self.mesh4().north_mesh()?;
                format!("{}{}", mesh4.code(), n - 2,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn east_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(9).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 3 => {
                format!("{}{}", &self.code[0..9], n + 1,)
            }
            2 | 4 => {
                let mesh4 = self.mesh4().east_mesh()?;
                format!("{}{}", mesh4.code(), n - 1,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn south_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(9).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 2 => {
                let mesh4 = self.mesh4().south_mesh()?;
                format!("{}{}", mesh4.code(), n + 2,)
            }
            3 | 4 => {
                format!("{}{}", &self.code[0..9], n - 2,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn west_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(9).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 3 => {
                let mesh4 = self.mesh4().west_mesh()?;
                format!("{}{}", mesh4.code(), n + 1,)
            }
            2 | 4 => {
                format!("{}{}", &self.code[0..9], n - 1,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }
}

pub(crate) fn validate_mesh5_code(code: &str) -> Result<(), GSJPError> {
    // 上位9桁が2分の1地域メッシュのメッシュコードであることを確認
    if code.len() != 10 {
        return Err(GSJPError::InvalidMeshCode);
    }
    validate_mesh4_code(&code[0..9])?;
    // 4分の1地域メッシュのメッシュコードの4分の1地域メッシュ部分について、緯度方向の値と経度方向の値を確認
    let num = &code.chars().nth(9).unwrap();
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
    fn mesh5_new_ok() {
        let codes = vec!["5339359911", "5339359912", "5339359913", "5339359914"];
        for code in codes {
            let mesh = Mesh5::new(code.to_string());
            assert!(mesh.is_ok(), "{}", code);
            assert_eq!(code, mesh.unwrap().code(), "{}", code);
        }
    }

    #[test]
    fn mesh5_new_err() {
        let codes = vec!["5339359910", "5339359915"];
        for code in codes {
            let mesh = Mesh5::new(code.to_string());
            assert!(mesh.is_err(), "{}", code);
        }
    }

    #[test]
    fn mesh5_from_coordinate_ok() {
        let mesh4 = Mesh4::new("533935991".to_string()).unwrap();
        let mesh4_south = mesh4.south();
        let mesh4_west = mesh4.west();
        let coord_mesh5_1 = Coordinate::new(mesh4_south, mesh4_west).unwrap();
        let coord_mesh5_2 = Coordinate::new(mesh4_south, mesh4_west + MESH5_LON_DIFF).unwrap();
        let coord_mesh5_3 =
            Coordinate::new(mesh4_south + MESH5_LAT_DIFF + EPSILON, mesh4_west).unwrap();
        let coord_mesh5_4 = Coordinate::new(
            mesh4_south + MESH5_LAT_DIFF + EPSILON,
            mesh4_west + MESH5_LON_DIFF,
        )
        .unwrap();
        let inputs = vec![
            (coord_mesh5_1, "5339359911"),
            (coord_mesh5_2, "5339359912"),
            (coord_mesh5_3, "5339359913"),
            (coord_mesh5_4, "5339359914"),
        ];

        for (coord, expected) in inputs {
            let mesh = Mesh5::from_coordinate(coord);
            assert!(mesh.is_ok(), "{}", expected);
            assert_eq!(expected, mesh.unwrap().code(), "{}", expected);
        }
    }

    #[test]
    fn mesh5_north_ok() {
        let south = 35.65833333;
        let inputs = vec![
            ("5339359911", south + MESH5_LAT_DIFF),
            ("5339359912", south + MESH5_LAT_DIFF),
            ("5339359913", south + MESH5_LAT_DIFF * 2.0),
            ("5339359914", south + MESH5_LAT_DIFF * 2.0),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
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
    fn mesh5_east_ok() {
        let west = 139.7375;
        let inputs = vec![
            ("5339359911", west + MESH5_LON_DIFF),
            ("5339359912", west + MESH5_LON_DIFF * 2.0),
            ("5339359913", west + MESH5_LON_DIFF),
            ("5339359914", west + MESH5_LON_DIFF * 2.0),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
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
    fn mesh5_south_ok() {
        let south = 35.65833333;
        let inputs = vec![
            ("5339359911", south),
            ("5339359912", south),
            ("5339359913", south + MESH5_LAT_DIFF),
            ("5339359914", south + MESH5_LAT_DIFF),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
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
    fn mesh5_west_ok() {
        let west = 139.7375;
        let inputs = vec![
            ("5339359911", west),
            ("5339359912", west + MESH5_LON_DIFF),
            ("5339359913", west),
            ("5339359914", west + MESH5_LON_DIFF),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
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
    fn mesh5_center_ok() {
        let code = String::from("5339359911");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh5::new(code).unwrap();
        let coord = mesh.center();
        let expected = south + MESH5_LAT_DIFF / 2.0;
        assert!(
            eq_f64(expected, coord.lat()),
            "expected: {}, actual: {}",
            expected,
            coord.lat()
        );
        let expected = west + MESH5_LON_DIFF / 2.0;
        assert!(
            eq_f64(expected, coord.lon()),
            "expected: {}, actual: {}",
            expected,
            coord.lon()
        );
    }

    #[test]
    fn mesh5_north_east_ok() {
        let code = String::from("5339359911");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh5::new(code).unwrap();
        let north_east = mesh.north_east();
        let expected = south + MESH5_LAT_DIFF;
        assert!(
            eq_f64(expected, north_east.lat()),
            "expected: {}, actual: {}",
            expected,
            north_east.lat()
        );
        let expected = west + MESH5_LON_DIFF;
        assert!(
            eq_f64(expected, north_east.lon()),
            "expected: {}, actual: {}",
            expected,
            north_east.lon()
        );
    }

    #[test]
    fn mesh5_south_east_ok() {
        let code = String::from("5339359911");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh5::new(code).unwrap();
        let south_east = mesh.south_east();
        let expected = south;
        assert!(
            eq_f64(expected, south_east.lat()),
            "expected: {}, actual: {}",
            expected,
            south_east.lat()
        );
        let expected = west + MESH5_LON_DIFF;
        assert!(
            eq_f64(expected, south_east.lon()),
            "expected: {}, actual: {}",
            expected,
            south_east.lon()
        );
    }

    #[test]
    fn mesh5_south_west_ok() {
        let code = String::from("5339359911");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh5::new(code).unwrap();
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
    fn mesh5_north_west_ok() {
        let code = String::from("5339359911");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh5::new(code).unwrap();
        let north_west = mesh.north_west();
        let expected = south + MESH5_LAT_DIFF;
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
    fn mesh5_north_mesh_ok() {
        let inputs = vec![
            ("5339359911", "5339359913"),
            ("5339359912", "5339359914"),
            ("5339359913", "5339359931"),
            ("5339359914", "5339359932"),
            ("5339779933", "5439070911"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
            let north_mesh = mesh.north_mesh().unwrap();
            assert_eq!(expected, north_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh5_east_mesh_ok() {
        let inputs = vec![
            ("5339359911", "5339359912"),
            ("5339359912", "5339359921"),
            ("5339359913", "5339359914"),
            ("5339359914", "5339359923"),
            ("5339779944", "5340709033"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
            let east_mesh = mesh.east_mesh().unwrap();
            assert_eq!(expected, east_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh5_south_mesh_ok() {
        let inputs = vec![
            ("5339359911", "5339358933"),
            ("5339359912", "5339358934"),
            ("5339359913", "5339359911"),
            ("5339359914", "5339359912"),
            ("5039000011", "4939709033"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
            let south_mesh = mesh.south_mesh().unwrap();
            assert_eq!(expected, south_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh5_west_mesh_ok() {
        let inputs = vec![
            ("5339359911", "5339359822"),
            ("5339359912", "5339359911"),
            ("5339359913", "5339359824"),
            ("5339359914", "5339359913"),
            ("5330309011", "5329379922"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh5::new(code.to_string()).unwrap();
            let west_mesh = mesh.west_mesh().unwrap();
            assert_eq!(expected, west_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh5_is_neighbor_ok() {
        let inputs = vec![
            ("5135463511", "5135463513", NeighborDirection::North),
            ("5135463511", "5135463512", NeighborDirection::East),
            ("5135463511", "5135462533", NeighborDirection::South),
            ("5135463511", "5135463422", NeighborDirection::West),
            ("5135709033", "5235000011", NeighborDirection::North),
            ("5135070922", "5136000011", NeighborDirection::East),
            ("5135000011", "5035709033", NeighborDirection::South),
            ("5135000011", "5134070922", NeighborDirection::West),
        ];
        for (code1, code2, expected) in inputs {
            let mesh1 = Mesh5::new(String::from(code1)).unwrap();
            let mesh2 = Mesh5::new(String::from(code2)).unwrap();
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
    fn mesh5_is_neighbor_none() {
        let inputs = vec![
            ("5135463511", "5135463514"), // 北東隣
            ("5135463511", "5135462534"), // 南東隣
            ("5135463511", "5135462444"), // 南西隣
            ("5135463511", "5135463424"), // 北西隣
            ("5135463511", "5135463531"), // 2つ北隣
            ("5135463511", "5135463521"), // 2つ東隣
            ("5135463511", "5135462531"), // 2つ南隣
            ("5135463511", "5135463421"), // 2つ西隣
        ];
        for (code1, code2) in inputs {
            let mesh1 = Mesh5::new(String::from(code1)).unwrap();
            let mesh2 = Mesh5::new(String::from(code2)).unwrap();
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
