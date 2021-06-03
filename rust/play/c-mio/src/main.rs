// Trying some stuff seen in tokio..  io/driver/mod.rs
fn main() {
    let token = mio::Token(1 << 31);
    println!("{:#b}", token.0);

    let address = least_significant(24);

    println!("{:#b}", address.mask);
    println!("{:#b}", address.then(7).mask);
    println!("{:#b}", address.then(7).shift);
}

const fn mask_for(n: u32) -> usize {
    let shift = 1usize.wrapping_shl(n - 1);
    shift | (shift - 1)
}

const fn least_significant(width: u32) -> Pack {
    let mask = mask_for(width);

    Pack { mask, shift: 0 }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pack {
    mask: usize,
    shift: u32,
}

const fn pointer_width() -> u32 {
    std::mem::size_of::<usize>() as u32 * 8
}

impl Pack {
    const fn then(&self, width: u32) -> Pack {
        let shift = pointer_width() - self.mask.leading_zeros();
        let mask = mask_for(width) << shift;

        Pack { mask, shift }
    }
}
