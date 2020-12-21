use std::path::{Path, PathBuf};
const DEFAULT: &str = "lib";

fn asref_example<T: AsRef<str>>(s: T) {
    assert_eq!("sample", s.as_ref());
}

fn asmut<T: AsMut<u8>>(s: &mut T) {
    *s.as_mut() += 12
}

fn asreference<P: AsRef<Path>, Q: AsRef<Path>>(p: P, q: Option<Q>) -> PathBuf {
    let q: &Path = q.as_ref().map(|x| x.as_ref()).unwrap_or(DEFAULT.as_ref());
    p.as_ref().join(q)
}

fn main() {
    let x = asreference("a", Some("b"));
    let y = asreference("b", Option::<String>::None);

    asref_example("sample");

    let mut bx = Box::new(0);
    asmut(&mut bx);

    assert_eq!(*bx, 12);
    println!("{:?}", x);
    println!("{:?}", y);
}