use regex::Regex;

/// 国土基本図図郭
///
/// 国土地理院が平面直角座標系の原点からの距離を基準に定義した図郭です。
#[derive(Debug, Clone, PartialEq)]
pub enum BaseMapFrame {
    /// 地図情報レベル50000
    ///
    /// 平面直角座標系の19個の原点から、南北300km、東西160kmの範囲を
    /// 縦20個、横8個に分割した縦30km、横40kmの図郭です。
    ///
    /// 図郭名は、例えば平面直角座標系の第7系であれば、`07AB(4文字)`と表現されます。
    Level50000(String),

    /// 地図情報レベル5000
    ///
    /// 地図情報レベルの図郭を縦横に10等分した図郭で、縦3km、横4kmの図郭です。
    ///
    /// 図郭名は、例えば平面直角座標系の第7系であれば、`07AB10(6文字)`と表現されます。
    Level5000(String),

    /// 地図情報レベル2500
    ///
    /// 地図情報レベル2500の図郭を縦横に2等分した図郭で、縦1.5km、横2kmの図郭です。
    ///
    /// 図郭名は、例えば平面直角座標系の第7系であれば、`07AB103(7文字)`と表現されます。
    Level2500(String),

    /// 地図情報レベル1000
    ///
    /// 地図情報レベル5000の図郭を縦横に5等分した図郭で、縦600m、横800mの図郭です。
    ///
    /// 図郭名は、例えば平面直角座標系の第7系であれば、`07AB101A(8文字)`と表現されます。
    Level1000(String),

    /// 地図情報レベル500
    ///
    /// 地図情報レベル5000の図郭を縦横に10等分した図郭で、縦300m、横400mの図郭です。
    ///
    /// 図郭名は、例えば平面直角座標系の第7系であれば、`07AB1010(8文字)`と表現されます。
    Level500(String),
}

/// 地図情報レベル
#[derive(Debug, Clone, PartialEq)]
pub enum BaseMapFrameLevel {
    /// 地図情報レベル50000
    Level50000,
    /// 地図情報レベル5000
    Level5000,
    /// 地図情報レベル2500
    Level2500,
    /// 地図情報レベル1000
    Level1000,
    /// 地図情報レベル500
    Level500,
}

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
pub enum BaseMapFrameErr {
    #[error("図郭コードが不正です。")]
    InvalidFrameCode,
    #[error("X座標が範囲外です。")]
    XOutOfRange,
    #[error("Y座標が範囲外です。")]
    YOutOfRange,
    #[error("図郭がありません。")]
    FrameDoesNotExist,
}

pub type BaseMapFrameResult<T> = Result<T, BaseMapFrameErr>;

/// 国土基本図図郭の図郭名の正規表現
const LEVEL_50000_PATTERN: &str = "^[0-1][0-9][A-T][A-H]$";
const LEVEL_5000_PATTERN: &str = "^[0-1][0-9][A-T][A-H][0-9][0-9]$";
const LEVEL_2500_PATTERN: &str = "^[0-1][0-9][A-T][A-H][0-9][0-9][1-4]$";
const LEVEL_1000_PATTERN: &str = "^[0-1][0-9][A-T][A-H][0-9][0-9][0-4][A-E]$";
const LEVEL_500_PATTERN: &str = "^[0-1][0-9][A-T][A-H][0-9][0-9][0-9][0-9]$";

impl TryFrom<String> for BaseMapFrame {
    type Error = BaseMapFrameErr;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // 地図情報レベル50000
        let re = Regex::new(LEVEL_50000_PATTERN).unwrap();
        if re.is_match(&s) {
            return Ok(Self::Level50000(s));
        }
        // 地図情報レベル5000
        let re = Regex::new(LEVEL_5000_PATTERN).unwrap();
        if re.is_match(&s) {
            return Ok(Self::Level5000(s));
        }
        // 地図情報レベル2500
        let re = Regex::new(LEVEL_2500_PATTERN).unwrap();
        if re.is_match(&s) {
            return Ok(Self::Level2500(s));
        }
        // 地図情報レベル1000
        let re = Regex::new(LEVEL_1000_PATTERN).unwrap();
        if re.is_match(&s) {
            return Ok(Self::Level1000(s));
        }
        // 地図情報レベル500
        let re = Regex::new(LEVEL_500_PATTERN).unwrap();
        if re.is_match(&s) {
            return Ok(Self::Level500(s));
        }

