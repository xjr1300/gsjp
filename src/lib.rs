use std::borrow::Cow;

/// <https://www.gsi.go.jp/KOKUJYOHO/center.htm>
/// | 区分 | 場所 | 経度 | 緯度 |
/// | --- | --- | --- | --- |
/// | 最東端 | 東京都 南鳥島 | 153°59′12″ | 24°16′59″ |
/// | 最西端 | 沖縄県 与那国島 | 122°55′57″ | 24°27′05″ |
/// | 最南端 | 東京都 沖ノ鳥島| 136°04′11″ | 20°25′31″ |
/// | 最北端 | 北海道 択捉島 | 148°45′08″ | 45°33′26″ |
pub const NORTHERNMOST_LAT: f64 = 45.0 + 33.0 / 60.0 + 26.0 / 3600.0;
pub const SOUTHERNMOST_LAT: f64 = 20.0 + 25.0 / 60.0 + 31.0 / 3600.0;
pub const EASTERNMOST_LON: f64 = 153.0 + 59.0 / 60.0 + 12.0 / 3600.0;
pub const WESTERNMOST_LON: f64 = 122.0 + 55.0 / 60.0 + 57.0 / 3600.0;

/// メッシュトレイト
pub trait Mesh: Sized {
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

    /// メッシュの南端を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの南端の緯度
    fn south(&self) -> f64;

    /// メッシュの東端を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの東端の経度
    fn east(&self) -> f64;

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
    fn north_mesh(&self) -> Self;

    /// 東隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 東隣のメッシュ
    fn east_mesh(&self) -> Self;

    /// 南隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 南隣のメッシュ
    fn south_mesh(&self) -> Self;

    /// 西隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 西隣のメッシュ
    fn west_mesh(&self) -> Self;

    /// 北東隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 北東隣のメッシュ
    fn north_east_mesh(&self) -> Self {
        self.north_mesh().east_mesh()
    }

    /// 南東隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 南東隣のメッシュ
    fn south_east_mesh(&self) -> Self {
        self.south_mesh().east_mesh()
    }

    /// 南西隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 南西隣のメッシュ
    fn south_west_mesh(&self) -> Self {
        self.south_mesh().west_mesh()
    }

