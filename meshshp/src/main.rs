use std::path::PathBuf;

use clap::Parser;
use gsjp::{Coordinate, Mesh, Mesh1, Mesh2, Mesh3, Mesh4, Mesh5, Mesh6};
use shapefile::dbase::{FieldName, FieldValue, Record, TableWriterBuilder};
use shapefile::{Point, Polygon, PolygonRing, ShapeWriter};

/// 北緯48度から20度、東経118度から150度までの範囲の標準地域メッシュをShapeファイル形式で出力する。
fn main() {
    // コマンドライン引数をパース
    let args = Args::parse();

    let boundary = Boundary {
        northernmost: args.northernmost,
        southernmost: args.southernmost,
        westernmost: args.westernmost,
        easternmost: args.easternmost,
    };
    let shp_path = PathBuf::from(&args.output);
    let dbf_path = shp_path.with_extension("dbf");

    let width = args.mesh_kind.width();
    let height = args.mesh_kind.height();

    let mut shape_writer = ShapeWriter::from_path(shp_path).unwrap();
    let code_field_name = FieldName::try_from("code").unwrap();
    let mut table_writer = TableWriterBuilder::new()
        .add_character_field(code_field_name, 11)
        .build_with_file_dest(dbf_path)
        .unwrap();

    // 西から東、南から北に向かってメッシュを出力
    // メッシュの中心の座標を走査
    let mut lat = 20.0 + height / 2.0;
    while lat < 48.0 {
        let mut lon = 118.0 + width / 2.0;
        while lon < 150.0 {
            if boundary.contains(lat, lon) {
                let mesh_info = args.mesh_kind.mesh_info(lat, lon);
                let mesh = Polygon::with_rings(vec![PolygonRing::Outer(vec![
                    Point::new(mesh_info.west, mesh_info.north),
                    Point::new(mesh_info.east, mesh_info.north),
                    Point::new(mesh_info.east, mesh_info.south),
                    Point::new(mesh_info.west, mesh_info.south),
                    Point::new(mesh_info.west, mesh_info.north),
                ])]);
                shape_writer.write_shape(&mesh).unwrap();
                let mut record = Record::default();
                record.insert(
                    String::from("code"),
                    FieldValue::Character(Some(mesh_info.code)),
                );
                table_writer.write_record(&record).unwrap();
            }
            lon += width;
        }
        lat += height;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum MeshKind {
    /// 第1次地域区画
    Mesh1,

    /// 第2次地域区画
    Mesh2,

    /// 基準地域メッシュ（第3次地域区画）
    Mesh3,

    /// 2分の１地域メッシュ（分割地域メッシュ）
    Mesh4,

    /// 4分の１地域メッシュ（分割地域メッシュ）
    Mesh5,

    /// 8分の１地域メッシュ（分割地域メッシュ）
    Mesh6,
}

struct MeshInfo {
    /// メッシュコード
    code: String,
    /// メッシュの北端の緯度
    north: f64,
    /// メッシュの南端の緯度
    south: f64,
    /// メッシュの西端の経度
    west: f64,
    /// メッシュの東端の経度
    east: f64,
}

impl MeshKind {
    /// メッシュの幅を度単位で返す。
    fn width(&self) -> f64 {
        match self {
            MeshKind::Mesh1 => 1.0,
            MeshKind::Mesh2 => 7.0 / 60.0 + 30.0 / 3600.0,
            MeshKind::Mesh3 => 45.0 / 3600.0,
            MeshKind::Mesh4 => 22.5 / 3600.0,
            MeshKind::Mesh5 => 11.25 / 3600.0,
            MeshKind::Mesh6 => 5.625 / 3600.0,
        }
    }

    /// メッシュの高さを度単位で返す。
    fn height(&self) -> f64 {
        match self {
            MeshKind::Mesh1 => 40.0 / 60.0,
            MeshKind::Mesh2 => 5.0 / 60.0,
            MeshKind::Mesh3 => 30.0 / 3600.0,
            MeshKind::Mesh4 => 15.0 / 3600.0,
            MeshKind::Mesh5 => 7.5 / 3600.0,
            MeshKind::Mesh6 => 3.75 / 3600.0,
        }
    }

    /// メッシュのコードを返す。
    fn mesh_info(&self, lat: f64, lon: f64) -> MeshInfo {
        let coord = Coordinate::new(lat, lon).unwrap();
        match self {
            MeshKind::Mesh1 => {
                let mesh = Mesh1::from_coordinate(coord).unwrap();
                MeshInfo {
                    code: mesh.code().to_string(),
                    north: mesh.north(),
                    south: mesh.south(),
                    west: mesh.west(),
                    east: mesh.east(),
                }
            }
            MeshKind::Mesh2 => {
                let mesh = Mesh2::from_coordinate(coord).unwrap();
                MeshInfo {
                    code: mesh.code().to_string(),
                    north: mesh.north(),
                    south: mesh.south(),
                    west: mesh.west(),
                    east: mesh.east(),
                }
            }
            MeshKind::Mesh3 => {
                let mesh = Mesh3::from_coordinate(coord).unwrap();
                MeshInfo {
                    code: mesh.code().to_string(),
                    north: mesh.north(),
                    south: mesh.south(),
                    west: mesh.west(),
                    east: mesh.east(),
                }
            }
            MeshKind::Mesh4 => {
                let mesh = Mesh4::from_coordinate(coord).unwrap();
                MeshInfo {
                    code: mesh.code().to_string(),
                    north: mesh.north(),
                    south: mesh.south(),
                    west: mesh.west(),
                    east: mesh.east(),
                }
            }
            MeshKind::Mesh5 => {
                let mesh = Mesh5::from_coordinate(coord).unwrap();
                MeshInfo {
                    code: mesh.code().to_string(),
                    north: mesh.north(),
                    south: mesh.south(),
                    west: mesh.west(),
                    east: mesh.east(),
                }
            }
            MeshKind::Mesh6 => {
                let mesh = Mesh6::from_coordinate(coord).unwrap();
                MeshInfo {
                    code: mesh.code().to_string(),
                    north: mesh.north(),
                    south: mesh.south(),
                    west: mesh.west(),
                    east: mesh.east(),
                }
            }
        }
    }
}

/// コマンドライン引数
#[derive(clap::Parser)]
#[clap(name = "meshshp", version = "0.1.0", author = "xjr1300.04@gmail.com")]
struct Args {
    /// 出力するメッシュの種類
    mesh_kind: MeshKind,

    /// 出力するメッシュの最北端の緯度
    #[arg(short, long, help = "格子点を出力する最北端の緯度(例:36.0)")]
    northernmost: Option<f64>,

    /// 出力するメッシュの最南端の緯度
    #[arg(short, long, help = "格子点を出力する最南端の緯度(例:35.0)")]
    southernmost: Option<f64>,

    /// 出力するメッシュの最西端の経度
    #[arg(short, long, help = "格子点を出力する最西端の経度(例:135.0)")]
    westernmost: Option<f64>,

    /// 出力するメッシュの最西端の経度
    #[arg(short, long, help = "格子点を出力する最東端の経度(例:136.0)")]
    easternmost: Option<f64>,

    /// 出力Shapeファイル
    #[arg(help = "出力Shapeファイルのパス")]
    output: String,
}

#[derive(Default)]
pub struct Boundary {
    northernmost: Option<f64>,
    southernmost: Option<f64>,
    westernmost: Option<f64>,
    easternmost: Option<f64>,
}

impl Boundary {
    fn contains(&self, lat: f64, lon: f64) -> bool {
        if let Some(northernmost) = self.northernmost {
            if northernmost < lat {
                return false;
            }
        }
        if let Some(southernmost) = self.southernmost {
            if lat < southernmost {
                return false;
            }
        }
        if let Some(westernmost) = self.westernmost {
            if lon < westernmost {
                return false;
            }
        }
        if let Some(easternmost) = self.easternmost {
            if easternmost < lon {
                return false;
            }
        }

        true
    }
}
