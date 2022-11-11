use std::{
    env,
    fs::{self},
    io::Read,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse_args(&args).unwrap();
    let path = Path::new(&path);

    let content = read_file(path);

    let commands = parse_commands(content);

    let mut core = Core::new();

    core.run(commands).unwrap();
}

fn parse_args(args: &Vec<String>) -> Result<String> {
    if args.len() < 3 {
        Err("no enough args")
    } else {
        let mode = &args[1];
        if mode != "file" {
            Err("invalid execute mode, should be file")
        } else {
            Ok(args[2].clone())
        }
    }
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("could not open file")
}

fn parse_commands(content: String) -> Vec<Command> {
    let mut cmd: Vec<&str> = content.split(" ").collect();

    cmd.drain(..).map(Command::from).collect()
}

fn read_char() -> Result<MemValue> {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|res| res.ok())
        .map(|v| v as MemValue)
        .ok_or("failed to read a char")
}

fn read_int() -> Result<MemValue> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| "not a valid string")?;
    input.trim().parse().map_err(|_| "not a valid integer")
}

type MemValue = i32;

struct Core {
    mem: Vec<MemValue>,
    mem_p: usize,
    cmd_p: usize,
    register: Option<MemValue>,
    commands: Vec<Command>,
}

impl Core {
    fn new() -> Self {
        Core {
            mem: Vec::new(),
            mem_p: 0,
            register: None,
            cmd_p: 0,
            commands: Vec::new(),
        }
    }

    fn run(&mut self, commands: Vec<Command>) -> Result<()> {
        self.cmd_p = 0;
        self.commands = commands;

        while self.cmd_p < self.commands.len() {
            let cmd = self.commands[self.cmd_p].clone();
            self.handle_cmd(&cmd)?;
        }

        Ok(())
    }

    fn handle_cmd(&mut self, cmd: &Command) -> Result<()> {
        match cmd {
            Command::Ao => {
                if self.cmd_p == 0 {
                    return Ok(());
                }

                self.cmd_p -= 1;

                let mut level = 1;

                while level > 0 {
                    if self.cmd_p == 0 {
                        break;
                    }

                    self.cmd_p -= 1;

                    let cmd = &self.commands[self.cmd_p];
                    if let Command::Ao = cmd {
                        level += 1;
                    }
                    if let Command::Ayaka = cmd {
                        level -= 1;
                    }
                }

                if level != 0 {
                    return Ok(());
                }

                let v = self.mem[self.cmd_p];
                let cmd = Command::from(&v);
                return self.handle_cmd(&cmd);
            }
            Command::HuTao => {
                if self.mem_p == 0 {
                    return Ok(());
                }
                self.backward_p();
            }
            Command::XiangLing => {
                self.forward_p();
            }
            Command::NingGuang => {
                let cmd = self.get_cmd()?;
                if cmd == Command::NingGuang {
                    return Err("infinite command 3");
                }
                self.handle_cmd(&cmd)?;
            }
            Command::KeQing => {
                let v = self.get_value()?;
                if v == &0 {
                    let ch = read_char()?;
                    self.set_value(ch);
                } else {
                    print!("{}", char::from(*v as u8));
                }
            }
            Command::YeLan => {
                self.decrease();
            }
            Command::Shogun => self.increase(),
            Command::Ayaka => {
                if self.get_value()? == &0 {
                    let mut level = 1;

                    self.cmd_p += 1;

                    if self.cmd_p != self.commands.len() {
                        while level > 0 {
                            let prev = &self.commands[self.cmd_p];

                            self.cmd_p += 1;

                            if self.cmd_p == self.commands.len() {
                                break;
                            }

                            let cmd = &self.commands[self.cmd_p];

                            if let Command::Ayaka = cmd {
                                level += 1;
                            }

                            if let Command::Ao = cmd {
                                level -= 1;
                                if let Command::Ayaka = prev {
                                    level -= 1;
                                }
                            }
                        }
                        if level != 0 {
                            return Ok(());
                        }
                    }
                }
            }
            Command::Yoimiya => {
                self.set_value(0);
            }
            Command::Miko => {
                if let Some(v) = self.get_reg() {
                    self.set_value(v);
                    self.clear_reg();
                } else {
                    let v = *self.get_value()?;
                    self.set_reg(v);
                }
            }
            Command::Barbara => {
                print!("{}", self.get_value()?);
            }
            Command::Klee => {
                let v = read_int()?;
                self.set_value(v);
            }
        }
        self.cmd_p += 1;
        Ok(())
    }

    fn forward_p(&mut self) {
        self.mem_p += 1;
        if self.mem_p >= self.mem.len() {
            self.mem.push(0);
        }
    }

    fn backward_p(&mut self) {
        self.mem_p -= 1;
    }

    fn get_value(&self) -> Result<&i32> {
        self.mem.get(self.mem_p).ok_or("value not found")
    }

    fn get_cmd(&self) -> Result<Command> {
        let v = self.get_value()?;
        Ok(Command::from(v))
    }

    fn set_value(&mut self, v: MemValue) {
        self.mem[self.mem_p] = v;
    }

    fn decrease(&mut self) {
        self.mem[self.mem_p] -= 1;
    }

    fn increase(&mut self) {
        self.mem[self.mem_p] += 1;
    }

    fn get_reg(&self) -> Option<i32> {
        self.register
    }

    fn set_reg(&mut self, v: MemValue) {
        self.register = Some(v);
    }

    fn clear_reg(&mut self) {
        self.register = None;
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Command {
    Ao = 0,
    HuTao = 1,
    XiangLing = 2,
    NingGuang = 3,
    KeQing = 4,
    YeLan = 5,
    Shogun = 6,
    Ayaka = 7,
    Yoimiya = 8,
    Miko = 9,
    Barbara = 10,
    Klee = 11,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "ao" => Command::Ao,
            "hutao" => Command::HuTao,
            "xiangling" => Command::XiangLing,
            "ningguang" => Command::NingGuang,
            "keqing" => Command::KeQing,
            "yelan" => Command::YeLan,
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

impl From<&MemValue> for Command {
    fn from(v: &MemValue) -> Self {
        match v {
            0 => Command::Ao,
            1 => Command::HuTao,
            2 => Command::XiangLing,
            3 => Command::NingGuang,
            4 => Command::KeQing,
            5 => Command::YeLan,
            6 => Command::Shogun,
            7 => Command::Ayaka,
            8 => Command::Yoimiya,
            9 => Command::Miko,
            10 => Command::Barbara,
            11 => Command::Klee,
            _other => unreachable!("{_other} is not a valid command code"),
        }
    }
}

type Result<T> = std::result::Result<T, &'static str>;
