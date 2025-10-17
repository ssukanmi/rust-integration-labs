// #[derive(Debug)]
// enum Cereal {
//     Barley,
//     Millet,
//     Rice,
//     Rye,
//     Spelt,
//     Wheat,
// }

// use std::thread;

fn main() {
    // let mut data = 100;
    // thread::spawn(|| {
    //     data = 500;
    // });
    // thread::spawn(|| data = 1000);
    // println!("{}", data);

    // let mut grains: Vec<Cereal> = vec![];
    // grains.push(Cereal::Rye);
    // drop(grains);
    // println!("{:?}", grains);
    // greet_world();

    // let penguin_data = "\
    // common name,length (cm)
    // Little penguin,33
    // Yellow-eyed penguin,65
    // Fiordland penguin,60
    // Invalid,data
    // ";

    // let records = penguin_data.lines();

    // for (i, record) in records.enumerate() {
    //     if i == 0 || record.trim().is_empty() {
    //         continue;
    //     }

    //     // let fields = record.split(",").map(|f| f.trim()).collect::<Vec<_>>();
    //     let fields: Vec<_> = record.split(",").map(|f| f.trim()).collect();

    //     if cfg!(debug_assertions) {
    //         eprintln!("debug: {:?} -> {:?}", record, fields);
    //     }

    //     let name = fields[0];

    //     if let Ok(length) = fields[1].parse::<f32>() {
    //         println!("{}, {}cm", name, length);
    //     }

    // }
}

// fn greet_world() {
//     println!("Hello, world!");
//     let southern_german = "Grüß Gott!";
//     let japan = "ハロー・ワールド";
//     let regions = [southern_german, japan];

//     for region in regions.iter() {
//         println!("{}", &region);
//     }

//     println!("{regions:#?}");
// }
