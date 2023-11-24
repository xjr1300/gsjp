use crate::{GSJPError, Mesh, EASTERNMOST, NORTHERNMOST, SOUTHERNMOST, WESTERNMOST};

/// 第1次地域区画の南端と北端の緯度の差
const MESH1_LAT_DIFF: f64 = 40.0 / 60.0; // 40分
/// 第1次地域区画の西端と東端の緯度の差
const MESH1_LON_DIFF: f64 = 1.0; // 1度

/// 第1次地域区画
///
/// 緯度は20度から46度まで、経度は122度から155度までの範囲を表現する。
/// メッシュコードは、区画の南西端の緯度と経度で決まる。
///
/// よって、第1次地域区画のメッシュコードは次の通り。
///
/// * 北東端の第1次地域区画のメッシュコードは`6854`
/// * 南東端の第1次地域区画のメッシュコードは`3054`
/// * 南西端の第1次地域区画のメッシュコードは`3022`
/// * 北西端の第1次地域区画のメッシュコードは`6822`
///
/// 南西端の緯度が36度の場合、36 * 1.5 = 54となる。
/// 南西端の経度が138度の場合、138 - 100 = 38となる。
/// この第1次地域区画のメッシュコードは、5438となる。
pub struct Mesh1 {
    /// メッシュコード
    code: String,
}

impl Mesh for Mesh1 {
    fn new(code: String) -> Result<Self, GSJPError> {
        validate_mesh1_code(&code)?;

        Ok(Mesh1 { code })
    }

    fn from_coordinate(coord: crate::Coordinate) -> Self {
        let lat = (coord.lat() * 1.5) as u8;
        let lon = (coord.lon() - 100.0) as u8;
        let code = format!("{:02}{:02}", lat, lon);

        Mesh1 { code }
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn north(&self) -> f64 {
        self.south() + MESH1_LAT_DIFF
    }

    fn south(&self) -> f64 {
        self.code[0..2].parse::<f64>().unwrap() / 1.5
    }

    fn east(&self) -> f64 {
        self.west() + MESH1_LON_DIFF
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
fn validate_mesh1_code(code: &str) -> Result<(), GSJPError> {
    // メッシュコードを緯度部分と経度部分に分割
    if code.len() != 4 {
        return Err(GSJPError::InvalidMeshCode);
    }
    let lat = &code[0..2];
    let lon = &code[2..4];

    // 緯度の範囲を20度から46度までとして、緯度部分を検証
    let lat_min = ((SOUTHERNMOST * 1.5) as u8).to_string();
    let lat_max = (((NORTHERNMOST - MESH1_LAT_DIFF) * 1.5) as u8).to_string();
    if lat < &lat_min || lat > &lat_max {
        return Err(GSJPError::InvalidMeshCode);
    }

    // 経度の範囲を122度から154度までとして、経度部分を検証
    let lon_min = (WESTERNMOST as u8 % 100).to_string();
    let lon_max = ((EASTERNMOST - MESH1_LON_DIFF) as u8 % 100).to_string();
    if lon < &lon_min || lon > &lon_max {
        return Err(GSJPError::InvalidMeshCode);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::eq_f64;
    use crate::{
        Coordinate, NeighborDirection, EASTERNMOST, NORTHERNMOST, SOUTHERNMOST, WESTERNMOST,
    };

    use super::*;

    #[test]
    fn validate_mesh1_code_ok() {
        assert!(validate_mesh1_code("3022").is_ok());
        assert!(validate_mesh1_code("6854").is_ok());
    }

    #[test]
    fn validate_mesh1_code_err() {
        assert!(validate_mesh1_code("2922").is_err());
        assert!(validate_mesh1_code("3021").is_err());
        assert!(validate_mesh1_code("6954").is_err());
        assert!(validate_mesh1_code("6855").is_err());
    }

    #[test]
    fn mesh1_new_ok() {
        assert!(Mesh1::new(String::from("3022")).is_ok());
        assert!(Mesh1::new(String::from("6854")).is_ok());
    }

    #[test]
    fn mesh1_new_err() {
        assert!(Mesh1::new(String::from("2922")).is_err());
        assert!(Mesh1::new(String::from("3021")).is_err());
        assert!(Mesh1::new(String::from("6954")).is_err());
        assert!(Mesh1::new(String::from("6855")).is_err());
    }

    #[test]
    fn mesh1_from_coordinate_ok() {
        let nw = Coordinate::new(NORTHERNMOST, WESTERNMOST).unwrap();
        let ne = Coordinate::new(NORTHERNMOST, EASTERNMOST).unwrap();
        let sw = Coordinate::new(SOUTHERNMOST, WESTERNMOST).unwrap();
        let se = Coordinate::new(SOUTHERNMOST, EASTERNMOST).unwrap();

        let n = (NORTHERNMOST * 1.5) as u8;
        let e = (EASTERNMOST - 100.0) as u8;
        let s = (SOUTHERNMOST * 1.5) as u8;
        let w = (WESTERNMOST - 100.0) as u8;

        let data = vec![
            (nw, format!("{:02}{:02}", n, w)),
            (ne, format!("{:02}{:02}", n, e)),
            (sw, format!("{:02}{:02}", s, w)),
            (se, format!("{:02}{:02}", s, e)),
        ];
        for (coord, code) in data {
            let mesh = Mesh1::from_coordinate(coord.to_owned());
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
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let n_mesh = mesh.north_mesh().unwrap();
        assert_eq!("3122", n_mesh.code());
    }

    #[test]
    fn mesh1_north_mesh_err() {
        let mesh = Mesh1::new(String::from("6822")).unwrap();
        assert!(mesh.north_mesh().is_err());
    }

    #[test]
    fn mesh1_east_mesh_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let e_mesh = mesh.east_mesh().unwrap();
        assert_eq!("3023", e_mesh.code());
    }

    #[test]
    fn mesh1_east_mesh_err() {
        let mesh = Mesh1::new(String::from("3054")).unwrap();
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
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        assert!(mesh.south_mesh().is_err());
    }

    #[test]
    fn mesh1_west_mesh_ok() {
        let mesh = Mesh1::new(String::from("3023")).unwrap();
        let w_mesh = mesh.west_mesh().unwrap();
        assert_eq!("3022", w_mesh.code());
    }

    #[test]
    fn mesh1_west_mesh_err() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        assert!(mesh.west_mesh().is_err());
    }

    #[test]
    fn mesh1_north_east_mesh_ok() {
        let mesh = Mesh1::new(String::from("3022")).unwrap();
        let ne_mesh = mesh.north_east_mesh().unwrap();
        assert_eq!("3123", ne_mesh.code());
    }

    #[test]
    fn mesh1_south_east_mesh_ok() {
        let mesh = Mesh1::new(String::from("3122")).unwrap();
        let se_mesh = mesh.south_east_mesh().unwrap();
        assert_eq!("3023", se_mesh.code());
    }

    #[test]
    fn mesh1_south_west_mesh_ok() {
        let mesh = Mesh1::new(String::from("3123")).unwrap();
        let sw_mesh = mesh.south_west_mesh().unwrap();
        assert_eq!("3022", sw_mesh.code());
    }

    #[test]
    fn mesh1_north_west_mesh_ok() {
        let mesh = Mesh1::new(String::from("3023")).unwrap();
        let nw_mesh = mesh.north_west_mesh().unwrap();
        assert_eq!("3122", nw_mesh.code());
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
