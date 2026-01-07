use pyo3::prelude::*;
use std::f32::consts::PI;

#[pyfunction]
#[pyo3(signature = (left, right, pos = 0.0))]
pub fn pan2(left: f32, right: f32, pos: f32) -> (f32, f32) {
    let half_pi = PI / 2.0;
    let mut res_left: f32 = 0.0;
    let mut res_right: f32 = 0.0;

    if pos < 0.0 {
        res_left += left;
        res_left += right * (pos.abs() * half_pi).sin();
        res_right += right * (pos.abs() * half_pi).cos();
    } else {
        res_right += right;
        res_left += left * (pos * half_pi).cos();
        res_right += left * (pos * half_pi).sin();
    }

    (res_left, res_right)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32, eps: f32) {
        assert!((a - b).abs() <= eps, "not approx equal: a={a}, b={b}",);
    }

    #[test]
    fn pan2_center_is_balanced() {
        let (l, r) = pan2(1.0, 1.0, 0.0);
        approx(l, 1.0, 1e-6);
        approx(r, 1.0, 1e-6);
    }

    #[test]
    fn pan2_full_left() {
        let (l, r) = pan2(1.0, 1.0, -1.0);
        approx(l, 1.0, 1e-6);
        approx(r, 0.0, 1e-6);
    }

    #[test]
    fn pan2_full_right() {
        let (l, r) = pan2(1.0, 1.0, 1.0);
        approx(l, 0.0, 1e-6);
        approx(r, 1.0, 1e-6);
    }
}
