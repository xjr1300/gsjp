use crate::{
    contains_coordinate, Coordinate, GSJPError, Mesh, EASTERNMOST, NORTHERNMOST, SOUTHERNMOST,
    WESTERNMOST,
};

/// 第1次地域区画の南端と北端の緯度の差
const MESH1_LAT_DIFF: f64 = 40.0 / 60.0; // 40分
/// 第1次地域区画の西端と東端の緯度の差
const MESH1_LON_DIFF: f64 = 1.0; // 1度

/// 第1次地域区画
///
/// 第1次地域区画の辺の長さは約80kmである。
///
/// 緯度は20度から48度まで、経度は118度から150度までの範囲を表現する。
///
/// 第1次地域区画のメッシュコードは、区画の南西端の緯度と経度で次の通り決まる。
///
/// メッシュコードの緯度部分 = 整数部分(緯度 * 1.5)
/// メッシュコードの経度部分 = 整数部分(経度 - 100.0)
///
/// 南西端の緯度が36度の場合、36.0 * 1.5 = 54.0となる。
/// 南西端の経度が138度の場合、138.0 - 100.0 = 38.0となる。
/// この座標が位置する第1次地域区画のメッシュコードは、5438となる。
///
/// 最北端の第1次地域区画のメッシュコードの緯度部分は次の通り。
/// (48.0 - 40.0 / 60.0) * 1.5 = 71.0 ∴ 71
/// 最東端の第1次地域区画のメッシュコードの経度部分は次の通り。
/// 150.0 - 1.0 - 100.0 = 49.0 ∴ 49
/// 最南端の第1次地域区画のメッシュコードの緯度部分は次の通り。
/// 20.0 * 1.5 = 30.0 ∴ 30
/// 最西端の第1次地域区画のメッシュコードの経度部分は次の通り。
/// 118.0 - 100.0 = 18.0 ∴ 18
///
/// 第1次地域区画のメッシュコードは、次の通り。
///
/// * 北東端の第1次地域区画のメッシュコードは`7149`
/// * 南東端の第1次地域区画のメッシュコードは`3049`
/// * 南西端の第1次地域区画のメッシュコードは`3018`
/// * 北西端の第1次地域区画のメッシュコードは`7118`
pub struct Mesh1 {
    /// メッシュコード
    code: String,
}

impl Mesh for Mesh1 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh1_code(&code)?;

        Ok(Mesh1 { code })
    }

    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError> {
        contains_coordinate(&coord)?;

        let lat = (coord.lat() * 1.5) as u8;
        let lon = (coord.lon() - 100.0) as u8;
        let code = format!("{:02}{:02}", lat, lon);

        Ok(Mesh1 { code })
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH1_LAT_DIFF
    }

    fn east(&self) -> f64 {
        self.west() + MESH1_LON_DIFF
    }

    fn south(&self) -> f64 {
        self.code[0..2].parse::<f64>().unwrap() / 1.5
    }

    fn west(&self) -> f64 {
        self.code[2..4].parse::<f64>().unwrap() + 100.0
    }

    fn north_mesh(&self) -> Result<Self, GSJPError> {
        let lat = self.code[0..2].parse::<u8>().unwrap() + 1;
        let code = format!("{:02}{}", lat, &self.code[2..4]);

        Mesh1::new(code)
    }

    fn east_mesh(&self) -> Result<Self, GSJPError> {
        let lon = self.code[2..4].parse::<u8>().unwrap() + 1;
        let code = format!("{}{:02}", &self.code[0..2], lon);

        Mesh1::new(code)
    }

    fn south_mesh(&self) -> Result<Self, GSJPError> {
        let lat = self.code[0..2].parse::<u8>().unwrap() - 1;
        let code = format!("{:02}{}", lat, &self.code[2..4]);

        Mesh1::new(code)
    }

    fn west_mesh(&self) -> Result<Self, GSJPError> {
        let lon = self.code[2..4].parse::<u8>().unwrap() - 1;
        let code = format!("{}{:02}", &self.code[0..2], lon);

        Mesh1::new(code)
    }
}

