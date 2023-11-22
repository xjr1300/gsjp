
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

    /// メッシュの中心のを度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの中心の緯度と経度格納したタプル
    fn center(&self) -> (f64, f64) {
        let lat = (self.north() + self.south()) / 2.0;
        let lon = (self.east() + self.west()) / 2.0;
        (lat, lon)
    }

    /// メッシュの北東端の座標を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの北東端の緯度と経度格納したタプル
    fn north_east(&self) -> (f64, f64) {
        (self.north(), self.east())
    }

    /// メッシュの南東端の座標を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの南東端の緯度と経度格納したタプル
    fn south_east(&self) -> (f64, f64) {
        (self.south(), self.east())
    }

    /// メッシュの南西端の座標を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの南西端の緯度と経度格納したタプル
    fn south_west(&self) -> (f64, f64) {
        (self.south(), self.west())
    }

    /// メッシュの北西端の座標を度単位で返す。
    ///
    /// # 戻り値
    ///
    /// メッシュの北西端の緯度と経度格納したタプル
    fn north_west(&self) -> (f64, f64) {
        (self.north(), self.west())
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

    /// メッシュが隣接しているか確認する。
    ///
    /// 北東、南東、南西及び北西隣のメッシュは隣接していないと判定する。
    ///
    /// # 引数
    ///
    /// * `mesh` - 隣接しているか確認するメッシュ。
    ///
    /// # 戻り値
    ///
    /// 隣接しているかを示す`NeighborDirection`列挙型。
    fn is_joining(&self, mesh: &Self) -> NeighborDirection {
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
