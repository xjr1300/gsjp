use crate::mesh::{mesh1::validate_mesh1_code, Coordinate, GSJPError, Mesh, Mesh1};

/// 第2次地域区画の南端と北端の緯度の差
const MESH2_LAT_DIFF: f64 = 5.0 / 60.0; // 5分
/// 第2次地域区画の西端と東端の緯度の差
const MESH2_LON_DIFF: f64 = 7.0 / 60.0 + 30.0 / 3600.0; // 7分30秒

/// 第2次地域区画（統合地域メッシュ）
///
/// 第1次地域区画を南北に8等分、東西に8等分した区画を示す。
/// 第2次地域区画の辺の長さは約10kmである。
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh2 {
    code: String,
}

impl Mesh2 {
    /// 第2次地域区画を含む第1次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第1次地域区画
    pub fn mesh1(&self) -> Mesh1 {
        Mesh1::new(self.code[0..4].to_string()).unwrap()
    }
}

impl Mesh for Mesh2 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh2_code(&code)?;

        Ok(Self { code })
    }

    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError> {
        let mesh1 = Mesh1::from_coordinate(coord)?;
        let lat_n = ((coord.lat() - mesh1.south()) / MESH2_LAT_DIFF) as u8;
        let lon_n = ((coord.lon() - mesh1.west()) / MESH2_LON_DIFF) as u8;
        let code = format!("{}{}{}", mesh1.code(), lat_n, lon_n);

        Self::new(code)
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH2_LAT_DIFF
    }

    fn east(&self) -> f64 {
        self.west() + MESH2_LON_DIFF
    }

    fn south(&self) -> f64 {
        self.mesh1().south()
            + MESH2_LAT_DIFF * self.code.chars().nth(4).unwrap().to_digit(10).unwrap() as f64
    }

    fn west(&self) -> f64 {
        self.mesh1().west()
            + MESH2_LON_DIFF * self.code.chars().nth(5).unwrap().to_digit(10).unwrap() as f64
    }

    fn north_mesh(&self) -> Result<Self, GSJPError> {
        let lat_idx = self.code.chars().nth(4).unwrap().to_digit(10).unwrap();
        let code = if lat_idx == 7 {
            let mesh1 = self.mesh1().north_mesh()?;
            format!("{}0{}", mesh1.code(), self.code.chars().nth(5).unwrap())
        } else {
            format!(
                "{}{}{}",
                &self.code[0..4],
                lat_idx + 1,
                self.code.chars().nth(5).unwrap(),
            )
        };

        Self::new(code)
    }

    fn east_mesh(&self) -> Result<Self, GSJPError> {
        let lon_idx = self.code.chars().nth(5).unwrap().to_digit(10).unwrap();
        let code = if lon_idx == 7 {
            let mesh1 = self.mesh1().east_mesh()?;
            format!("{}{}0", mesh1.code(), self.code.chars().nth(4).unwrap())
        } else {
            format!("{}{}", &self.code[0..5], lon_idx + 1,)
        };

        Self::new(code)
    }

    fn south_mesh(&self) -> Result<Self, GSJPError> {
        let lat_idx = self.code.chars().nth(4).unwrap().to_digit(10).unwrap();
        let code = if lat_idx == 0 {
            let mesh1 = self.mesh1().south_mesh()?;
            format!("{}7{}", mesh1.code(), self.code.chars().nth(5).unwrap())
        } else {
            format!(
                "{}{}{}",
                &self.code[0..4],
                lat_idx - 1,
                self.code.chars().nth(5).unwrap(),
            )
        };

        Self::new(code)
    }

    fn west_mesh(&self) -> Result<Self, GSJPError> {
        let lon_idx = self.code.chars().nth(5).unwrap().to_digit(10).unwrap();
        let code = if lon_idx == 0 {
            let mesh1 = self.mesh1().west_mesh()?;
            format!("{}{}7", mesh1.code(), self.code.chars().nth(4).unwrap())
        } else {
            format!("{}{}", &self.code[0..5], lon_idx - 1,)
        };

        Self::new(code)
    }
}

