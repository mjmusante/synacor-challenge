// use std::process::exit;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

struct Machine {
    buf: Vec<u8>,
    pos: usize,
}

impl Machine {
    fn load() -> Machine {
        let mut m = Machine { pos: 0, buf: Vec::new() };
        let mut f = File::open("challenge.bin").unwrap();
        f.read_to_end(&mut m.buf).unwrap();
        m
    }

    fn next_num(&mut self) -> u16 {
        let rslt : u16 = self.buf[self.pos] as u16 +
                    256 * self.buf[self.pos + 1] as u16;
        self.pos += 2;

        rslt        
    }

    fn next_val(&mut self) -> String {
        let rslt = self.next_num();

        if rslt > 32767 {
            format!("r{}", rslt - 32768)
        } else {
            format!("{}", rslt)
        }
    }

    fn next_reg(&mut self) -> String {
        let s = self.next_val();
        assert!(s.starts_with("r"));
        s
    }
}

fn main() -> io::Result<()> {
    let mut m = Machine::load();
    let argv : Vec<String> = env::args().collect();

    let startpc : usize;

    if argv.len() > 1 {
        startpc = argv[1].parse::<usize>().unwrap();
    } else {
        startpc = 0;
    }

    m.pos = startpc * 2;

    loop {
        let pc = m.pos / 2;
        let op = m.next_num();
        match op {
            0 => { print!("{} halt\n", pc); }
            1 => { let a = m.next_reg(); let b = m.next_val(); print!("{} set {} {}\n", pc, a, b); },
            2 => { let val = m.next_val(); print!("{} push {}\n", pc, val); },
            3 => { let reg = m.next_reg(); print!("{} pop {}\n", pc, reg); },
            4 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} eq {} {} {}\n", pc, reg, v1, v2); },
            5 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} gt {} {} {}\n", pc, reg, v1, v2); },
            6 => { let goto = m.next_val(); print!("{} jmp {}\n", pc, goto); },
            7 => { let val = m.next_val(); let goto = m.next_val(); print!("{} jt {} {}\n", pc, val, goto); },
            8 => { let val = m.next_val(); let goto = m.next_val(); print!("{} jf {} {}\n", pc, val, goto); },
            9 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} add {} {} {}\n", pc, reg, v1, v2); }
            10 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} mult {} {} {}\n", pc, reg, v1, v2); }
            11 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} mod {} {} {}\n", pc, reg, v1, v2); }
            12 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} and {} {} {}\n", pc, reg, v1, v2); }
            13 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); print!("{} or {} {} {}\n", pc, reg, v1, v2); }
            14 => { let reg = m.next_reg(); let v = m.next_val();print!("{} not {} {}\n", pc, reg, v); }
            15 => { let reg = m.next_reg(); let ptr = m.next_val(); print!("{} rmem {} {}\n", pc, reg, ptr); },
            16 => { let ptr = m.next_val(); let val = m.next_val(); print!("{} wmem {} {}\n", pc, ptr, val); },
            17 => { let gosub = m.next_val(); print!("{} call {}\n", pc, gosub); },
            18 => { print!("{} ret\n", pc); },
            19 => { let c = m.next_val(); print!("{} out {}\n", pc, c); },
            20 => { let reg = m.next_reg(); print!("{} in {}\n", pc, reg); },
            21 => { print!("{} nop\n", pc); },
            _ => { print!("unknown op {}\n", op); break; }
        }
    }

    Ok(())
}
