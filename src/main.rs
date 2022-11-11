use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
}

struct Core {
    mem: Vec<i32>,
    p: u64,
}

impl Core {
    fn new() -> Self {
        Core {
            mem: Vec::new(),
            p: 0,
        }
    }
}

enum Command {
    Ao,
    HuTao,
    XiangLing,
    NingGuang,
    Keqing,
    Yelan,
    Shogun,
    Ayaka,
    Yoimiya,
    Miko,
    Barbara,
    Klee,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "ao" => Command::Ao,
            "hutao" => Command::HuTao,
            "xiangling" => Command::XiangLing,
            "ningguang" => Command::NingGuang,
            "keqing" => Command::Keqing,
            "yelan" => Command::Yelan,
            "shogun" => Command::Shogun,
            "ayaka" => Command::Ayaka,
            "yoimiya" => Command::Yoimiya,
            "miko" => Command::Miko,
            "barbara" => Command::Barbara,
            "klee" => Command::Klee,
            _ => unreachable!("invalid command"),
        }
    }
}
