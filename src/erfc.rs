pub struct Erfc {
    pub coeffs: Vec<f32>,
}

impl Erfc {
    fn generate_erfc_coeffs(&mut self, degree: usize) {
        // let mut coeffs = vec![0.0; degree + 1];

        self.coeffs[0] = 1.0;

        let inv_sqrt_pi = 1.0 / std::f32::consts::PI.sqrt();

        // c_(2n+1) = -2 (-1)^n / (sqrt(pi) n! (2n+1))
        let mut factorial = 1.0;

        for n in 0.. {
            let power = 2 * n + 1;

            if power > degree {
                break;
            }

            if n > 0 {
                factorial *= n as f32;
            }

            let sign = if n % 2 == 0 { -1.0 } else { 1.0 };

            self.coeffs[power] = sign * 2.0 * inv_sqrt_pi / (factorial * power as f32);
        }
    }

    pub fn new() -> Self {
        let coeffs: Vec<f32> = vec![0.; 30];
        let mut v = Erfc { coeffs };
        v.generate_erfc_coeffs(30);
        v
    }

    pub fn eval(&self, x: f32) -> f32 {
        if x > 2.3 {
            return 0.;
        }
        if x < -2.3 {
            return 2.;
        }
        let mut result = 0.0;

        // Horner's method
        for &coeff in self.coeffs.iter().rev() {
            result = result * x + coeff;
        }

        result
    }
}