        Err(BaseMapFrameErr::InvalidFrameCode)
    }
}

impl BaseMapFrame {
    /// 国土基本図図郭のレベルと座標から国土基本図図郭を取得する。
    ///
    /// # 引数
    ///
    /// * `level` - 国土基本図図郭のレベル
    /// * `x` - x座標
    /// * `y` - y座標
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭のレベルと座標から取得した国土基本図図郭
    pub fn from_level_xy(
        system: u8,
        level: BaseMapFrameLevel,
        x: f64,
        y: f64,
    ) -> BaseMapFrameResult<Self> {
        if !(-160_000_f64..160_000_f64).contains(&x) {
            return Err(BaseMapFrameErr::XOutOfRange);
        }
        if !(-300_000_f64..300_000_f64).contains(&y) {
            return Err(BaseMapFrameErr::YOutOfRange);
        }

        // 地図情報レベル50000の図郭を取得
        let y = y + 300_000_f64;
        let x = x + 160_000_f64;
        let y_times = (y / 30_000_f64) as u8;
        let x_times = (x / 40_000_f64) as u8;
        let yy = b'T' - y_times;
        let xx = b'A' + x_times;
        let code = format!("{:02}{}{}", system, yy as char, xx as char);
        if level == BaseMapFrameLevel::Level50000 {
            return Self::try_from(code);
        }

        // 地図情報レベル5000の図郭を取得
        let y = y - y_times as f64 * 30_000_f64;
        let x = x - x_times as f64 * 40_000_f64;
        let y_times = (y / 3_000_f64) as u8;
        let x_times = (x / 4_000_f64) as u8;
        let yy = 9 - y_times;
        let xx = x_times;
        let code = format!("{}{}{}", code, yy, xx);
        if level == BaseMapFrameLevel::Level5000 {
            return Self::try_from(code);
        }

        let y = y - y_times as f64 * 3_000_f64;
        let x = x - x_times as f64 * 4_000_f64;
        let code = if level == BaseMapFrameLevel::Level2500 {
            // 地図情報レベル2500の図郭を取得
            let n = if y < 1_500_f64 { 3 } else { 1 };
            let n = if x < 2_000_f64 { n } else { n + 1 };
            format!("{}{}", code, n)
        } else if level == BaseMapFrameLevel::Level1000 {
            // 地図情報レベル1000の図郭を取得
            let yy = 4 - (y / 600_f64) as u8;
            let xx = b'A' + (x / 800_f64) as u8;
            format!("{}{}{}", code, yy, xx as char)
        } else {
            // 地図情報レベル500の図郭を取得
            let yy = 9 - (y / 300_f64) as u8;
            let xx = (x / 400_f64) as u8;
            format!("{}{}{}", code, yy, xx)
        };

        Ok(Self::try_from(code).unwrap())
    }

    /// 国土基本図図郭のレベルを返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭のレベル
    pub fn level(&self) -> BaseMapFrameLevel {
        match self {
            BaseMapFrame::Level50000(_) => BaseMapFrameLevel::Level50000,
            BaseMapFrame::Level5000(_) => BaseMapFrameLevel::Level5000,
            BaseMapFrame::Level2500(_) => BaseMapFrameLevel::Level2500,
            BaseMapFrame::Level1000(_) => BaseMapFrameLevel::Level1000,
            BaseMapFrame::Level500(_) => BaseMapFrameLevel::Level500,
        }
    }

    /// 国土基本図図郭の幅をm単位で返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の幅（m単位）
    pub fn width(&self) -> u32 {
        match self {
            BaseMapFrame::Level50000(_) => 40_000,
            BaseMapFrame::Level5000(_) => 4_000,
            BaseMapFrame::Level2500(_) => 2_000,
            BaseMapFrame::Level1000(_) => 800,
            BaseMapFrame::Level500(_) => 400,
        }
    }

