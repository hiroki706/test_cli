use clap::Parser;
use soup::{NodeExt, QueryBuilderExt, Soup};
use std::path::PathBuf;

fn main() {
    let Args {
        data_path,
        output_name,
    } = Args::parse();
    let data = std::fs::read_to_string(data_path).expect("ファイルが読み込めませんでした");
    let soup = Soup::new(&data);
    let svgs = soup.tag("svg").find_all();
    let output_slides = svgs.map(|node| node.display()).collect::<Vec<_>>().join("");

    let texts = soup.tag("div").class("col-3").find_all();
    let output_texts = texts
        .map(|node| node.text())
        .collect::<Vec<_>>()
        .join("\n- ");

    std::fs::write(output_name.with_extension("html"), output_slides)
        .expect("ファイルが書き込めませんでした");
    std::fs::write(output_name.with_extension("md"), output_texts)
        .expect("ファイルが書き込めませんでした");
}

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    /// 授業資料のパス
    data_path: PathBuf,
    /// 出力先のファイル名
    #[clap(short, long, default_value = "output")]
    output_name: PathBuf,
}
