use noise::NoiseFn;

pub fn fibonacci(x: i32) -> i32 {
    if x <= 2 {
        return 1;
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}

pub fn get_noise_texture(w: u32, h: u32, seed: u32) -> Vec<Vec<i32>> {
    let n = noise::Value::new(seed);
    let mut ret = vec![0 as f64; (w * h) as usize];
    for x in 0..h {
        for y in 0..w {
            let val = n.get([x as f64, y as f64]);
            ret[(x + y * w) as usize] = val;
        }
    }
    (0..h).map(|_| Vec::new()).collect()
}

#[test]
fn t() {
    // let val = get_noise_texture(0.0, 0.0, 1);
    // println!("{val}");
}
