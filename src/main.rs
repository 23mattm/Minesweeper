
struct Space {
    dug: bool,
    mine: bool
}

impl Space {

    fn new() -> Self {
        return Space{dug: false, mine: false}
    }

    fn dig(mut self) {
        if !self.dug { //mine has not yet been dug
            if self.mine{
                //you lose
            } else {
                self.dug = true;
            }
        }
    }
}




fn main() {
    println!("Hello, world!");
}
