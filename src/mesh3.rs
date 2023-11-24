use crate::mesh2::validate_mesh2_code;
use crate::{Coordinate, GSJPError, Mesh, Mesh1, Mesh2};

/// 地域基準メッシュの南端と北端の緯度の差
const MESH3_LAT_DIFF: f64 = 30.0 / 3600.0; // 30秒
/// 地域基準メッシュの西端と東端の緯度の差
const MESH3_LON_DIFF: f64 = 45.0 / 3600.0; // 45秒

/// 基準地域メッシュ（第3次地域区画）
///
/// 第2次地域区画を南北に10等分、東西に10等分した区画を示す。
/// 基準地域メッシュの辺の長さは約1kmである。
pub struct Mesh3 {
    code: String,
}

impl Mesh3 {
    /// 基準地域メッシュを含む第1次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第1次地域区画
    pub fn mesh1(&self) -> Mesh1 {
        Mesh1::new(self.code[0..4].to_string()).unwrap()
    }

    /// 基準地域メッシュを含む第2次地域区画を返す。
    ///
    /// # 戻り値
    ///
    /// 第2次地域区画
    pub fn mesh2(&self) -> Mesh2 {
        Mesh2::new(self.code[0..6].to_string()).unwrap()
    }
}

impl Mesh for Mesh3 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh3_code(&code)?;

        Ok(Self { code })
    }

    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError> {
        let mesh2 = Mesh2::from_coordinate(coord)?;
        let lat_n = ((coord.lat() - mesh2.south()) / MESH3_LAT_DIFF) as u8;
        let lon_n = ((coord.lon() - mesh2.west()) / MESH3_LON_DIFF) as u8;
        let code = format!("{}{}{}", mesh2.code(), lat_n, lon_n);

        Self::new(code)
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH3_LAT_DIFF
    }

    fn south(&self) -> f64 {
        self.mesh2().south()
            + MESH3_LAT_DIFF * self.code.chars().nth(6).unwrap().to_digit(10).unwrap() as f64
    }

    fn east(&self) -> f64 {
        self.west() + MESH3_LON_DIFF
    }

    fn west(&self) -> f64 {
        self.mesh2().west()
            + MESH3_LON_DIFF * self.code.chars().nth(7).unwrap().to_digit(10).unwrap() as f64
    }

    fn north_mesh(&self) -> Result<Self, GSJPError> {
        let lat_idx = self.code.chars().nth(6).unwrap().to_digit(10).unwrap();
        let code = if lat_idx == 9 {
            let mesh2 = self.mesh2().north_mesh()?;
            format!("{}0{}", mesh2.code(), self.code.chars().nth(7).unwrap())
        } else {
            format!(
                "{}{}{}",
                &self.code[0..6],
                lat_idx + 1,
                self.code.chars().nth(7).unwrap(),
            )
        };

        Self::new(code)
    }

    fn east_mesh(&self) -> Result<Self, GSJPError> {
        let lon_idx = self.code.chars().nth(7).unwrap().to_digit(10).unwrap();
        let code = if lon_idx == 9 {
            let mesh2 = self.mesh2().east_mesh()?;
            format!("{}{}0", mesh2.code(), self.code.chars().nth(6).unwrap())
        } else {
            format!("{}{}", &self.code[0..7], lon_idx + 1,)
        };

        Self::new(code)
    }

    fn south_mesh(&self) -> Result<Self, GSJPError> {
        let lat_idx = self.code.chars().nth(6).unwrap().to_digit(10).unwrap();
        let code = if lat_idx == 0 {
            let mesh1 = self.mesh2().south_mesh()?;
            format!("{}9{}", mesh1.code(), self.code.chars().nth(7).unwrap())
        } else {
            format!(
                "{}{}{}",
                &self.code[0..6],
                lat_idx - 1,
                self.code.chars().nth(7).unwrap(),
            )
        };

        Self::new(code)
    }

    fn west_mesh(&self) -> Result<Self, GSJPError> {
        let lon_idx = self.code.chars().nth(7).unwrap().to_digit(10).unwrap();
        let code = if lon_idx == 0 {
            let mesh2 = self.mesh2().west_mesh()?;
            format!("{}{}9", mesh2.code(), self.code.chars().nth(6).unwrap())
        } else {
            format!("{}{}", &self.code[0..7], lon_idx - 1,)
        };

        Self::new(code)
    }
}