    /// 北西隣のメッシュを返す。
    ///
    /// # 戻り値
    ///
    /// 北西隣のメッシュ
    fn north_west_mesh(&self) -> Self {
        self.north_mesh().west_mesh()
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
    fn is_neighboring(&self, mesh: &Self) -> NeighborDirection {
        if self.north_mesh().code() == mesh.code() {
            return NeighborDirection::North;
        } else if self.east_mesh().code() == mesh.code() {
            return NeighborDirection::East;
        } else if self.south_mesh().code() == mesh.code() {
            return NeighborDirection::South;
        } else if self.west_mesh().code() == mesh.code() {
            return NeighborDirection::West;
        }

        NeighborDirection::None
    }
}

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
pub fn validate_lat(lat: f64) -> Result<f64, GSJPError> {
    if !(SOUTHERNMOST_LAT..=NORTHERNMOST_LAT).contains(&lat) {
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
pub fn validate_lon(lon: f64) -> Result<f64, GSJPError> {
    if !(WESTERNMOST_LON..=EASTERNMOST_LON).contains(&lon) {
        return Err(GSJPError::OutOfRange("経度が範囲外です。".into()));
    }

    Ok(lon)
}

/// 度分秒
pub struct DMS {
    /// 度
    degree: i16,
    /// 分
    minute: u8,
    /// 秒
    second: f64,
}

impl DMS {
    /// 度分秒を作成する。
    ///
    /// # 引数
    ///
    /// * `degree` - 度
    /// * `minute` - 分
    /// * `second` - 秒
    ///
    /// # 戻り値
    ///
    /// 度分秒
    pub fn new(degree: i16, minute: u8, second: f64) -> Self {
        Self {
            degree,
            minute,
            second,
        }
    }

    /// 度を返す。
    ///
    /// # 戻り値
    ///
    /// 度
    pub fn degree(&self) -> i16 {
        self.degree
    }

    /// 分を返す。
    ///
    /// # 戻り値
    ///
    /// 分
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// 秒を返す。
    ///
    /// # 戻り値
    ///
    /// 秒
    pub fn second(&self) -> f64 {
        self.second
    }

    /// 緯度に変換する。
    ///
    /// # 戻り値
    ///
    /// 緯度
    pub fn to_lat(&self) -> Result<f64, GSJPError> {
        let sign = if self.degree >= 0 { 1.0 } else { -1.0 };
        let lat = self.degree as f64 * sign + self.minute as f64 / 60.0 + self.second / 3600.0;

        validate_lat(lat * sign)
    }

    /// 経度に変換する。
    ///
    /// # 戻り値
    ///
    /// 経度
    pub fn to_lon(&self) -> Result<f64, GSJPError> {
        let sign = if self.degree >= 0 { 1.0 } else { -1.0 };
        let lon = self.degree as f64 * sign + self.minute as f64 / 60.0 + self.second / 3600.0;

        validate_lon(lon * sign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_lat() {
        assert!(validate_lat(SOUTHERNMOST_LAT).is_ok());
        assert!(validate_lat(NORTHERNMOST_LAT).is_ok());
    }

    #[test]
    fn validate_lat_err() {
        assert!(validate_lat(SOUTHERNMOST_LAT - 1e-8).is_err());
        assert!(validate_lat(NORTHERNMOST_LAT + 1e-8).is_err());
    }

    #[test]
    fn validate_lon_ok() {
        assert!(validate_lon(WESTERNMOST_LON).is_ok());
        assert!(validate_lon(EASTERNMOST_LON).is_ok());
    }

    #[test]
    fn validate_lon_err() {
        assert!(validate_lon(WESTERNMOST_LON - 1e-8).is_err());
        assert!(validate_lon(EASTERNMOST_LON + 1e-8).is_err());
    }

    #[test]
    fn coordinate_new_ok() {
        assert!(Coordinate::new(NORTHERNMOST_LAT, WESTERNMOST_LON).is_ok());
        assert!(Coordinate::new(NORTHERNMOST_LAT, EASTERNMOST_LON).is_ok());
        assert!(Coordinate::new(SOUTHERNMOST_LAT, WESTERNMOST_LON).is_ok());
        assert!(Coordinate::new(SOUTHERNMOST_LAT, EASTERNMOST_LON).is_ok());
    }

    #[test]
    fn coordinate_new_err() {
        assert!(Coordinate::new(NORTHERNMOST_LAT + 1e-8, WESTERNMOST_LON).is_err());
        assert!(Coordinate::new(NORTHERNMOST_LAT, WESTERNMOST_LON - 1e-8).is_err());
        assert!(Coordinate::new(SOUTHERNMOST_LAT - 1e-8, WESTERNMOST_LON).is_err());
        assert!(Coordinate::new(SOUTHERNMOST_LAT, EASTERNMOST_LON + 1e-8).is_err());
    }

    #[test]
    fn coordinate_lat_lon_ok() {
        let coordinate = Coordinate::new(35.0, 135.0).unwrap();
        assert!((coordinate.lat() - 35.0).abs() < 1e-8);
        assert!((coordinate.lon() - 135.0).abs() < 1e-8);
    }

    #[test]
    fn dms_ok() {
        let dms = DMS::new(35, 50, 35.49);
        assert_eq!(35, dms.degree());
        assert_eq!(50, dms.minute());
        assert!((dms.second() - 35.49).abs() < 1e-8);
    }

    #[test]
    fn dms_minus_ok() {
        let dms = DMS::new(-35, 50, 35.49);
        assert_eq!(-35, dms.degree());
        assert_eq!(50, dms.minute());
        assert!((dms.second() - 35.49).abs() < 1e-8);
    }

    #[test]
    fn dms_to_lat_ok() {
        let dms = DMS::new(35, 50, 35.49);
        assert!(((35.0 + 50.0 / 60.0 + 35.49 / 3600.0) - dms.to_lat().unwrap()).abs() < 1e-8);
    }

    #[test]
    fn dms_to_lat_err() {
        let dms = DMS::new(-35, 50, 35.49);
        // let expected = (35.0 + 50.0 / 60.0 + 35.49 / 3600.0) * -1.0;
        // assert!(
        //     (expected - dms.to_lat().unwrap()).abs() < 1e-8,
        //     "expected: {}, actual: {}",
        //     expected,
        //     dms.to_lat().unwrap()
        // );
        assert!(dms.to_lat().is_err());
    }

    #[test]
    fn dms_to_lon_ok() {
        let dms = DMS::new(135, 50, 35.49);
        assert!(((135.0 + 50.0 / 60.0 + 35.49 / 3600.0) - dms.to_lon().unwrap()).abs() < 1e-8);
    }

    #[test]
    fn dms_to_lon_err() {
        let dms = DMS::new(-135, 50, 35.49);
        // let expected = (135.0 + 50.0 / 60.0 + 35.49 / 3600.0) * -1.0;
        // assert!(
        //     (expected - dms.to_lon().unwrap()).abs() < 1e-8,
        //     "expected: {}, actual: {}",
        //     expected,
        //     dms.to_lon().unwrap()
        // );
        assert!(dms.to_lon().is_err());
    }
}
