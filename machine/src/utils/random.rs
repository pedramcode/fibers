use rand::RngCore;

pub fn random_fiber_id(rng: &mut Box<rand::prelude::ThreadRng>) -> u64 {
    rng.next_u64()
}