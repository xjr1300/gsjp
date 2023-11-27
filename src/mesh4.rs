use crate::mesh3::validate_mesh3_code;
use crate::{Coordinate, GSJPError, Mesh, Mesh1, Mesh2, Mesh3};

/// 2分の１地域メッシュの南端と北端の緯度の差
const MESH4_LAT_DIFF: f64 = 15.0 / 3600.0; // 15秒
/// 2分の１地域メッシュの西端と東端の経度の差
const MESH4_LON_DIFF: f64 = 22.5 / 3600.0; // 22.5秒

/// 2分の１地域メッシュ（分割地域メッシュ）
///
/// 標準地域メッシュを南北に2等分、東西に2等分した区画を示す。
/// 2分の1地域メッシュの辺の長さは約500mである。
pub struct Mesh4 {
    code: String,
}

impl Mesh4 {
    /// 2分の1地域メッシュを含む第1次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第1次地域区画
    pub fn mesh1(&self) -> Mesh1 {
        Mesh1::new(self.code[0..4].to_string()).unwrap()
    }

    /// 2分の1地域メッシュを含む第2次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第2次地域区画
    pub fn mesh2(&self) -> Mesh2 {
        Mesh2::new(self.code[0..6].to_string()).unwrap()
    }

    /// 2分の1地域メッシュを含む基準地域メッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 基準地域メッシュ
    pub fn mesh3(&self) -> Mesh3 {
        Mesh3::new(self.code[0..8].to_string()).unwrap()
    }
}

