use rmp_serde;
use serde::{Serialize, Deserialize};


fn main() {
    #[derive(Serialize, Deserialize, Debug)]
    enum Cmd {
        State(i32),
        Msg(String),
        Number(f32),
    }

    let x = Cmd::Msg("abc".to_string());
    let e = rmp_serde::to_vec(&x).unwrap();
    let y: Cmd = rmp_serde::from_slice(&e).unwrap();
    println!("{:?} --> 0x{:x?} --> {:?}", x, e, y);

}
