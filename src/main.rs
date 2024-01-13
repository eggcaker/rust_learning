#![allow(dead_code)]
#![allow(unused_labels, unreachable_code)]

fn main() {
    let mut optional = Some(0);

    // loop {
    //     match optional {
    //         Some(i) => {
    //             if i > 9 {
    //                 println!("Greater than 9, quit!");
    //                 optional = None;
    //             } else {
    //                 println!("`i` is `{:?}`. Try again.", i);
    //                 optional = Some(i + 1);
    //             }
    //         }
    //         _ => {
    //             break;
    //         }
    //     }
    // }

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
