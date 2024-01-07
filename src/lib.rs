#![allow(non_snake_case)]

pub mod app;
pub mod pages {
    pub mod browse;
    pub use browse::Browse;
    pub mod stats;
    pub use stats::StatsPage;
}
pub mod components {
    pub mod table;
    pub use table::DFTable;

    pub mod top_bar;
    pub use top_bar::TopBar;

    pub mod cards;
    pub use cards::Card;

    pub mod head;
    pub use head::DefaultHead;

    pub mod assets;
    pub use assets::{StockIcon, StageImg};
}
pub mod utils;

// TODO move this to slpprocess
#[macro_export]
macro_rules! static_str {
    ($x:expr) => {
        Into::<&'static str>::into($x)
    }
}




// async fn parse_async(path: &str, multithreaded: bool) -> Vec<GameStub> {
//     if path.is_empty() {
//         return Vec::new();
//     }

//     let f_path = Path::new(path);
//     if f_path.is_file() {
//         return vec![Game::stub(f_path).unwrap()];
//     }
//     if f_path.is_dir() {
//         let files: Vec<PathBuf> = fs::read_dir(f_path)
//             .unwrap()
//             .filter_map(|file| {
//                 if let Ok(entry) = file {
//                     let path = entry.path();
//                     if path.is_file() && path.extension().unwrap() == "slp" {
//                         Some(path)
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .collect();

//         let mut result: Vec<GameStub> = if multithreaded {
//             files
//                 .par_iter()
//                 .filter_map(|path| Game::stub(path.as_path()).ok())
//                 .collect()
//         } else {
//             files
//                 .iter()
//                 .filter_map(|path| {
//                     #[cfg(debug_assertions)]
//                     dbg!(path);

//                     Game::stub(path.as_path()).ok()
//                 })
//                 .collect()
//         };

//         // sort newest -> oldest by date
//         result.sort_by(|a,b| b.cmp(a));

//         return result;
//     }
//     panic!("invalid file path: {f_path:?}")
// }
