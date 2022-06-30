use std::time::Instant;

#[allow(dead_code)]
pub fn mesure_performance<T>(name: &str, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    let end = Instant::now();
    println!("{} took {:?}", name, end.duration_since(start));

    result
}
