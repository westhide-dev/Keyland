macro_rules! nil {
    () => {
        ()
    };
}

pub type Nil = nil!();

pub const NIL: Nil = nil!();