    /// 国土基本図図郭の高さをm単位で返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の高さ（m単位）
    pub fn height(&self) -> u32 {
        match self {
            BaseMapFrame::Level50000(_) => 30_000,
            BaseMapFrame::Level5000(_) => 3_000,
            BaseMapFrame::Level2500(_) => 1_500,
            BaseMapFrame::Level1000(_) => 600,
            BaseMapFrame::Level500(_) => 300,
        }
    }

    /// 国土基本図図郭の範囲を返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の範囲
    pub fn extent(&self) -> FrameExtent {
        let (x, y) = match self {
            BaseMapFrame::Level50000(code) => level_50000_left_top(code),
            BaseMapFrame::Level5000(code) => level_5000_left_top(code),
            BaseMapFrame::Level2500(code) => level_2500_left_top(code),
            BaseMapFrame::Level1000(code) => level_1000_left_top(code),
            BaseMapFrame::Level500(code) => level_500_left_top(code),
        };

        FrameExtent::new(x, y - self.height() as i32, x + self.width() as i32, y)
    }

    pub fn left_frame(&self) -> BaseMapFrameResult<Self> {
        match self {
            BaseMapFrame::Level50000(_) => todo!(),
            BaseMapFrame::Level5000(_) => todo!(),
            BaseMapFrame::Level2500(_) => todo!(),
            BaseMapFrame::Level1000(_) => todo!(),
            BaseMapFrame::Level500(_) => todo!(),
        }
    }
}

/// 国土基本図図郭の範囲
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameExtent {
    /// 国土基本図図郭の左端のx座標
    left: i32,
    /// 国土基本図図郭の下端のy座標
    bottom: i32,
    /// 国土基本図図郭の右端のx座標
    right: i32,
    /// 国土基本図図郭の上端のy座標
    top: i32,
}

impl FrameExtent {
    /// 国土基本図図郭の範囲を作成する。
    ///
    /// # 引数
    ///
    /// * `left` - 国土基本図図郭の左端のx座標
    /// * `bottom` - 国土基本図図郭の下端のy座標
    /// * `right` - 国土基本図図郭の右端のx座標
    /// * `top` - 国土基本図図郭の上端のy座標
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の範囲
    pub fn new(left: i32, bottom: i32, right: i32, top: i32) -> Self {
        Self {
            left,
            bottom,
            right,
            top,
        }
    }

    /// 国土基本図図郭の左端のx座標を返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の左端のx座標
    pub fn left(&self) -> i32 {
        self.left
    }

    /// 国土基本図図郭の下端のy座標を返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の下端のy座標
    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    /// 国土基本図図郭の右端のx座標を返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の右端のx座標
    pub fn right(&self) -> i32 {
        self.right
    }

    /// 国土基本図図郭の上端のy座標を返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の上端のy座標
    pub fn top(&self) -> i32 {
        self.top
    }

    /// 国土基本図図郭の幅をm単位で返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の幅（m単位）
    pub fn width(&self) -> u32 {
        (self.right - self.left) as u32
    }

    /// 国土基本図図郭の高さをm単位で返す。
    ///
    /// # 戻り値
    ///
    /// 国土基本図図郭の高さ（m単位）
    pub fn height(&self) -> u32 {
        (self.top - self.bottom) as u32
    }
}

/// 地図情報レベル50000の図郭の左上の座標を返す。
fn level_50000_left_top(code: &str) -> (i32, i32) {
    assert!(4 <= code.len(), "図郭コード({})の長さが不正です。", code);
    assert!(
        ('A'..='T').contains(&code.chars().nth(2).unwrap()),
        "図郭コード({})の3文字目が'A'から'T'のアルファベットではありません。",
        code
    );
    assert!(
        ('A'..='H').contains(&code.chars().nth(3).unwrap()),
        "図郭コード({})の4文字目が'A'から'H'までのファルファベットではありません。",
        code
    );

    (
        -160_000 + (code.chars().nth(3).unwrap() as i32 - 'A' as i32) * 40_000,
        300_000 - (code.chars().nth(2).unwrap() as i32 - 'A' as i32) * 30_000,
    )
}