/// 第1次地域区画のメッシュコードを検証する。
///
/// 緯度の範囲を20度から46度までとする。
/// 経度の範囲を122度から154度までとする。
///
/// # 引数
///
/// * `code` - 第1次地域区画のメッシュコード
///
/// # 戻り値
///
/// `()`
pub(crate) fn validate_mesh1_code(code: &str) -> Result<(), GSJPError> {
    // メッシュコードを緯度部分と経度部分に分割
    if code.len() != 4 {
        return Err(GSJPError::InvalidMeshCode);
    }
    let lat = &code[0..2];
    let lon = &code[2..4];

    // 緯度部分を検証
    let lat_min = ((SOUTHERNMOST * 1.5) as u8).to_string();
    let lat_max = (((NORTHERNMOST - MESH1_LAT_DIFF) * 1.5) as u8).to_string();
    if lat < &lat_min || lat > &lat_max {
        return Err(GSJPError::InvalidMeshCode);
    }

    // 経度部分を検証
    let lon_min = (WESTERNMOST as u8 % 100).to_string();
    let lon_max = ((EASTERNMOST - MESH1_LON_DIFF) as u8 % 100).to_string();
    if lon < &lon_min || lon > &lon_max {
        return Err(GSJPError::InvalidMeshCode);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::eq_f64;
    use crate::{
        Coordinate, NeighborDirection, EASTERNMOST, NORTHERNMOST, SOUTHERNMOST, WESTERNMOST,
    };

    #[test]
    fn mesh1_new_ok() {
        // 第1次地域区画の最北東端メッシュコードは、次の通り。
        // (48.0 - 40.0 / 60.0) * 1.5 = 71.0
        // 150.0 - 1.0 - 100.0 = 49.0
        // よって、メッシュコードは、7149となる。
        //
        // 第1次地域区画の最南東端メッシュコードは、次の通り。
        // 20.0 * 1.5 = 30.0
        // 150.0 - 1.0 - 100.0 = 49.0
        // よって、メッシュコードは、3049となる。
        //
        // 第1次地域区画の最南西端メッシュコードは、次の通り。
        // 20.0 * 1.5 = 30.0
        // 118.0 - 100.0 = 18.0
        // よって、メッシュコードは、3018となる。
        assert!(Mesh1::new(String::from("7149")).is_ok()); // 最北東端
        assert!(Mesh1::new(String::from("3049")).is_ok()); // 最南東端
        assert!(Mesh1::new(String::from("3018")).is_ok()); // 最南西端
        assert!(Mesh1::new(String::from("7118")).is_ok()); // 北西端
    }

    #[test]
    fn mesh1_new_err() {
        assert!(Mesh1::new(String::from("7249")).is_err()); // 最北東端の1つ北側
        assert!(Mesh1::new(String::from("7150")).is_err()); // 最北東端の1つ東側
        assert!(Mesh1::new(String::from("2949")).is_err()); // 最南東端の1つ南側
        assert!(Mesh1::new(String::from("3050")).is_err()); // 最南東端の1つ東側
        assert!(Mesh1::new(String::from("2918")).is_err()); // 最南西端の1つ南側
        assert!(Mesh1::new(String::from("3017")).is_err()); // 最南西端の1つ西側
        assert!(Mesh1::new(String::from("7218")).is_err()); // 最北西端の1つ北側
        assert!(Mesh1::new(String::from("7217")).is_err()); // 最北西端の1つ西側
    }

    #[test]
    fn mesh1_from_coordinate_ok() {
        let inputs = vec![
            // 最北東端のメッシュの中心座標
            (
                Coordinate::new(
                    NORTHERNMOST - MESH1_LAT_DIFF / 2.0,
                    EASTERNMOST - MESH1_LON_DIFF / 2.0,
                )
                .unwrap(),
                "7149",
            ),
            // 最南東端のメッシュの中心座標
            (
                Coordinate::new(
                    SOUTHERNMOST + MESH1_LAT_DIFF / 2.0,
                    EASTERNMOST - MESH1_LON_DIFF / 2.0,
                )
                .unwrap(),
                "3049",
            ),
            // 最南西端のメッシュの中心座標
            (
                Coordinate::new(
                    SOUTHERNMOST + MESH1_LAT_DIFF / 2.0,
                    WESTERNMOST + MESH1_LON_DIFF / 2.0,
                )
                .unwrap(),
                "3018",
            ),
            // 最北西端のメッシュの中心座標
            (
                Coordinate::new(
                    NORTHERNMOST - MESH1_LAT_DIFF / 2.0,
                    WESTERNMOST + MESH1_LON_DIFF / 2.0,
                )
                .unwrap(),
                "7118",
            ),
            // 東京付近のメッシュの中心座標
            (
                Coordinate::new(
                    35.0 + 20.0 / 60.0 + MESH1_LAT_DIFF / 2.0,
                    139.0 + MESH1_LON_DIFF / 2.0,
                )
                .unwrap(),
                "5339",
            ),
        ];
        for (coord, code) in inputs {
            let mesh = Mesh1::from_coordinate(coord.to_owned()).unwrap();
            assert_eq!(
                code,
                mesh.code(),
                "expected: {}, actual: {}",
                code,
                mesh.code()
            );
        }
    }

    #[test]
    fn mesh1_from_coordinate_err() {
        let data = vec![
            Coordinate::new(NORTHERNMOST + 1.0, WESTERNMOST).unwrap(),
            Coordinate::new(NORTHERNMOST, WESTERNMOST - 1.0).unwrap(),
            Coordinate::new(SOUTHERNMOST - 1.0, WESTERNMOST).unwrap(),
            Coordinate::new(SOUTHERNMOST, EASTERNMOST + 1.0).unwrap(),
        ];
        for coord in data {
            assert!(Mesh1::from_coordinate(coord).is_err());
        }
    }

    #[test]
    fn mesh1_north_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = 30.0 / 1.5 + MESH1_LAT_DIFF;
        assert!(
            eq_f64(expected, mesh.north()),
            "expected: {}, actual: {}",
            expected,
            mesh.north()
        );
    }

    #[test]
    fn mesh1_east_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = 122.0 + MESH1_LON_DIFF;
        assert!(
            eq_f64(expected, mesh.east()),
            "expected: {}, actual: {}",
            expected,
            mesh.north()
        );
    }

    #[test]
    fn mesh1_south_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = 30.0 / 1.5;
        assert!(
            eq_f64(expected, mesh.south()),
            "expected: {}, actual: {}",
            expected,
            mesh.north()
        );
    }

    #[test]
    fn mesh1_west_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = 122.0;
        assert!(
            eq_f64(expected, mesh.west()),
            "expected: {}, actual: {}",
            expected,
            mesh.north()
        );
    }

    #[test]
    fn mesh1_center_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expect = Coordinate::new(
            30.0 / 1.5 + MESH1_LAT_DIFF / 2.0,
            122.0 + MESH1_LON_DIFF / 2.0,
        )
        .unwrap();
        assert!(
            eq_f64(expect.lat(), mesh.center().lat()),
            "expected: {}, actual: {}",
            expect.lat(),
            mesh.center().lat()
        );
        assert!(
            eq_f64(expect.lon(), mesh.center().lon()),
            "expected: {}, actual: {}",
            expect.lon(),
            mesh.center().lon()
        );
    }

    #[test]
    fn mesh1_north_east_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected =
            Coordinate::new(30.0 / 1.5 + MESH1_LAT_DIFF, 122.0 + MESH1_LON_DIFF).unwrap();
        let ne = mesh.north_east();
        assert!(
            eq_f64(expected.lat(), ne.lat()),
            "expected: {}, actual: {}",
            expected.lat(),
            ne.lat(),
        );
        assert!(
            eq_f64(expected.lon(), ne.lon()),
            "expected: {}, actual: {}",
            expected.lon(),
            ne.lon(),
        );
    }

    #[test]
    fn mesh1_south_east_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = Coordinate::new(30.0 / 1.5, 122.0 + MESH1_LON_DIFF).unwrap();
        let se = mesh.south_east();
        assert!(
            eq_f64(expected.lat(), se.lat()),
            "expected: {}, actual: {}",
            expected.lat(),
            se.lat(),
        );
        assert!(
            eq_f64(expected.lon(), se.lon()),
            "expected: {}, actual: {}",
            expected.lon(),
            se.lon(),
        );
    }

    #[test]
    fn mesh1_south_west_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = Coordinate::new(30.0 / 1.5, 122.0).unwrap();
        let sw = mesh.south_west();
        assert!(
            eq_f64(expected.lat(), sw.lat()),
            "expected: {}, actual: {}",
            expected.lat(),
            sw.lat(),
        );
        assert!(
            eq_f64(expected.lon(), sw.lon()),
            "expected: {}, actual: {}",
            expected.lon(),
            sw.lon(),
        );
    }

    #[test]
    fn mesh1_north_west_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let expected = Coordinate::new(30.0 / 1.5 + MESH1_LAT_DIFF, 122.0).unwrap();
        let nw = mesh.north_west();
        assert!(
            eq_f64(expected.lat(), nw.lat()),
            "expected: {}, actual: {}",
            expected.lat(),
            nw.lat(),
        );
        assert!(
            eq_f64(expected.lon(), nw.lon()),
            "expected: {}, actual: {}",
            expected.lon(),
            nw.lon(),
        );
    }

    #[test]
    fn mesh1_north_mesh_ok() {
        let mesh = Mesh1::new(String::from("7022")).unwrap();
        let n_mesh = mesh.north_mesh().unwrap();
        assert_eq!("7122", n_mesh.code());
    }

    #[test]
    fn mesh1_north_mesh_err() {
        let mesh = Mesh1::new(String::from("7118")).unwrap();
        assert!(mesh.north_mesh().is_err());
    }

    #[test]
    fn mesh1_east_mesh_ok() {
        let mesh = Mesh1::new(String::from("3048")).unwrap();
        let e_mesh = mesh.east_mesh().unwrap();
        assert_eq!("3049", e_mesh.code());
    }

    #[test]
    fn mesh1_east_mesh_err() {
        let mesh = Mesh1::new(String::from("3049")).unwrap();
        assert!(mesh.east_mesh().is_err());
    }

    #[test]
    fn mesh1_south_mesh_ok() {
        let mesh = Mesh1::new(String::from("3122")).unwrap();
        let s_mesh = mesh.south_mesh().unwrap();
        assert_eq!("3022", s_mesh.code());
    }

    #[test]
    fn mesh1_south_mesh_err() {
        let mesh = Mesh1::new(String::from("3049")).unwrap();
        assert!(mesh.south_mesh().is_err());
    }

    #[test]
    fn mesh1_west_mesh_ok() {
        let mesh = Mesh1::new(String::from("3019")).unwrap();
        let w_mesh = mesh.west_mesh().unwrap();
        assert_eq!("3018", w_mesh.code());
    }

    #[test]
    fn mesh1_west_mesh_err() {
        let mesh = Mesh1::new(String::from("3018")).unwrap();
        assert!(mesh.west_mesh().is_err());
    }

    #[test]
    fn mesh1_is_neighbor_ok() {
        let mesh = Mesh1::new(String::from("3123")).unwrap();
        let n_mesh = Mesh1::new(String::from("3223")).unwrap();
        let e_mesh = Mesh1::new(String::from("3124")).unwrap();
        let s_mesh = Mesh1::new(String::from("3023")).unwrap();
        let w_mesh = Mesh1::new(String::from("3122")).unwrap();

        assert_eq!(
            mesh.is_neighboring(&n_mesh).unwrap(),
            NeighborDirection::North
        );
        assert_eq!(
            mesh.is_neighboring(&e_mesh).unwrap(),
            NeighborDirection::East
        );
        assert_eq!(
            mesh.is_neighboring(&s_mesh).unwrap(),
            NeighborDirection::South,
        );
        assert_eq!(
            mesh.is_neighboring(&w_mesh).unwrap(),
            NeighborDirection::West
        );
    }

    #[test]
    fn mesh1_is_neighbor_none1() {
        let mesh = Mesh1::new(String::from("3123")).unwrap();
        let ne_mesh = Mesh1::new(String::from("3224")).unwrap();
        let se_mesh = Mesh1::new(String::from("3024")).unwrap();
        let sw_mesh = Mesh1::new(String::from("3022")).unwrap();
        let nw_mesh = Mesh1::new(String::from("3222")).unwrap();

        assert_eq!(
            mesh.is_neighboring(&ne_mesh).unwrap(),
            NeighborDirection::None
        );
        assert_eq!(
            mesh.is_neighboring(&se_mesh).unwrap(),
            NeighborDirection::None
        );
        assert_eq!(
            mesh.is_neighboring(&sw_mesh).unwrap(),
            NeighborDirection::None,
        );
        assert_eq!(
            mesh.is_neighboring(&nw_mesh).unwrap(),
            NeighborDirection::None
        );
    }

    #[test]
    fn mesh1_is_neighbor_none2() {
        let mesh = Mesh1::new(String::from("3224")).unwrap();
        let n_mesh = Mesh1::new(String::from("3424")).unwrap();
        let e_mesh = Mesh1::new(String::from("3226")).unwrap();
        let s_mesh = Mesh1::new(String::from("3024")).unwrap();
        let w_mesh = Mesh1::new(String::from("3222")).unwrap();

        assert_eq!(
            mesh.is_neighboring(&n_mesh).unwrap(),
            NeighborDirection::None
        );
        assert_eq!(
            mesh.is_neighboring(&e_mesh).unwrap(),
            NeighborDirection::None
        );
        assert_eq!(
            mesh.is_neighboring(&s_mesh).unwrap(),
            NeighborDirection::None,
        );
        assert_eq!(
            mesh.is_neighboring(&w_mesh).unwrap(),
            NeighborDirection::None
        );
    }
}
