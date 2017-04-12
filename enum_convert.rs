#[derive(Debug)]
enum CanConvert {
    Vec1(Vec<String>),
    Vec2(Vec<String>),
    NotVec(i32),
}

fn main() {
    let v1 = CanConvert::Vec1(["one", "two", "three", "uno", "dos", "tres"]
                                  .iter()
                                  .map(|&s| s.into())
                                  .collect());
    let v2 = match v1 {
        CanConvert::Vec1(underlying) |
        CanConvert::Vec2(underlying) => CanConvert::Vec2(underlying),
        _ => panic!("v1 did not have appropriate underlying type!"),
    };
    // let v3 = v1 as CanConvert::Vec2;

    println!("{:#?}", v2);
    // println!("{:#?}", v3);
}
