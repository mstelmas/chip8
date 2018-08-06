use std::borrow::Cow;
use std::str::{self, FromStr};
use nom::{IResult, space, eol, digit};

#[derive(Debug, Clone, Copy)]
pub enum Commands {
    Cpu,
    Disasm(u16),
    Mem(u16),
    Start,
    Step, // TODO: step size!
    Stop
}

impl FromStr for Commands {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command: IResult<&[u8], Commands, u32> = command(s.trim().as_bytes());

        match command {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into())
        }
    }
}

// TODO: make it more sophisticated
// TODO: upgrade to nom 4.0.0
named!(
    command<Commands>,
    alt!(cpu | mem | start | step | stop | disasm)
);

named!(
    cpu<Commands>,
    map!(
        tag!("cpu"),
        |_| Commands::Cpu)
);

named!(
    step<Commands>,
    map!(
        tag!("step"),
        |_| Commands::Step)
);

named!(
    stop<Commands>,
    map!(
        tag!("stop"),
        |_| Commands::Stop)
);

named!(
    start<Commands>,
    map!(
        tag!("start"),
        |_| Commands::Start)
);

// TODO: allow hexadecimal digits
named!(
    disasm<Commands>,
    chain!(
        tag!("disasm") ~
        addr: preceded!(space, addr_parser),
        || Commands::Disasm(addr)
    )
);

// TODO: allow hexadecimal digits
named!(
    mem<Commands>,
    chain!(
        tag!("mem") ~
        addr: preceded!(space, addr_parser),
        || Commands::Mem(addr)
    )
);

named!(
    addr_parser<u16>,
    map_res!(
        map_res!(digit, str::from_utf8),FromStr::from_str));