/// 第2次地域区画のメッシュコードを検証する。
///
/// # 引数
///
/// * `code` - メッシュコード
///
/// # 戻り値
///
/// `()`
pub(crate) fn validate_mesh2_code(code: &str) -> Result<(), GSJPError> {
    // 上位4桁が第1次地域区画のメッシュコードであることを確認
    if code.len() != 6 {
        return Err(GSJPError::InvalidMeshCode);
    }
    validate_mesh1_code(&code[0..4])?;
    // 第2次地域区画のメッシュコードの第2次地域区画部分について、緯度方向の値と経度方向の値を確認
    let lat = &code.chars().nth(4).unwrap();
    if !(&'0'..=&'7').contains(&lat) {
        return Err(GSJPError::InvalidMeshCode);
    }
    let lon = &code.chars().nth(5).unwrap();
    if !(&'0'..=&'7').contains(&lon) {
        return Err(GSJPError::InvalidMeshCode);
    }

    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::mesh::tests::{eq_f64, EPSILON};
    use crate::mesh::{NeighborDirection, EASTERNMOST, NORTHERNMOST, SOUTHERNMOST, WESTERNMOST};

    #[test]
    fn mesh2_new_ok() {
        // 東京付近の第1次地域区画の北東端の第2次地域区画
        assert!(Mesh2::new(String::from("533977")).is_ok());
        // 東京付近の第1次地域区画の南東端の第2次地域区画
        assert!(Mesh2::new(String::from("533907")).is_ok());
        // 東京付近の第1次地域区画の南西端の第2次地域区画
        assert!(Mesh2::new(String::from("533900")).is_ok());
        // 東京付近の第1次地域区画の北西端の第2次地域区画
        assert!(Mesh2::new(String::from("533970")).is_ok());
    }

    #[test]
    fn mesh2_new_err() {
        // 下位2桁の数値が不正
        assert!(Mesh2::new(String::from("695480")).is_err(), "695480");
        assert!(Mesh2::new(String::from("695408")).is_err(), "695408");
        // 最北東端(714977)の1つ北側の第2次地域区画
        assert!(Mesh2::new(String::from("724907")).is_err(), "724907");
        // 最北東端の1つ東側の第2次地域区画
        assert!(Mesh2::new(String::from("725070")).is_err(), "725070");
        // 最南東端(304907)の1つ南側の第2次地域区画
        assert!(Mesh2::new(String::from("294977")).is_err(), "304877");
        // 最南東端の1つ東側の第2次地域区画
        assert!(Mesh2::new(String::from("305000")).is_err(), "305000");
        // 最南西端(301800)の1つ南側の第2次地域区画
        assert!(Mesh2::new(String::from("291870")).is_err(), "291870");
        // 最南西端の1つ西側の第2次地域区画
        assert!(Mesh2::new(String::from("301707")).is_err(), "301707");
        // 最北西端(711870)の1つ北側の第2次地域区画
        assert!(Mesh2::new(String::from("721800")).is_err(), "721800");
        // 最北西端の1つ西側の第2次地域区画
        assert!(Mesh2::new(String::from("711777")).is_err(), "711777");
    }

    #[test]
    #[rustfmt::skip]
    fn mesh2_from_coordinate_ok() {
        let inputs = vec![
            // 東京付近の第1次地域区画の北東端の第2次地域区画の中心座標
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH2_LAT_DIFF * 7.5,
                    139.0 + MESH2_LON_DIFF * 7.5,
                )
                .unwrap(),
                "533977",
            ),
            // 東京付近の第1次地域区画の北東端の第2次地域区画
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH2_LAT_DIFF * 8.0  - EPSILON,
                    139.0 + MESH2_LON_DIFF * 8.0 - EPSILON,
                )
                .unwrap(),
                "533977",
            ),
            // 東京付近の第1次地域区画の南東端の第2次地域区画の中心座標
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH2_LAT_DIFF / 2.0,
                    139.0 + MESH2_LON_DIFF * 7.5
                ).unwrap(),
                "533907",
            ),
            // 東京付近の第1次地域区画の南東端の第2次地域区画
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + EPSILON,
                    139.0 + MESH2_LON_DIFF * 8.0 - EPSILON,
                ).unwrap(),
                "533907",
            ),
            // 東京付近の第1次地域区画の南西端の第2次地域区画の中心座標
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH2_LAT_DIFF / 2.0,
                    139.0 + MESH2_LON_DIFF / 2.0,
                ).unwrap(),
                "533900"
            ),
            // 東京付近の第1次地域区画の南西端の第2次地域区画
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + EPSILON,
                    139.0 + EPSILON,
                ).unwrap(),
                "533900"
            ),
            // 東京付近の第1次地域区画の北西端の第2次地域区画の中心座標
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH2_LAT_DIFF * 7.5,
                    139.0 + MESH2_LON_DIFF / 2.0,
                ).unwrap(),
                "533970",
            ),
            // 東京付近の第1次地域区画の北西端の第2次地域区画
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH2_LAT_DIFF * 8.0 - EPSILON,
                    139.0 + EPSILON,
                ).unwrap(),
                "533970",
            ),
            // 東京タワーを含む第2次地域区画
            (
                Coordinate::new(
                    35.65858404079,
                    139.74543164468,
                ).unwrap(),
                "533935",
            ),
        ];
        for (coord, code) in inputs {
            let mesh = Mesh2::from_coordinate(coord).unwrap();
            assert_eq!(mesh.code(), code, "{}", code);
        }
    }

    #[test]
    fn mesh2_from_coordinate_err() {
        let data = vec![
            Coordinate::new(NORTHERNMOST + 1.0, WESTERNMOST).unwrap(),
            Coordinate::new(NORTHERNMOST, WESTERNMOST - 1.0).unwrap(),
            Coordinate::new(SOUTHERNMOST - 1.0, WESTERNMOST).unwrap(),
            Coordinate::new(SOUTHERNMOST, EASTERNMOST + 1.0).unwrap(),
        ];
        for coord in data {
            assert!(Mesh2::from_coordinate(coord).is_err());
        }
    }

    pub(crate) fn mesh2_south(code: &str) -> f64 {
        &code[0..2].parse::<f64>().unwrap() / 1.5
            + code.chars().nth(4).unwrap().to_digit(10).unwrap() as f64 * MESH2_LAT_DIFF
    }

    pub(crate) fn mesh2_west(code: &str) -> f64 {
        &code[2..4].parse::<f64>().unwrap()
            + 100.0
            + code.chars().nth(5).unwrap().to_digit(10).unwrap() as f64 * MESH2_LON_DIFF
    }

    #[test]
    fn mesh2_north_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let expected = mesh2_south(code) + MESH2_LAT_DIFF;
        assert!(
            eq_f64(mesh.north(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.north()
        );
    }

    #[test]
    fn mesh2_east_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let expected = mesh2_west(code) + MESH2_LON_DIFF;
        assert!(
            eq_f64(mesh.east(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.east()
        );
    }

    #[test]
    fn mesh2_south_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let expected = mesh2_south(code);
        assert!(
            eq_f64(mesh.south(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.south()
        );
    }

    #[test]
    fn mesh2_west_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let expected = mesh2_west(code);
        assert!(
            eq_f64(mesh.west(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.west()
        );
    }

    #[test]
    fn mesh2_center_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let lat_center = mesh2_south(code) + MESH2_LAT_DIFF / 2.0;
        let lon_center = mesh2_west(code) + MESH2_LON_DIFF / 2.0;
        let center = mesh.center();
        assert!(
            eq_f64(center.lat(), lat_center),
            "expected: {}, actual: {}",
            lat_center,
            center.lat()
        );
        assert!(
            eq_f64(center.lon(), lon_center),
            "expected: {}, actual: {}",
            lon_center,
            center.lon()
        );
    }

    #[test]
    fn mesh2_north_east_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let north = mesh2_south(code) + MESH2_LAT_DIFF;
        let east = mesh2_west(code) + MESH2_LON_DIFF;
        let north_east = mesh.north_east();
        assert!(
            eq_f64(north_east.lat(), north),
            "expected: {}, actual: {}",
            north,
            north_east.lat()
        );
        assert!(
            eq_f64(north_east.lon(), east),
            "expected: {}, actual: {}",
            east,
            north_east.lon()
        );
    }

    #[test]
    fn mesh2_south_east_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let south = mesh2_south(code);
        let east = mesh2_west(code) + MESH2_LON_DIFF;
        let south_east = mesh.south_east();
        assert!(
            eq_f64(south_east.lat(), south),
            "expected: {}, actual: {}",
            south,
            south_east.lat()
        );
        assert!(
            eq_f64(south_east.lon(), east),
            "expected: {}, actual: {}",
            east,
            south_east.lon()
        );
    }

    #[test]
    fn mesh2_south_west_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let south = mesh2_south(code);
        let west = mesh2_west(code);
        let south_west = mesh.south_west();
        assert!(
            eq_f64(south_west.lat(), south),
            "expected: {}, actual: {}",
            south,
            south_west.lat()
        );
        assert!(
            eq_f64(south_west.lon(), west),
            "expected: {}, actual: {}",
            west,
            south_west.lon()
        );
    }

    #[test]
    fn mesh2_north_west_ok() {
        let code = "513546";
        let mesh = Mesh2::new(String::from(code)).unwrap();
        let north = mesh2_south(code) + MESH2_LAT_DIFF;
        let west = mesh2_west(code);
        let north_west = mesh.north_west();
        assert!(
            eq_f64(north_west.lat(), north),
            "expected: {}, actual: {}",
            north,
            north_west.lat()
        );
        assert!(
            eq_f64(north_west.lon(), west),
            "expected: {}, actual: {}",
            west,
            north_west.lon()
        );
    }

    #[test]
    #[rustfmt::skip]
    fn mesh2_north_mesh_ok() {
        let input = vec![
            ("714967", "714977"),
            ("301800", "301810"),
        ];
        for (code, expected) in input {
            let mesh = Mesh2::new(String::from(code)).unwrap();
            let north_mesh = mesh.north_mesh().unwrap();
            assert_eq!(north_mesh.code(), expected);
        }
    }

    #[test]
    fn mesh2_north_mesh_err() {
        let mesh = Mesh2::new(String::from("714977")).unwrap();
        assert!(mesh.north_mesh().is_err());
    }

    #[test]
    #[rustfmt::skip]
    fn mesh2_east_mesh_ok() {
        let input = vec![
            ("513546", "513547"),
            ("513547", "513640"),
        ];
        for (code, expected) in input {
            let mesh = Mesh2::new(String::from(code)).unwrap();
            let east_mesh = mesh.east_mesh().unwrap();
            assert_eq!(expected, east_mesh.code());
        }
    }

    #[test]
    fn mesh2_east_mesh_err() {
        let mesh = Mesh2::new(String::from("304907")).unwrap();
        assert!(mesh.east_mesh().is_err());
    }

    #[test]
    fn mesh2_south_mesh_ok() {
        let input = vec![("513546", "513536"), ("513506", "503576")];
        for (code, expected) in input {
            let mesh = Mesh2::new(String::from(code)).unwrap();
            let south_mesh = mesh.south_mesh().unwrap();
            assert_eq!(south_mesh.code(), expected);
        }
    }

    #[test]
    fn mesh2_south_mesh_err() {
        let mesh = Mesh2::new(String::from("302200")).unwrap();
        assert!(mesh.south_mesh().is_err());
    }

    #[test]
    fn mesh2_west_mesh_ok() {
        let input = vec![("513546", "513545"), ("513540", "513447")];
        for (code, expected) in input {
            let mesh = Mesh2::new(String::from(code)).unwrap();
            let west_mesh = mesh.west_mesh().unwrap();
            assert_eq!(expected, west_mesh.code());
        }
    }

    #[test]
    fn mesh2_west_mesh_err() {
        let mesh = Mesh2::new(String::from("301800")).unwrap();
        assert!(mesh.west_mesh().is_err());
    }

    #[test]
    fn mesh2_is_neighbor_ok() {
        let inputs = vec![
            ("513546", "513556", NeighborDirection::North),
            ("513546", "513547", NeighborDirection::East),
            ("513546", "513536", NeighborDirection::South),
            ("513546", "513545", NeighborDirection::West),
            ("513570", "523500", NeighborDirection::North),
            ("513507", "513600", NeighborDirection::East),
            ("513500", "503570", NeighborDirection::South),
            ("513500", "513407", NeighborDirection::West),
        ];
        for (code1, code2, expected) in inputs {
            let mesh1 = Mesh2::new(String::from(code1)).unwrap();
            let mesh2 = Mesh2::new(String::from(code2)).unwrap();
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
    fn mesh2_is_neighbor_none() {
        let inputs = vec![
            ("513546", "513557"),
            ("513546", "513537"),
            ("513546", "513535"),
            ("513546", "513555"),
            ("513546", "513566"),
            ("513546", "513640"),
            ("513546", "513526"),
            ("513546", "513544"),
        ];
        for (code1, code2) in inputs {
            let mesh1 = Mesh2::new(String::from(code1)).unwrap();
            let mesh2 = Mesh2::new(String::from(code2)).unwrap();
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
