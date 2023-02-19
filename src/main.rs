mod enum_command_line;

use enum_command_line::EnumCommandLine;

#[derive(Clone)]
pub struct T {
    arg1: String,
}
impl T {
    pub const CMD: &'static str = "cmd";
    pub fn run(&self) -> Result<(), ()> {
        println!("cmd1 with arg: {}", self.arg1);
        Ok(())
    }
    pub fn new_cmd(params: Vec<String>) -> Result<Self, ()> {
        Ok(Self {
            arg1: params[0].clone(),
        })
    }
}

#[derive(Clone)]
pub struct Y {
    arg1: String,
    arg2: String,
}
impl Y {
    pub const CMD: &'static str = "cmd2";
    pub fn run(&self) -> Result<(), ()> {
        println!("cmd2 with args: {}, {}", self.arg1, self.arg2);
        Ok(())
    }
    pub fn new_cmd(params: Vec<String>) -> Result<Self, ()> {
        Ok(Self {
            arg1: params[0].clone(),
            arg2: params[1].clone(),
        })
    }
}

#[derive(Clone, EnumCommandLine)]
enum Test {
    T(T),
    Y(Y),
}

fn main() {
    Test::command_line("cmd_sys_test> ")
}
