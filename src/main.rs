use std::collections::HashSet;

pub mod fsm;

fn main() {
    let S_1 = "S1";
    let S_2 = "S2";
    let S_3 = "S3";
    let jumps = vec![
        fsm::Jump {
            state_name: S_1,
            input: "q12",
            reaction: fsm::Reaction {
                output: "a12",
                target_state_name: S_2,
            },
        },
        fsm::Jump {
            state_name: S_2,
            input: "q22",
            reaction: fsm::Reaction {
                output: "a22",
                target_state_name: S_2,
            },
        },
        fsm::Jump {
            state_name: S_2,
            input: "q23",
            reaction: fsm::Reaction {
                output: "a23",
                target_state_name: S_3,
            },
        },
        fsm::Jump {
            state_name: S_3,
            input: "q31",
            reaction: fsm::Reaction {
                output: "a31",
                target_state_name: S_1,
            },
        },
    ];
    let mut sm = fsm::Fsm::create(&S_1, &HashSet::from([S_1, S_2, S_3]), &jumps).unwrap();
    let mut s = sm.get_state();
    println!("{s}");

    let mut o = sm.input("q12").unwrap_or("err");
    println!("{o}");

    s = sm.get_state();
    println!("{s}");

    o = sm.input("q22").unwrap_or("err");
    println!("{o}");
    o = sm.input("q23").unwrap_or("err");
    println!("{o}");
    o = sm.input("q31").unwrap_or("err");
    println!("{o}");
}
