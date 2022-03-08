use num::Float;
use num_traits::FromPrimitive;
use vek::ColorComponent;

pub(crate) fn gamma_rgb_to_hsl<T: Float + FromPrimitive + ColorComponent>(
    rgb: vek::Rgb<T>,
) -> vek::Vec3<T> {
    let cmax = rgb.r.max(rgb.g).max(rgb.b);
    let cmin = rgb.r.min(rgb.g).min(rgb.b);
    let delta = cmax - cmin;
    let h = if delta == T::from_f64(0.0).unwrap() {
        T::from_f64(0.0).unwrap()
    } else if cmax == rgb.r {
        T::from_f64(60.0).unwrap() * ((rgb.g - rgb.b) / delta % T::from_f64(6.0).unwrap())
    } else if cmax == rgb.g {
        T::from_f64(60.0).unwrap() * ((rgb.b - rgb.r) / delta + T::from_f64(2.0).unwrap())
    } else if cmax == rgb.b {
        T::from_f64(60.0).unwrap() * ((rgb.r - rgb.g) / delta + T::from_f64(4.0).unwrap())
    } else {
        T::from_f64(0.0).unwrap()
    };
    let h = (h + T::from_f64(360.0).unwrap()) % T::from_f64(360.0).unwrap();
    let l = (cmax + cmin) / T::from_f64(2.0).unwrap();
    let s: T;
    if delta == T::from_f64(0.0).unwrap() {
        s = T::from_f64(0.0).unwrap();
    } else {
        s = delta
            / (T::from_f64(1.0).unwrap()
                - (T::from_f64(2.0).unwrap() * l - T::from_f64(1.0).unwrap()).abs());
    }
    return vek::Vec3::new(h, s, l);
}

pub(crate) fn linear_rgb_to_hsl<T: Float + FromPrimitive + ColorComponent>(
    rgb: vek::Rgb<T>,
) -> vek::Vec3<T> {
    return gamma_rgb_to_hsl(vek::Rgb::new(
        rgb.r.powf(T::from_f64(1.0 / 2.2).unwrap()),
        rgb.g.powf(T::from_f64(1.0 / 2.2).unwrap()),
        rgb.b.powf(T::from_f64(1.0 / 2.2).unwrap()),
    ));
}

pub(crate) fn gamma_rgb_to_hsv<T: Float + FromPrimitive + ColorComponent>(
    rgb: vek::Rgb<T>,
) -> vek::Vec3<T> {
    let cmax = rgb.r.max(rgb.g).max(rgb.b);
    let cmin = rgb.r.min(rgb.g).min(rgb.b);
    let delta = cmax - cmin;
    let h = if delta == T::from_f64(0.0).unwrap() {
        T::from_f64(0.0).unwrap()
    } else if cmax == rgb.r {
        T::from_f64(60.0).unwrap() * ((rgb.g - rgb.b) / delta % T::from_f64(6.0).unwrap())
    } else if cmax == rgb.g {
        T::from_f64(60.0).unwrap() * ((rgb.b - rgb.r) / delta + T::from_f64(2.0).unwrap())
    } else if cmax == rgb.b {
        T::from_f64(60.0).unwrap() * ((rgb.r - rgb.g) / delta + T::from_f64(4.0).unwrap())
    } else {
        T::from_f64(0.0).unwrap()
    };
    let h = (h + T::from_f64(360.0).unwrap()) % T::from_f64(360.0).unwrap();
    let s = if cmax == T::from_f64(0.0).unwrap() {
        T::from_f64(0.0).unwrap()
    } else {
        delta / cmax
    };
    let v = cmax;
    return vek::Vec3::new(h, s, v);
}

pub(crate) fn linear_rgb_to_hsv<T: Float + FromPrimitive + ColorComponent>(
    rgb: vek::Rgb<T>,
) -> vek::Vec3<T> {
    return gamma_rgb_to_hsv(vek::Rgb::new(
        rgb.r.powf(T::from_f64(1.0 / 2.2).unwrap()),
        rgb.g.powf(T::from_f64(1.0 / 2.2).unwrap()),
        rgb.b.powf(T::from_f64(1.0 / 2.2).unwrap()),
    ));
}

pub(crate) fn hsv_to_gamma_rgb<T: Float + FromPrimitive + ColorComponent>(
    hsv: vek::Vec3<T>,
) -> vek::Rgb<T> {
    let c = hsv.z * hsv.y;
    let x = c
        * (T::one()
            - ((hsv.x / T::from_f64(60.0).unwrap()) % T::from_f64(2.0).unwrap() - T::one()).abs());
    let m = hsv.z - c;
    let h = (hsv.x + T::from_f64(360.0).unwrap()) % T::from_f64(360.0).unwrap();
    if h < T::from_f64(60.0).unwrap() {
        return vek::Rgb::new(c + m, x + m, m);
    } else if h < T::from_f64(120.0).unwrap() {
        return vek::Rgb::new(x + m, c + m, m);
    } else if h < T::from_f64(180.0).unwrap() {
        return vek::Rgb::new(m, c + m, x + m);
    } else if h < T::from_f64(240.0).unwrap() {
        return vek::Rgb::new(m, x + m, c + m);
    } else if h < T::from_f64(300.0).unwrap() {
        return vek::Rgb::new(x + m, m, c + m);
    } else {
        return vek::Rgb::new(c + m, m, x + m);
    }
}

pub(crate) fn hsv_to_linear_rgb<T: Float + FromPrimitive + ColorComponent>(
    hsv: vek::Vec3<T>,
) -> vek::Rgb<T> {
    let gamma_rgb = hsv_to_gamma_rgb(vek::Vec3::new(hsv.x, hsv.y, hsv.z));
    return vek::Rgb::new(
        gamma_rgb.r.powf(T::from_f64(2.2).unwrap()),
        gamma_rgb.g.powf(T::from_f64(2.2).unwrap()),
        gamma_rgb.b.powf(T::from_f64(2.2).unwrap()),
    );
}
