// pub mod generic;
use chrono::Datelike;
use text_to_ascii_art::to_art;
use crate::config::model::Config;

fn copyright(cfg: &Config) {
    let (v,a,h,d,r) = cfg.get_config();
    let current_date = chrono::Utc::now();
    let year = current_date.year();

    let y = if year == 2025 {
        "2025".to_string()
    } else {
        format!("2025 - {}", year)
    };

    println!("\n(c) {} - {} - Version {}", y, a, v);
    println!("📆 {}\nVisit: {}\nGithub: {}\n\n", d, h, r);

}

pub fn show(cfg: &Config) {
    let art = format!("{}", env!("CARGO_PKG_NAME"));
    match to_art(art.to_string(), "", 0, 0, 0) {
        Ok(art) => println!("\n\n\n{}", art),
        Err(e) => tracing::error!("{}", e),
    }
    copyright(cfg);
}