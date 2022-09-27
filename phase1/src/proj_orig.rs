trait Lyrics {
    fn bottles(&self, bool) -> Self;
    fn take(&self) -> Self;
    fn wall(&self) -> Self;
    fn mid(&self) -> Self;
    fn end(&self);
}

impl Lyrics for u32 {
    fn bottles(&self, cap:bool) -> u32 {
        match *self {
            0 => print!("{}o more bottles of beer", if cap {"N"} else {"n"}),
            1 => print!("{} bottle of beer", self),
            _ => print!("{} bottles of beer", self)
        }
        *self
    }

    fn take(&self) -> u32 {
        match *self {
            0 => { print!("Go to the store and buy some more, "); 99 }
            _ => { print!("Take one down and pass it around, "); *self - 1 }
        }
    }

    fn wall(&self) -> u32 {
        print!(" on the wall");
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
    for i in (0..100).rev() {
        i.bottles(true).wall().mid().bottles(false).end();
        i.take().bottles(false).wall().end();
        println!();
    }
}