pub(crate) fn validate_mesh3_code(code: &str) -> Result<(), GSJPError> {
    // 上位6桁が第2次地域区画のメッシュコードであることを確認
    if code.len() != 8 {
        return Err(GSJPError::InvalidMeshCode);
    }
    validate_mesh2_code(&code[0..6])?;
    // 基準地域メッシュのメッシュコードの基準地域メッシュ部分について、緯度方向の値と経度方向の値を確認
    let lat = &code.chars().nth(6).unwrap();
    if !(&'0'..=&'9').contains(&lat) {
        return Err(GSJPError::InvalidMeshCode);
    }
    let lon = &code.chars().nth(7).unwrap();
    if !(&'0'..=&'9').contains(&lon) {
        return Err(GSJPError::InvalidMeshCode);
    }

    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        mesh2::tests::{mesh2_south, mesh2_west},
        tests::eq_f64,
        NeighborDirection, EASTERNMOST, NORTHERNMOST, SOUTHERNMOST, WESTERNMOST,
    };

    use super::*;

    #[test]
    fn mesh3_new_ok() {
        assert!(Mesh3::new(String::from("68547799")).is_ok());
        assert!(Mesh3::new(String::from("30540709")).is_ok());
        assert!(Mesh3::new(String::from("30220000")).is_ok());
        assert!(Mesh3::new(String::from("68227090")).is_ok());
    }

    #[test]
    fn mesh3_new_err() {
        assert!(Mesh3::new(String::from("69540709")).is_err());
        assert!(Mesh3::new(String::from("68557090")).is_err());
        assert!(Mesh3::new(String::from("29547799")).is_err());
        assert!(Mesh3::new(String::from("30550000")).is_err());
        assert!(Mesh3::new(String::from("29227090")).is_err());
        assert!(Mesh3::new(String::from("30210709")).is_err());
        assert!(Mesh3::new(String::from("69220000")).is_err());
        assert!(Mesh3::new(String::from("68217799")).is_err());
    }

    #[test]
    fn mesh3_from_coordinate_ok() {
        // 東京タワーを含む第2次地域区画
        let mesh2 = Mesh2::new(String::from("533935")).unwrap();
        #[rustfmt::skip]
        let inputs = vec![
            // 東京タワーを含むメッシュの北東端の中心座標
            (
                Coordinate::new(
                    mesh2.north() - MESH3_LAT_DIFF / 2.0,
                    mesh2.east() - MESH3_LON_DIFF / 2.0,
                )
                .unwrap(),
                "53393599",
                "北東端の中心"
            ),
            // 東京タワーを含むメッシュの北東端
            (
                Coordinate::new(
                    mesh2.north() - 1e-8,
                    mesh2.east() - 1e-8,
                ).unwrap(),
                "53393599",
                "北東端"
            ),
            // 東京タワーを含むメッシュの南東端の中心座標
            (
                Coordinate::new(
                    mesh2.south() + MESH3_LAT_DIFF / 2.0,
                    mesh2.east() - MESH3_LON_DIFF / 2.0,
                )
                .unwrap(),
                "53393509",
                "南東端の中心"
            ),
            // 東京タワーを含むメッシュの南東端
            (
                Coordinate::new(
                    mesh2.south() + 1e-8,
                    mesh2.east() - 1e-8,
                ).unwrap(),
                "53393509",
                "南東端"
            ),
            // 東京タワーを含むメッシュの南西端の中心座標
            (
                Coordinate::new(
                    mesh2.south() + MESH3_LAT_DIFF / 2.0,
                    mesh2.west() + MESH3_LON_DIFF / 2.0,
                )
                .unwrap(),
                "53393500",
                "南西端の中心"
            ),
            // 東京タワーを含むメッシュの南西端
            (
                Coordinate::new(
                    mesh2.south() + 1e-8,
                    mesh2.west() + 1e-8,
                ).unwrap(),
                "53393500",
                "南西端"
            ),
            // 東京タワーを含むメッシュの北西端の中心座標
            (
                Coordinate::new(
                    mesh2.north() - MESH3_LAT_DIFF / 2.0,
                    mesh2.west() + MESH3_LON_DIFF / 2.0,
                )
                .unwrap(),
                "53393590",
                "北西端の中心"
            ),
            // 東京タワーを含むメッシュの北西端
            (
                Coordinate::new(
                    mesh2.north() - 1e-8,
                    mesh2.west() + 1e-8,
                ).unwrap(),
                "53393590",
                "北西端"
            ),
            // 東京タワーを含む第3次地域区画
            (
                Coordinate::new(
                    35.65858404079,
                    139.74543164468,
                ).unwrap(),
                "53393599",
                "東京タワー"
            ),
        ];
        for (coord, expected, name) in inputs {
            let mesh = Mesh3::from_coordinate(coord).unwrap();
            assert_eq!(
                expected,
                mesh.code(),
                "expected: {}, actual: {}, name: {}",
                expected,
                mesh.code(),
                name
            );
        }
    }

    #[test]
    fn mesh3_from_coordinate_err() {
        let data = vec![
            Coordinate::new(NORTHERNMOST + 1.0, WESTERNMOST).unwrap(),
            Coordinate::new(NORTHERNMOST, WESTERNMOST - 1.0).unwrap(),
            Coordinate::new(SOUTHERNMOST - 1.0, WESTERNMOST).unwrap(),
            Coordinate::new(SOUTHERNMOST, EASTERNMOST + 1.0).unwrap(),
        ];
        for coord in data {
            assert!(Mesh3::from_coordinate(coord).is_err());
        }
    }

    fn mesh3_south(code: &str) -> f64 {
        mesh2_south(code)
            + MESH3_LAT_DIFF * code.chars().nth(6).unwrap().to_digit(10).unwrap() as f64
    }

    fn mesh3_west(code: &str) -> f64 {
        mesh2_west(code)
            + MESH3_LON_DIFF * code.chars().nth(7).unwrap().to_digit(10).unwrap() as f64
    }

    #[test]
    fn mesh3_north_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let expected = mesh3_south(code) + MESH3_LAT_DIFF;
        assert!(
            eq_f64(mesh.north(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.north()
        );
    }

    #[test]
    fn mesh3_east_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let expected = mesh3_west(code) + MESH3_LON_DIFF;
        assert!(
            eq_f64(mesh.east(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.east()
        );
    }

    #[test]
    fn mesh3_south_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let expected = mesh3_south(code);
        assert!(
            eq_f64(mesh.south(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.south()
        );
    }

    #[test]
    fn mesh3_west_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let expected = mesh3_west(code);
        assert!(
            eq_f64(mesh.west(), expected),
            "expected: {}, actual: {}",
            expected,
            mesh.west()
        );
    }

    #[test]
    fn mesh3_center_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let lat_center = mesh3_south(code) + MESH3_LAT_DIFF / 2.0;
        let lon_center = mesh3_west(code) + MESH3_LON_DIFF / 2.0;
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
    fn mesh3_north_east_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let north = mesh3_south(code) + MESH3_LAT_DIFF;
        let east = mesh3_west(code) + MESH3_LON_DIFF;
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
    fn mesh3_south_east_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let south = mesh3_south(code);
        let east = mesh3_west(code) + MESH3_LON_DIFF;
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
    fn mesh3_south_west_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let south = mesh3_south(code);
        let west = mesh3_west(code);
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
    fn mesh3_north_west_ok() {
        let code = "51354637";
        let mesh = Mesh3::new(String::from(code)).unwrap();
        let north = mesh3_south(code) + MESH3_LAT_DIFF;
        let west = mesh3_west(code);
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
    fn mesh3_north_mesh_ok() {
        let input = vec![
            ("51354635", "51354645"),
            ("51354695", "51355605"),
        ];
        for (code, expected) in input {
            let mesh = Mesh3::new(String::from(code)).unwrap();
            let north_mesh = mesh.north_mesh().unwrap();
            assert_eq!(north_mesh.code(), expected);
        }
    }

    #[test]
    fn mesh3_north_mesh_err() {
        let mesh = Mesh3::new(String::from("68227090")).unwrap();
        assert!(mesh.north_mesh().is_err());
    }

    #[test]
    #[rustfmt::skip]
    fn mesh3_east_mesh_ok() {
        let input = vec![
            ("51354635", "51354636"),
            ("51354639", "51354730"),
        ];
        for (code, expected) in input {
            let mesh = Mesh3::new(String::from(code)).unwrap();
            let east_mesh = mesh.east_mesh().unwrap();
            assert_eq!(expected, east_mesh.code());
        }
    }

    #[test]
    fn mesh3_east_mesh_err() {
        let mesh = Mesh3::new(String::from("30540709")).unwrap();
        assert!(mesh.east_mesh().is_err());
    }

    #[test]
    #[rustfmt::skip]
    fn mesh3_south_mesh_ok() {
        let input = vec![
            ("51354635", "51354625"),
            ("51350600", "50357690"),
        ];
        for (code, expected) in input {
            let mesh = Mesh3::new(String::from(code)).unwrap();
            let south_mesh = mesh.south_mesh().unwrap();
            assert_eq!(south_mesh.code(), expected);
        }
    }

    #[test]
    fn mesh3_south_mesh_err() {
        let mesh = Mesh3::new(String::from("30220000")).unwrap();
        assert!(mesh.south_mesh().is_err());
    }

    #[test]
    #[rustfmt::skip]
    fn mesh3_west_mesh_ok() {
        let input = vec![
            ("51354635", "51354634"),
            ("51354030", "51344739"),
        ];
        for (code, expected) in input {
            let mesh = Mesh3::new(String::from(code)).unwrap();
            let west_mesh = mesh.west_mesh().unwrap();
            assert_eq!(expected, west_mesh.code());
        }
    }

    #[test]
    fn mesh3_west_mesh_err() {
        let mesh = Mesh3::new(String::from("30220000")).unwrap();
        assert!(mesh.west_mesh().is_err());
    }

    #[test]
    fn mesh3_is_neighbor_ok() {
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
    fn mesh3_is_neighbor_none() {
        let inputs = vec![
            ("51354635", "51354646"), // 北東隣
            ("51354635", "51354626"), // 南東隣
            ("51354635", "51354624"), // 南西隣
            ("51354635", "51354644"), // 北西隣
            ("51354635", "51354655"), // 2つ北隣
            ("51354635", "51354637"), // 2つ東隣
            ("51354635", "51354615"), // 2つ南隣
            ("51354635", "51354633"), // 2つ西隣
        ];
        for (code1, code2) in inputs {
            let mesh1 = Mesh3::new(String::from(code1)).unwrap();
            let mesh2 = Mesh3::new(String::from(code2)).unwrap();
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