/// 地図情報レベル5000の図郭の左上の座標を返す。
fn level_5000_left_top(code: &str) -> (i32, i32) {
    assert!(6 <= code.len(), "図郭コード({})の長さが不正です。", code);
    assert!(
        code.chars().nth(4).unwrap().is_ascii_digit(),
        "図郭コード({})の5文字目が数字ではありません。",
        code
    );
    assert!(
        code.chars().nth(5).unwrap().is_ascii_digit(),
        "図郭コード({})の6文字目が数字ではありません。",
        code
    );
    let (x, y) = level_50000_left_top(code);
    let c4 = code.chars().nth(4).unwrap().to_digit(10).unwrap() as i32;
    let c5 = code.chars().nth(5).unwrap().to_digit(10).unwrap() as i32;

    (x + c5 * 4_000, y - c4 * 3_000)
}

/// 地図情報レベル2500の図郭の左上の座標を返す。
fn level_2500_left_top(code: &str) -> (i32, i32) {
    assert!(7 <= code.len(), "図郭コード({})の長さが不正です。", code);
    assert!(
        ('0'..='4').contains(&code.chars().nth(6).unwrap()),
        "図郭コード({})の7文字目が'0'から'4'までの数字ではありません。",
        code
    );
    let (mut x, mut y) = level_5000_left_top(code);
    let c6 = code.chars().nth(6).unwrap().to_digit(10).unwrap() as i32;
    if c6 % 2 == 0 {
        x += 2_000;
    }
    if c6 >= 3 {
        y -= 1_500;
    }

    (x, y)
}

/// 地図情報レベル1000の図郭の左上の座標を返す。
fn level_1000_left_top(code: &str) -> (i32, i32) {
    assert!(8 <= code.len(), "図郭コード({})の長さが不正です。", code);
    assert!(
        ('0'..='4').contains(&code.chars().nth(6).unwrap()),
        "図郭コード({})の7文字目が'0'から'4'までの数字ではありません。",
        code
    );
    assert!(
        ('A'..='E').contains(&code.chars().nth(7).unwrap()),
        "図郭コード({})の8文字目が'A'から'E'までのアルファベットではありません。",
        code
    );
    let (mut x, mut y) = level_5000_left_top(code);
    let c6 = code.chars().nth(6).unwrap().to_digit(10).unwrap() as i32;
    y -= c6 * 600;
    let c7 = code.chars().nth(7).unwrap() as i32 - 'A' as i32;
    x += c7 * 800;

    (x, y)
}

