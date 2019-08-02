// use std::process::exit;
use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Machine {
    buf: Vec<u8>,
    stack: Vec<u16>,
    pos: usize,
    reg: [u16; 8]
}

impl Machine {
    fn load() -> Machine {
        let mut m = Machine { pos: 0, buf: Vec::new(), reg: [0; 8], stack: Vec::new() };
        let mut f = File::open("challenge.bin").unwrap();
        f.read_to_end(&mut m.buf).unwrap();
        m
    }

    fn next_val(&mut self) -> u16 {
        let rslt : u16 = self.buf[self.pos] as u16 +
                    256 * self.buf[self.pos + 1] as u16;
        self.pos += 2;

        if rslt > 32767 {
            self.reg[(rslt - 32768) as usize]
        } else {
            rslt
        }
    }

    fn next_reg(&mut self) -> usize {
        let rslt = self.buf[self.pos] as usize +
                    256 * self.buf[self.pos + 1] as usize;
        self.pos += 2;

        assert!(rslt >= 32768 && rslt <= 32775);

        rslt - 32768
    }

    fn read_mem(&self, ptr: u16) -> u16 {
        let ind : usize = ptr as usize * 2;

        self.buf[ind] as u16 | ((self.buf[ind + 1] as u16) << 8)
    }

    fn write_mem(&mut self, ptr: u16, v: u16) {
        if ptr >= 32768 {
            self.reg[(ptr - 32768) as usize] = v;
        } else {
            let ind : usize = ptr as usize * 2;
            self.buf[ind] = (v & 0xff) as u8;
            self.buf[ind + 1] = ((v >> 8) & 0xff) as u8;
        }
    }

    fn jump(&mut self, a: u16) {
        self.pos = 2 * a as usize;
    }

    fn set_reg(&mut self, r: usize, v: u16) {
        self.reg[r] = v;
    }

    fn stack_push(&mut self, v: u16) {
        self.stack.push(v);
    }

    fn stack_pop(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }

    fn call(&mut self, a: u16) {
        let p = self.pos as u16;
        self.stack_push(p / 2);
        self.jump(a);
    }

    fn ret(&mut self) {
        let rloc = self.stack_pop();
        self.jump(rloc);
    }
}

fn main() -> io::Result<()> {
    let mut m = Machine::load();

    print!("total size {}\n", m.buf.len());

    loop {
        let op = m.next_val();
        match op {
            0 => { print!("<<HALT>>\n"); break; }
            1 => { let reg = m.next_reg(); let val = m.next_val(); m.set_reg(reg, val); },
            2 => { let val = m.next_val(); m.stack_push(val); },
            3 => { let reg = m.next_reg(); let val = m.stack_pop(); m.set_reg(reg, val); },
            4 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, match v1 == v2 { false => 0, true => 1 }); },
            5 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, match v1 > v2 { false => 0, true => 1 }); },
            6 => { let goto = m.next_val(); m.jump(goto); },
            7 => { let val = m.next_val(); let goto = m.next_val(); if val != 0 { m.jump(goto); } },
            8 => { let val = m.next_val(); let goto = m.next_val(); if val == 0 { m.jump(goto); } },
            9 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, v1.wrapping_add(v2) % 32768); }
            10 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, v1.wrapping_mul(v2) % 32768); }
            11 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, v1.wrapping_rem(v2) % 32768); }
            12 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, v1 & v2); }
            13 => { let reg = m.next_reg(); let v1 = m.next_val(); let v2 = m.next_val(); m.set_reg(reg, v1 | v2); }
            14 => { let reg = m.next_reg(); let v = m.next_val(); m.set_reg(reg, (!v) & 0x7fff); }
            15 => { let reg = m.next_reg(); let ptr = m.next_val(); let val = m.read_mem(ptr); m.set_reg(reg, val); },
            16 => { let ptr = m.next_val(); let val = m.next_val(); m.write_mem(ptr, val); },
            17 => { let gosub = m.next_val(); m.call(gosub); },
            18 => { m.ret(); },
            19 => { print!("{}", m.next_val() as u8 as char); },
            21 => {},
            _ => { print!("unknown op {}\n", op); break; }
        }
    }

    Ok(())
}
