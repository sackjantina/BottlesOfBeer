use std::io;
use std::time;
use std::thread;
use rand::Rng;

trait Lyrics {
    fn lines(&self, cap:bool) -> Self;
    fn print(&self, increasing:bool, step:u32) -> Self;
    fn screen(&self) -> Self;
    fn mid(&self) -> Self;
    fn end(&self);
}

impl Lyrics for u32 {
    fn lines(&self, cap:bool) -> u32 {
        match *self {
            0 => print!("{}o more lines of text", if cap {"N"} else {"n"}),
            1 => print!("{} line of text", self),
            _ => print!("{} lines of text", self)
        }
        *self
    }

    fn print(&self, increasing:bool, step:u32) -> u32 {
        match *self {
            0 => { print!("Go to the store and buy some more, "); 99 }
            _ => { print!("Print it out, stand up and shout, "); 
		    if increasing {*self + step} else {*self - step} }
        }
    }

    fn screen(&self) -> u32 {
        print!(" on the screen");
        *self
    }

    fn mid(&self) -> u32 {
        print!(", ");
        *self
    }

    fn end(&self) {
        println!(".");
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let step:u32 = rng.gen_range(1..=10);
    let increasing = true;
    //let max_lines = 500;

    let mut buffer = String::new();
    println!("How many lines?");
    let stdin = io::stdin();
    stdin.read_line(&mut buffer);
    let max_lines:u32 = buffer.trim().parse().unwrap();

    let second = time::Duration::new(1,0);

    if increasing {
        for i in (1..max_lines).step_by(step.try_into().unwrap()) {
            i.lines(true).screen().mid().lines(false).end();
            i.print(increasing, step).lines(false).screen().end();
            println!();
            thread::sleep(second);
        }
    } else {
        for i in (0..max_lines).step_by(step.try_into().unwrap()).rev() {
            i.lines(true).screen().mid().lines(false).end();
            i.print(increasing, step).lines(false).screen().end();
            println!();
            thread::sleep(second);
        }
    }
}
