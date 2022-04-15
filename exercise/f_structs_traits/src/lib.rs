pub trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
pub struct Grapes {
    pub amount_left: i32
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(t: &mut T, num_bites: i32) {
    for _i in 1..=num_bites {
        t.bite();
    }
}