// run-pass

const HASH_LEN: usize = 20;
struct Hash([u8; HASH_LEN]);
fn init_hash(_: &mut [u8; HASH_LEN]) {}

fn foo<'a>() -> &'a () {
    Hash([0; HASH_LEN]);
    init_hash(&mut [0; HASH_LEN]);
    &()
}

fn main() {
    foo();
}
