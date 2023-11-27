use std::borrow::Cow;

mod mesh1;
mod mesh2;
mod mesh3;
mod mesh4;

pub use mesh1::Mesh1;
pub use mesh2::Mesh2;
pub use mesh3::Mesh3;
pub use mesh4::Mesh4;

/// <https://www.gsi.go.jp/KOKUJYOHO/center.htm>
/// | 区分 | 場所 | 経度 | 緯度 |
/// | --- | --- | --- | --- |
/// | 最東端 | 東京都 南鳥島 | 153°59′12″ | 24°16′59″ |
/// | 最西端 | 沖縄県 与那国島 | 122°55′57″ | 24°27′05″ |
/// | 最南端 | 東京都 沖ノ鳥島| 136°04′11″ | 20°25′31″ |
/// | 最北端 | 北海道 択捉島 | 148°45′08″ | 45°33′26″ |
///
/// メッシュの北端の緯度（度単位）
pub const NORTHERNMOST: f64 = 46.0;
/// メッシュの南端の緯度（度単位）
pub const SOUTHERNMOST: f64 = 20.0;
/// メッシュの東端の経度（度単位）
pub const EASTERNMOST: f64 = 155.0;
/// メッシュの西端の経度（度単位）
pub const WESTERNMOST: f64 = 122.0;

/// メッシュトレイト
pub trait Mesh: Sized {
    /// メッシュを作成する。
    ///
    /// # 引数
    ///
    /// * `code` - メッシュコード
    ///
    /// # 戻り値
    ///
    /// メッシュ
    fn new(code: String) -> Result<Self, GSJPError>;

    /// 指定された座標を含むメッシュを作成する。
    ///
    /// # 引数
    ///
    /// * `coord` - 座標
    ///
    /// # 戻り値
    ///
    /// メッシュ
    fn from_coordinate(coord: Coordinate) -> Result<Self, GSJPError>;

    /// メッシュコードを返す。
    ///
    /// # 戻り値
    ///
    /// メッシュコード
    fn code(&self) -> &str;

    /// メッシュの北端を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの北端の緯度
    fn north(&self) -> f64;

    /// メッシュの東端を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの東端の経度
    fn east(&self) -> f64;

    /// メッシュの南端を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの南端の緯度
    fn south(&self) -> f64;

    /// メッシュの西端を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの西端の経度
    fn west(&self) -> f64;

    /// メッシュの中心の座標を返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの中心の座標
    fn center(&self) -> Coordinate {
        let lat = (self.north() + self.south()) / 2.0;
        let lon = (self.east() + self.west()) / 2.0;

        Coordinate::new(lat, lon).unwrap()
    }

    /// メッシュの北東端の座標を返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの北東端の座標
    fn north_east(&self) -> Coordinate {
        Coordinate::new(self.north(), self.east()).unwrap()
    }

    /// メッシュの南東端の座標を返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの南東端の座標
    fn south_east(&self) -> Coordinate {
        Coordinate::new(self.south(), self.east()).unwrap()
    }

    /// メッシュの南西端の座標を返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの南西端の座標
    fn south_west(&self) -> Coordinate {
        Coordinate::new(self.south(), self.west()).unwrap()
    }

    /// メッシュの北西端の座標を返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの北西端の座標
    fn north_west(&self) -> Coordinate {
        Coordinate::new(self.north(), self.west()).unwrap()
    }

    /// 北隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 北隣のメッシュ
    fn north_mesh(&self) -> Result<Self, GSJPError>;

    /// 東隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 東隣のメッシュ
    fn east_mesh(&self) -> Result<Self, GSJPError>;

    /// 南隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 南隣のメッシュ
    fn south_mesh(&self) -> Result<Self, GSJPError>;

    /// 西隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 西隣のメッシュ
    fn west_mesh(&self) -> Result<Self, GSJPError>;

