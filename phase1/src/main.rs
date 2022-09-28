use std::io;
use std::time;
use std::thread;
use rand::Rng;
use std::fs::File;
use std::io::Write;

trait Lyrics {
    // Functions to print to screen
    fn lines(&self, cap:bool) -> Self;
    fn print(&self, increasing:bool, step:u32) -> Self;
    fn screen(&self) -> Self;
    fn mid(&self) -> Self;
    fn end(&self);

    // Functions to write to file
    fn lines_file(&self, cap:bool, file: &mut File) -> Self;
    fn print_file(&self, increasing:bool, step:u32, file: &mut File) -> Self;
    fn screen_file(&self, file: &mut File) -> Self;
    fn mid_file(&self, file: &mut File) -> Self;
    fn end_file(&self, file: &mut File);
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

    fn lines_file(&self, cap:bool, file: &mut File) -> u32 {
        match *self {
            0 => write!(file, "{}o more lines of text", if cap {"N"} else {"n"}),
            1 => write!(file, "{} line of text", self),
            _ => write!(file, "{} lines of text", self)
        };
        *self
    }

    fn print_file(&self, increasing:bool, step:u32, file: &mut File) -> u32 {
        match *self {
            0 => { write!(file, "Go to the store and buy some more, "); 99 }
            _ => { write!(file, "Print it out, stand up and shout, "); 
                if increasing {*self + step} else {*self - step} }
        }
    }

    fn screen_file(&self, file: &mut File) -> u32 {
        write!(file, " in the file");
        *self
    }

    fn mid_file(&self, file: &mut File) -> u32 {
        write!(file, ", ");
        *self
    }

    fn end_file(&self, file: &mut File) {
        writeln!(file, ".");
    }
}

fn print_to_screen() {

}

fn write_to_file(i:u32, file: &mut File, step: u32) {
    i.lines_file(true, file).screen_file(file).mid_file(file).lines_file(false, file).end_file(file);
    i.print_file(true, step, file).lines_file(false, file).screen_file(file).end_file(file);
    writeln!(file);
}

fn main() {
    let mut rng = rand::thread_rng();
    let step:u32 = rng.gen_range(1..=10);
    let increasing = true;
    //let max_lines = 500;

    let mut buffer = String::new();
    println!("How many lines?");
    io::stdin().read_line(&mut buffer).expect("error reading input");
    let max_lines:u32 = buffer.trim().parse().unwrap();

    let mut fileout = File::create("out.txt").expect("error creating file");

    let second = time::Duration::new(1,0);

    if increasing {
        for i in (1..max_lines).step_by(step.try_into().unwrap()) {
            i.lines(true).screen().mid().lines(false).end();
            i.print(increasing, step).lines(false).screen().end();
            println!();
            write_to_file(i, &mut fileout, step);
            thread::sleep(second);
        }
    } else {
        for i in (0..max_lines).step_by(step.try_into().unwrap()).rev() {
            i.lines(true).screen().mid().lines(false).end();
            i.print(increasing, step).lines(false).screen().end();
            println!();
            write_to_file(i, &mut fileout, step);
            thread::sleep(second);
        }
    }
}