/// 地図情報レベル500の図郭の左上の座標を返す。
fn level_500_left_top(code: &str) -> (i32, i32) {
    assert!(8 <= code.len(), "図郭コード({})の長さが不正です。", code);
    assert!(
        code.chars().nth(6).unwrap().is_ascii_digit(),
        "図郭コード({})の7文字目が数字ではありません。",
        code
    );
    assert!(
        code.chars().nth(7).unwrap().is_ascii_digit(),
        "図郭コード({})の8文字目が数字ではありません。",
        code
    );
    let (mut x, mut y) = level_5000_left_top(code);
    let c6 = code.chars().nth(6).unwrap().to_digit(10).unwrap() as i32;
    y -= c6 * 300;
    let c7 = code.chars().nth(7).unwrap().to_digit(10).unwrap() as i32;
    x += c7 * 400;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn level_50000_ok() {
        let codes = vec!["07AA", "07AH", "07TA", "07TH"];
        for code in codes {
            let frame = BaseMapFrame::try_from(String::from(code)).unwrap();
            assert_eq!(
                frame,
                BaseMapFrame::Level50000(code.to_string()),
                "{}",
                code
            );
        }
    }

    #[test]
    fn level_50000_err() {
        let codes = vec!["0", "11", "07A", "07A0", "07AI", "07UA"];
        for code in codes {
            let frame = BaseMapFrame::try_from(String::from(code));
            assert!(frame.is_err(), "{}", code);
            assert_eq!(frame.err().unwrap(), BaseMapFrameErr::InvalidFrameCode);
        }
    }

    #[test]
    fn width_ok() {
        let frames = vec![
            (
                40_000,
                BaseMapFrame::try_from(String::from("07AA")).unwrap(),
            ),
            (
                4_000,
                BaseMapFrame::try_from(String::from("07AA10")).unwrap(),
            ),
            (
                2_000,
                BaseMapFrame::try_from(String::from("07AA101")).unwrap(),
            ),
            (
                800,
                BaseMapFrame::try_from(String::from("07AA101A")).unwrap(),
            ),
            (
                400,
                BaseMapFrame::try_from(String::from("07AA1010")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.width(), "{:?}", frame);
        }
    }

    #[test]
    fn height_ok() {
        let frames = vec![
            (
                30_000,
                BaseMapFrame::try_from(String::from("07AA")).unwrap(),
            ),
            (
                3_000,
                BaseMapFrame::try_from(String::from("07AA10")).unwrap(),
            ),
            (
                1_500,
                BaseMapFrame::try_from(String::from("07AA101")).unwrap(),
            ),
            (
                600,
                BaseMapFrame::try_from(String::from("07AA101A")).unwrap(),
            ),
            (
                300,
                BaseMapFrame::try_from(String::from("07AA1010")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.height(), "{:?}", frame);
        }
    }

    #[test]
    fn level_50000_extent_ok() {
        let frames = vec![
            (
                // 最北西端の図郭
                FrameExtent::new(-160_000, 270_000, -120_000, 300_000),
                BaseMapFrame::try_from(String::from("07AA")).unwrap(),
            ),
            (
                // 最北東端の図郭
                FrameExtent::new(120_000, 270_000, 160_000, 300_000),
                BaseMapFrame::try_from(String::from("07AH")).unwrap(),
            ),
            (
                // 最南東端の図郭
                FrameExtent::new(120_000, -300_000, 160_000, -270_000),
                BaseMapFrame::try_from(String::from("07TH")).unwrap(),
            ),
            (
                // 最南西端の図郭
                FrameExtent::new(-160_000, -300_000, -120_000, -270_000),
                BaseMapFrame::try_from(String::from("07TA")).unwrap(),
            ),
            (
                // 原点左上の図郭
                FrameExtent::new(-40_000, 0, 0, 30_000),
                BaseMapFrame::try_from(String::from("07JD")).unwrap(),
            ),
            (
                // 原点右上の図郭
                FrameExtent::new(0, 0, 40_000, 30_000),
                BaseMapFrame::try_from(String::from("07JE")).unwrap(),
            ),
            (
                // 原点右下の図郭
                FrameExtent::new(0, -30_000, 40_000, 0),
                BaseMapFrame::try_from(String::from("07KE")).unwrap(),
            ),
            (
                // 原点左下の図郭
                FrameExtent::new(-40_000, -30_000, 0, 0),
                BaseMapFrame::try_from(String::from("07KD")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.extent(), "{:?}", frame);
        }
    }

    #[test]
    fn level_5000_extent_ok() {
        let frames = vec![
            (
                // 07JEの最北西端の図郭
                FrameExtent::new(0, 27_000, 4_000, 30_000),
                BaseMapFrame::try_from(String::from("07JE00")).unwrap(),
            ),
            (
                // 07JEの最北東端の図郭
                FrameExtent::new(36_000, 27_000, 40_000, 30_000),
                BaseMapFrame::try_from(String::from("07JE09")).unwrap(),
            ),
            (
                // 07JEの最南東端の図郭
                FrameExtent::new(36_000, 0, 40_000, 3_000),
                BaseMapFrame::try_from(String::from("07JE99")).unwrap(),
            ),
            (
                // 07JEの最南西端の図郭
                FrameExtent::new(0, 0, 4_000, 3_000),
                BaseMapFrame::try_from(String::from("07JE90")).unwrap(),
            ),
            (
                // 07JEの中心の北西の図郭
                FrameExtent::new(16_000, 15_000, 20_000, 18_000),
                BaseMapFrame::try_from(String::from("07JE44")).unwrap(),
            ),
            (
                // 07JEの中心の北東の図郭
                FrameExtent::new(20_000, 15_000, 24_000, 18_000),
                BaseMapFrame::try_from(String::from("07JE45")).unwrap(),
            ),
            (
                // 07JEの中心の南東の図郭
                FrameExtent::new(20_000, 12_000, 24_000, 15_000),
                BaseMapFrame::try_from(String::from("07JE55")).unwrap(),
            ),
            (
                // 07JEの中心の南西の図郭
                FrameExtent::new(16_000, 12_000, 20_000, 15_000),
                BaseMapFrame::try_from(String::from("07JE54")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.extent(), "{:?}", frame);
        }
    }

    #[test]
    fn level_2500_extent_ok() {
        let frames = vec![
            (
                // 07JE90の最北西端の図郭
                FrameExtent::new(0, 1_500, 2_000, 3_000),
                BaseMapFrame::try_from(String::from("07JE901")).unwrap(),
            ),
            (
                // 07JE90の最北東端の図郭
                FrameExtent::new(2_000, 1_500, 4_000, 3_000),
                BaseMapFrame::try_from(String::from("07JE902")).unwrap(),
            ),
            (
                // 07JE90の最南東端の図郭
                FrameExtent::new(2_000, 0, 4_000, 1_500),
                BaseMapFrame::try_from(String::from("07JE904")).unwrap(),
            ),
            (
                // 07JE90の最南西端の図郭
                FrameExtent::new(0, 0, 2_000, 1_500),
                BaseMapFrame::try_from(String::from("07JE903")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.extent(), "{:?}", frame);
        }
    }

    #[test]
    fn level_1000_extent_ok() {
        let frames = vec![
            (
                // 07JE90の最北西端の図郭
                FrameExtent::new(0, 2_400, 800, 3_000),
                BaseMapFrame::try_from(String::from("07JE900A")).unwrap(),
            ),
            (
                // 07JE90の最北東端の図郭
                FrameExtent::new(3_200, 2_400, 4_000, 3_000),
                BaseMapFrame::try_from(String::from("07JE900E")).unwrap(),
            ),
            (
                // 07JE90の最南東端の図郭
                FrameExtent::new(3_200, 0, 4_000, 600),
                BaseMapFrame::try_from(String::from("07JE904E")).unwrap(),
            ),
            (
                // 07JE90の最南西端の図郭
                FrameExtent::new(0, 0, 800, 600),
                BaseMapFrame::try_from(String::from("07JE904A")).unwrap(),
            ),
            (
                // 07JE90の中心の図郭
                FrameExtent::new(1_600, 1_200, 2_400, 1_800),
                BaseMapFrame::try_from(String::from("07JE902C")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.extent(), "{:?}", frame);
        }
    }

    #[test]
    fn level_500_extent_ok() {
        let frames = vec![
            (
                // 07JE90の最北西端の図郭
                FrameExtent::new(0, 2_700, 400, 3_000),
                BaseMapFrame::try_from(String::from("07JE9000")).unwrap(),
            ),
            (
                // 07JE90の最北東端の図郭
                FrameExtent::new(3_600, 2_700, 4_000, 3_000),
                BaseMapFrame::try_from(String::from("07JE9009")).unwrap(),
            ),
            (
                // 07JE90の最南東端の図郭
                FrameExtent::new(3_600, 0, 4_000, 300),
                BaseMapFrame::try_from(String::from("07JE9099")).unwrap(),
            ),
            (
                // 07JE90の最南西端の図郭
                FrameExtent::new(0, 0, 400, 300),
                BaseMapFrame::try_from(String::from("07JE9090")).unwrap(),
            ),
            (
                // 07JE90の中心の左上の図郭
                FrameExtent::new(1_600, 1_500, 2_000, 1_800),
                BaseMapFrame::try_from(String::from("07JE9044")).unwrap(),
            ),
            (
                // 07JE90の中心の右上の図郭
                FrameExtent::new(2_000, 1_500, 2_400, 1_800),
                BaseMapFrame::try_from(String::from("07JE9045")).unwrap(),
            ),
            (
                // 07JE90の中心の右下の図郭
                FrameExtent::new(2_000, 1_200, 2_400, 1_500),
                BaseMapFrame::try_from(String::from("07JE9055")).unwrap(),
            ),
            (
                // 07JE90の中心の左下の図郭
                FrameExtent::new(1_600, 1_200, 2_000, 1_500),
                BaseMapFrame::try_from(String::from("07JE9054")).unwrap(),
            ),
        ];
        for (expected, frame) in frames {
            assert_eq!(expected, frame.extent(), "{:?}", frame);
        }
    }

    #[test]
    fn from_level_xy_50000_ok() {
        let targets = vec![
            // 最北西端の図郭
            ("07AA", -160_000_f64, 300_000_f64 - EPSILON),
            // 最北東端の図郭
            ("07AH", 160_000_f64 - EPSILON, 300_000_f64 - EPSILON),
            // 最南東端の図郭
            ("07TH", 160_000_f64 - EPSILON, -300_000_f64),
            // 最南西端の図郭
            ("07TA", -160_000_f64, -300_000_f64),
            // 中心の図郭
            ("07JE", 0_f64, 0_f64),
            // 中心の左上の図郭
            ("07JD", -40_000_f64, 0_f64),
            // // 中心の右上の図郭
            // ("07JE", 0_f64, 0_f64),
            // 中心の右下の図郭
            ("07KE", 0_f64, -30_000_f64),
            // 中心の左下の図郭
            ("07KD", -40_000_f64, -30_000_f64),
            // 任意の図郭
            ("07FG", 80_000_f64, 120_000_f64),
        ];
        for (expected, x, y) in targets {
            let frame =
                BaseMapFrame::from_level_xy(7, BaseMapFrameLevel::Level50000, x, y).unwrap();
            match frame {
                BaseMapFrame::Level50000(code) => {
                    assert_eq!(expected, code, "expected: {}, actual: {}", expected, code)
                }
                _ => panic!("想定していない地図情報レベルを取得しました({})。", expected),
            }
        }
    }

    /// 地図情報レベル50000の07NC図郭でテスト
    ///
    /// * 07NCの最北端は-90_000
    /// * 07NCの最南端は-120_000
    /// * 07NCの最西端は-80_000
    /// * 07NCの最東端は-40_000
    #[test]
    fn from_level_xy_5000_ok() {
        let targets = vec![
            // 最北西端の図郭
            ("07NC00", -80_000_f64, -90_000_f64 - EPSILON),
            // 最北東端の図郭
            ("07NC09", -40_000_f64 - EPSILON, -90_000_f64 - EPSILON),
            // 最南東端の図郭
            ("07NC99", -40_000_f64 - EPSILON, -120_000_f64),
            // 最南西端の図郭
            ("07NC90", -80_000_f64, -120_000_f64),
            // 中心の図郭
            ("07NC45", -60_000_f64, -105_000_f64),
            // 中心の左上の図郭
            ("07NC44", -64_000_f64, -105_000_f64),
            // // 中心の右上の図郭
            // ("07NC45", -60_000_f64, -105_000_f64),
            // 中心の右下の図郭
            ("07NC55", -60_000_f64, -108_000_f64),
            // 中心の左下の図郭
            ("07NC54", -64_000_f64, -108_000_f64),
            // 任意の図郭
            ("07NC37", -52_000_f64, -102_000_f64),
        ];
        for (expected, x, y) in targets {
            let frame = BaseMapFrame::from_level_xy(7, BaseMapFrameLevel::Level5000, x, y).unwrap();
            match frame {
                BaseMapFrame::Level5000(code) => {
                    assert_eq!(expected, code, "expected: {}, actual: {}", expected, code)
                }
                _ => panic!("想定していない地図情報レベルを取得しました({})。", expected),
            }
        }
    }

    /// 地図情報レベル5000の07NC00図郭でテスト
    ///
    /// * 07NC00の最北端は-90_000
    /// * 07NC00の最南端は-93_000
    /// * 07NC00の最西端は-80_000
    /// * 07NC00の最東端は-76_000
    #[test]
    fn from_level_xy_2500_ok() {
        let targets = vec![
            // 最北西端の図郭
            ("07NC001", -80_000_f64, -91_500_f64),
            // 最北東端の図郭
            ("07NC002", -78_000_f64, -91_500_f64),
            // 最南東端の図郭
            ("07NC004", -78_000_f64, -93_000_f64),
            // 最南西端の図郭
            ("07NC003", -80_000_f64, -93_000_f64),
        ];
        for (expected, x, y) in targets {
            let frame = BaseMapFrame::from_level_xy(7, BaseMapFrameLevel::Level2500, x, y).unwrap();
            match frame {
                BaseMapFrame::Level2500(code) => {
                    assert_eq!(expected, code, "expected: {}, actual: {}", expected, code)
                }
                _ => panic!("想定していない地図情報レベルを取得しました({})。", expected),
            }
        }
    }

    /// 地図情報レベル5000の07NC00図郭でテスト
    ///
    /// * 07NC00の最北端は-90_000
    /// * 07NC00の最南端は-93_000
    /// * 07NC00の最西端は-80_000
    /// * 07NC00の最東端は-76_000
    #[test]
    fn from_level_xy_1000_ok() {
        let targets = vec![
            // 最北西端の図郭
            ("07NC000A", -80_000_f64, -90_600_f64),
            // 最北東端の図郭
            ("07NC000E", -76_800_f64, -90_600_f64),
            // 最南東端の図郭
            ("07NC004E", -76_800_f64, -93_000_f64),
            // 最南西端の図郭
            ("07NC004A", -80_000_f64, -93_000_f64),
            // 中心の図郭
            ("07NC002C", -78_400_f64, -91_800_f64),
        ];
        for (expected, x, y) in targets {
            let frame = BaseMapFrame::from_level_xy(7, BaseMapFrameLevel::Level1000, x, y).unwrap();
            match frame {
                BaseMapFrame::Level1000(code) => {
                    assert_eq!(expected, code, "expected: {}, actual: {}", expected, code)
                }
                _ => panic!("想定していない地図情報レベルを取得しました({})。", expected),
            }
        }
    }

    /// 地図情報レベル5000の07NC00図郭でテスト
    ///
    /// * 07NC00の最北端は-90_000
    /// * 07NC00の最南端は-93_000
    /// * 07NC00の最西端は-80_000
    /// * 07NC00の最東端は-76_000
    #[test]
    fn from_level_xy_500_ok() {
        let targets = vec![
            // 最北西端の図郭
            ("07NC0000", -80_000_f64, -90_300_f64),
            // 最北東端の図郭
            ("07NC0009", -76_400_f64, -90_300_f64),
            // 最南東端の図郭
            ("07NC0099", -76_400_f64, -93_000_f64),
            // 最南西端の図郭
            ("07NC0090", -80_000_f64, -93_000_f64),
            // 中心の左上の図郭
            ("07NC0044", -78_400_f64, -91_500_f64),
            // 中心の右上の図郭
            ("07NC0045", -78_000_f64, -91_500_f64),
            // 中心の右下の図郭
            ("07NC0055", -78_000_f64, -91_800_f64),
            // 中心の左下の図郭
            ("07NC0054", -78_400_f64, -91_800_f64),
        ];
        for (expected, x, y) in targets {
            let frame = BaseMapFrame::from_level_xy(7, BaseMapFrameLevel::Level500, x, y).unwrap();
            match frame {
                BaseMapFrame::Level500(code) => {
                    assert_eq!(expected, code, "expected: {}, actual: {}", expected, code)
                }
                _ => panic!("想定していない地図情報レベルを取得しました({})。", expected),
            }
        }
    }
}