impl Mesh for Mesh4 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh4_code(&code)?;

        Ok(Self { code })
    }

    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError> {
        let mesh3 = Mesh3::from_coordinate(coord)?;
        let lat_n = ((coord.lat() - mesh3.south()) / MESH4_LAT_DIFF).floor() as u8;
        let lon_n = ((coord.lon() - mesh3.west()) / MESH4_LON_DIFF).floor() as u8;
        let num = 2 * lat_n + 1 + lon_n;
        let code = format!("{}{}", mesh3.code(), num);

        Self::new(code)
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH4_LAT_DIFF
    }

    fn east(&self) -> f64 {
        self.west() + MESH4_LON_DIFF
    }

    fn south(&self) -> f64 {
        let south = self.mesh3().south();
        match self.code.chars().nth(8).unwrap() {
            '1' | '2' => south,
            '3' | '4' => south + MESH4_LAT_DIFF,
            _ => unreachable!(),
        }
    }

    fn west(&self) -> f64 {
        let west = self.mesh3().west();
        match self.code.chars().nth(8).unwrap() {
            '1' | '3' => west,
            '2' | '4' => west + MESH4_LON_DIFF,
            _ => unreachable!(),
        }
    }

    fn north_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(8).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 2 => {
                format!("{}{}", &self.code[0..8], n + 2,)
            }
            3 | 4 => {
                let mesh3 = self.mesh3().north_mesh()?;
                format!("{}{}", mesh3.code(), n - 2,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn east_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(8).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 3 => {
                format!("{}{}", &self.code[0..8], n + 1,)
            }
            2 | 4 => {
                let mesh3 = self.mesh3().east_mesh()?;
                format!("{}{}", mesh3.code(), n - 1,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn south_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(8).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 2 => {
                let mesh3 = self.mesh3().south_mesh()?;
                format!("{}{}", mesh3.code(), n + 2,)
            }
            3 | 4 => {
                format!("{}{}", &self.code[0..8], n - 2,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }

    fn west_mesh(&self) -> Result<Self, GSJPError> {
        let n = self.code.chars().nth(8).unwrap().to_digit(10).unwrap();
        let code = match n {
            1 | 3 => {
                let mesh3 = self.mesh3().west_mesh()?;
                format!("{}{}", mesh3.code(), n + 1,)
            }
            2 | 4 => {
                format!("{}{}", &self.code[0..8], n - 1,)
            }
            _ => unreachable!(),
        };

        Self::new(code)
    }
}

pub(crate) fn validate_mesh4_code(code: &str) -> Result<(), GSJPError> {
    // 上位8桁が標準地域メッシュのメッシュコードであることを確認
    if code.len() != 9 {
        return Err(GSJPError::InvalidMeshCode);
    }
    validate_mesh3_code(&code[0..8])?;
    // 2分の1地域メッシュのメッシュコードの2分の1地域メッシュ部分について、緯度方向の値と経度方向の値を確認
    let num = &code.chars().nth(8).unwrap();
    if !(&'1'..=&'4').contains(&num) {
        return Err(GSJPError::InvalidMeshCode);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::{eq_f64, EPSILON},
        NeighborDirection,
    };

    #[test]
    fn mesh4_new_ok() {
        let codes = vec!["533935991", "533935992", "533935993", "533935994"];
        for code in codes {
            let mesh = Mesh4::new(code.to_string());
            assert!(mesh.is_ok(), "{}", code);
            assert_eq!(code, mesh.unwrap().code(), "{}", code);
        }
    }

    #[test]
    fn mesh4_new_err() {
        let codes = vec!["533935990", "533935995"];
        for code in codes {
            let mesh = Mesh4::new(code.to_string());
            assert!(mesh.is_err(), "{}", code);
        }
    }

    #[test]
    fn mesh4_from_coordinate_ok() {
        // 東京タワーを含む基準地域メッシュ
        let mesh3 = Mesh3::new("53393599".to_string()).unwrap();
        let coord_mesh4_1 = Coordinate::new(mesh3.south(), mesh3.west()).unwrap();
        let coord_mesh4_2 =
            Coordinate::new(mesh3.south(), mesh3.west() + MESH4_LON_DIFF + EPSILON).unwrap();
        let coord_mesh4_3 = Coordinate::new(mesh3.south() + MESH4_LAT_DIFF, mesh3.west()).unwrap();
        let coord_mesh4_4 = Coordinate::new(
            mesh3.south() + MESH4_LAT_DIFF,
            mesh3.west() + MESH4_LON_DIFF + EPSILON,
        )
        .unwrap();
        let inputs = vec![
            (coord_mesh4_1, "533935991"),
            (coord_mesh4_2, "533935992"),
            (coord_mesh4_3, "533935993"),
            (coord_mesh4_4, "533935994"),
        ];

        for (coord, expected) in inputs {
            let mesh = Mesh4::from_coordinate(coord);
            assert!(mesh.is_ok(), "{}", expected);
            assert_eq!(expected, mesh.unwrap().code(), "{}", expected);
        }
    }

    #[test]
    fn mesh4_north_ok() {
        let south = 35.65833333;
        let inputs = vec![
            ("533935991", south + MESH4_LAT_DIFF),
            ("533935992", south + MESH4_LAT_DIFF),
            ("533935993", south + MESH4_LAT_DIFF * 2.0),
            ("533935994", south + MESH4_LAT_DIFF * 2.0),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
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
    fn mesh4_east_ok() {
        let west = 139.7375;
        let inputs = vec![
            ("533935991", west + MESH4_LON_DIFF),
            ("533935992", west + MESH4_LON_DIFF * 2.0),
            ("533935993", west + MESH4_LON_DIFF),
            ("533935994", west + MESH4_LON_DIFF * 2.0),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
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
    fn mesh4_south_ok() {
        let south = 35.65833333;
        let inputs = vec![
            ("533935991", south),
            ("533935992", south),
            ("533935993", south + MESH4_LAT_DIFF),
            ("533935994", south + MESH4_LAT_DIFF),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
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
    fn mesh4_west_ok() {
        let west = 139.7375;
        let inputs = vec![
            ("533935991", west),
            ("533935992", west + MESH4_LON_DIFF),
            ("533935993", west),
            ("533935994", west + MESH4_LON_DIFF),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
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
    fn mesh4_center_ok() {
        let code = String::from("533935991");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh4::new(code).unwrap();
        let coord = mesh.center();
        let expected = south + MESH4_LAT_DIFF / 2.0;
        assert!(
            eq_f64(expected, coord.lat()),
            "expected: {}, actual: {}",
            expected,
            coord.lat()
        );
        let expected = west + MESH4_LON_DIFF / 2.0;
        assert!(
            eq_f64(expected, coord.lon()),
            "expected: {}, actual: {}",
            expected,
            coord.lon()
        );
    }

    #[test]
    fn mesh4_north_east_ok() {
        let code = String::from("533935991");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh4::new(code).unwrap();
        let north_east = mesh.north_east();
        let expected = south + MESH4_LAT_DIFF;
        assert!(
            eq_f64(expected, north_east.lat()),
            "expected: {}, actual: {}",
            expected,
            north_east.lat()
        );
        let expected = west + MESH4_LON_DIFF;
        assert!(
            eq_f64(expected, north_east.lon()),
            "expected: {}, actual: {}",
            expected,
            north_east.lon()
        );
    }

    #[test]
    fn mesh4_south_east_ok() {
        let code = String::from("533935991");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh4::new(code).unwrap();
        let south_east = mesh.south_east();
        let expected = south;
        assert!(
            eq_f64(expected, south_east.lat()),
            "expected: {}, actual: {}",
            expected,
            south_east.lat()
        );
        let expected = west + MESH4_LON_DIFF;
        assert!(
            eq_f64(expected, south_east.lon()),
            "expected: {}, actual: {}",
            expected,
            south_east.lon()
        );
    }

    #[test]
    fn mesh4_south_west_ok() {
        let code = String::from("533935991");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh4::new(code).unwrap();
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
    fn mesh4_north_west_ok() {
        let code = String::from("533935991");
        let south = 35.65833333;
        let west = 139.7375;
        let mesh = Mesh4::new(code).unwrap();
        let north_west = mesh.north_west();
        let expected = south + MESH4_LAT_DIFF;
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
    fn mesh4_north_mesh_ok() {
        let inputs = vec![
            ("533935991", "533935993"),
            ("533935992", "533935994"),
            ("533935993", "533945091"),
            ("533935994", "533945092"),
            ("533977993", "543907091"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
            let north_mesh = mesh.north_mesh().unwrap();
            assert_eq!(expected, north_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh4_east_mesh_ok() {
        let inputs = vec![
            ("533935991", "533935992"),
            ("533935992", "533936901"),
            ("533935993", "533935994"),
            ("533935994", "533936903"),
            ("533977994", "534070903"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
            let east_mesh = mesh.east_mesh().unwrap();
            assert_eq!(expected, east_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh4_south_mesh_ok() {
        let inputs = vec![
            ("533935991", "533935893"),
            ("533935992", "533935894"),
            ("533935993", "533935991"),
            ("533935994", "533935992"),
            ("503900001", "493970903"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
            let south_mesh = mesh.south_mesh().unwrap();
            assert_eq!(expected, south_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh4_west_mesh_ok() {
        let inputs = vec![
            ("533935991", "533935982"),
            ("533935992", "533935991"),
            ("533935993", "533935984"),
            ("533935994", "533935993"),
            ("533030901", "532937992"),
        ];
        for (code, expected) in inputs {
            let mesh = Mesh4::new(code.to_string()).unwrap();
            let west_mesh = mesh.west_mesh().unwrap();
            assert_eq!(expected, west_mesh.code(), "{}", code);
        }
    }

    #[test]
    fn mesh4_is_neighbor_ok() {
        let inputs = vec![
            ("51354635", "51354645", NeighborDirection::North),
            ("51354635", "51354636", NeighborDirection::East),
            ("51354635", "51354625", NeighborDirection::South),
            ("51354635", "51354634", NeighborDirection::West),
            ("51357090", "52350000", NeighborDirection::North),
            ("51350709", "51360000", NeighborDirection::East),
            ("51350000", "50357090", NeighborDirection::South),
            ("51350000", "51340709", NeighborDirection::West),
        ];
        for (code1, code2, expected) in inputs {
            let mesh1 = Mesh3::new(String::from(code1)).unwrap();
            let mesh2 = Mesh3::new(String::from(code2)).unwrap();
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
    fn mesh4_is_neighbor_none() {
        let inputs = vec![
            ("513546351", "513546354"), // 北東隣
            ("513546351", "513546254"), // 南東隣
            ("513546351", "513546254"), // 南西隣
            ("513546351", "513546343"), // 北西隣
            ("513546351", "513546451"), // 2つ北隣
            ("513546351", "513546361"), // 2つ東隣
            ("513546351", "513546151"), // 2つ南隣
            ("513546351", "513546251"), // 2つ西隣
        ];
        for (code1, code2) in inputs {
            let mesh1 = Mesh4::new(String::from(code1)).unwrap();
            let mesh2 = Mesh4::new(String::from(code2)).unwrap();
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