    /// 北東隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 北東隣のメッシュ
    fn north_east_mesh(&self) -> Result<Self, GSJPError> {
        self.north_mesh()?.east_mesh()
    }

    /// 南東隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 南東隣のメッシュ
    fn south_east_mesh(&self) -> Result<Self, GSJPError> {
        self.south_mesh()?.east_mesh()
    }

    /// 南西隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 南西隣のメッシュ
    fn south_west_mesh(&self) -> Result<Self, GSJPError> {
        self.south_mesh()?.west_mesh()
    }

    /// 北西隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 北西隣のメッシュ
    fn north_west_mesh(&self) -> Result<Self, GSJPError> {
        self.north_mesh()?.west_mesh()
    }

    /// メッシュが隣り合っているか確認する。
    ///
    /// 北東、南東、南西及び北西隣のメッシュは隣り合っていないと判定する。
    ///
    /// # 引数
    ///
    /// * `mesh` - 隣り合っているか確認するメッシュ。
    ///
    /// # 戻り値
    ///
    /// メッシュが隣り合っているかを示す`NeighborDirection`列挙型。
    fn is_neighboring(&self, mesh: &Self) -> Result<NeighborDirection, GSJPError> {
        if self.north_mesh()?.code() == mesh.code() {
            return Ok(NeighborDirection::North);
        } else if self.east_mesh()?.code() == mesh.code() {
            return Ok(NeighborDirection::East);
        } else if self.south_mesh()?.code() == mesh.code() {
            return Ok(NeighborDirection::South);
        } else if self.west_mesh()?.code() == mesh.code() {
            return Ok(NeighborDirection::West);
        }

        Ok(NeighborDirection::None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 隣にあるメッシュの方向
pub enum NeighborDirection {
    /// 隣り合っていない
    None,
    /// 北側に隣り合っている
    North,
    /// 東側に隣り合っている
    East,
    /// 南側に隣り合っている
    South,
    /// 西側に隣り合っている
    West,
}

/// 座標
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinate {
    /// 緯度（度単位）
    lat: f64,
    /// 経度（度単位）
    lon: f64,
}

impl Coordinate {
    /// 緯度と経度から座標を作成する。
    ///
    /// # 引数
    ///
    /// * `lat` - 緯度（度単位）
    /// * `lon` - 経度（度単位）
    ///
    /// # 戻り値
    ///
    /// 座標
    pub fn new(lat: f64, lon: f64) -> Result<Self, GSJPError> {
        let lat = validate_lat(lat)?;
        let lon = validate_lon(lon)?;

        Ok(Self { lat, lon })
    }

    /// 座標の緯度を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// 座標の緯度
    pub fn lat(self) -> f64 {
        self.lat
    }

    /// 座標の経度を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// 座標の経度
    pub fn lon(self) -> f64 {
        self.lon
    }
}

/// GSJPエラー
#[derive(thiserror::Error, Debug)]
pub enum GSJPError {
    /// 座標が範囲外
    #[error("{0}")]
    OutOfRange(Cow<'static, str>),
    /// メッシュコードが不正
    #[error("メッシュコードが不正です。")]
    InvalidMeshCode,
}

/// 緯度を検証する。
///
/// # 引数
///
/// * `lat` - 緯度（度単位）
///
/// # 戻り値
///
/// 緯度（度単位）
fn validate_lat(lat: f64) -> Result<f64, GSJPError> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(GSJPError::OutOfRange("緯度が範囲外です。".into()));
    }

    Ok(lat)
}

/// 経度を検証する。
///
/// # 引数
///
/// * `lon` - 経度（度単位）
///
/// # 戻り値
///
/// 経度（度単位）
fn validate_lon(lon: f64) -> Result<f64, GSJPError> {
    if !(-180.0..=180.0).contains(&lon) {
        return Err(GSJPError::OutOfRange("経度が範囲外です。".into()));
    }

    Ok(lon)
}

/// 標準地域メッシュが表現する範囲内に座標が含まれるか確認する。
///
/// # 引数
///
/// * `coord` - 座標
///
/// # 戻り値
///
/// `()`
pub(crate) fn contains_coordinate(coord: &Coordinate) -> Result<(), GSJPError> {
    if coord.lat() < SOUTHERNMOST || coord.lat() >= NORTHERNMOST + 1.0 {
        return Err(GSJPError::OutOfRange("緯度が範囲外です。".into()));
    }
    if coord.lon() < WESTERNMOST || coord.lon() >= EASTERNMOST + 1.0 {
        return Err(GSJPError::OutOfRange("経度が範囲外です。".into()));
    }

    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    // use std::f64::EPSILON;
    pub(crate) const EPSILON: f64 = 1e-8;

    pub(crate) fn eq_f64(expected: f64, actual: f64) -> bool {
        (expected - actual).abs() < EPSILON
    }

    #[test]
    fn test_validate_lat() {
        assert!(validate_lat(90.0).is_ok());
        assert!(validate_lat(-90.0).is_ok());
    }

    #[test]
    fn validate_lat_err() {
        assert!(validate_lat(90.0 + EPSILON).is_err());
        assert!(validate_lat(-90.0 - EPSILON).is_err());
    }

    #[test]
    fn validate_lon_ok() {
        assert!(validate_lon(-180.0).is_ok());
        assert!(validate_lon(180.0).is_ok());
    }

    #[test]
    fn validate_lon_err() {
        assert!(validate_lon(-180.0 - EPSILON).is_err());
        assert!(validate_lon(180.0 + EPSILON).is_err());
    }

    #[test]
    fn coordinate_new_ok() {
        assert!(Coordinate::new(90.0, -180.0).is_ok());
        assert!(Coordinate::new(90.0, 180.0).is_ok());
        assert!(Coordinate::new(-90.0, -180.0).is_ok());
        assert!(Coordinate::new(-90.0, 180.0).is_ok());
    }

    #[test]
    fn coordinate_new_err() {
        assert!(Coordinate::new(90.0 + EPSILON, -180.0).is_err());
        assert!(Coordinate::new(90.0, -180.0 - EPSILON).is_err());
        assert!(Coordinate::new(-90.0 - EPSILON, 180.0).is_err());
        assert!(Coordinate::new(-90.0, 180.0 + EPSILON).is_err());
    }

    #[test]
    fn coordinate_lat_lon_ok() {
        let coordinate = Coordinate::new(35.0, 135.0).unwrap();
        assert!(eq_f64(coordinate.lat(), 35.0));
        assert!(eq_f64(coordinate.lon(), 135.0));
    }

    #[test]
    fn contains_coordinate_ok() {
        assert!(contains_coordinate(&Coordinate::new(SOUTHERNMOST, WESTERNMOST).unwrap()).is_ok());
        assert!(contains_coordinate(
            &Coordinate::new(NORTHERNMOST + 1.0 - EPSILON, EASTERNMOST + 1.0 - EPSILON).unwrap()
        )
        .is_ok());
    }

    #[test]
    fn contains_coordinate_err() {
        assert!(contains_coordinate(
            &Coordinate::new(SOUTHERNMOST - EPSILON, WESTERNMOST).unwrap()
        )
        .is_err());
        assert!(contains_coordinate(
            &Coordinate::new(SOUTHERNMOST, WESTERNMOST - EPSILON).unwrap()
        )
        .is_err());
        assert!(contains_coordinate(
            &Coordinate::new(NORTHERNMOST + 1.0, EASTERNMOST + 1.0 - EPSILON).unwrap()
        )
        .is_err());
        assert!(contains_coordinate(
            &Coordinate::new(NORTHERNMOST + 1.0 - EPSILON, EASTERNMOST + 1.0).unwrap()
        )
        .is_err());
    }
}
