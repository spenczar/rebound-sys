#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_create_simulation() {
        unsafe {
            let sim = reb_simulation_create();
            assert!(!sim.is_null());
            reb_simulation_free(sim);
        }
    }

    #[test]
    fn test_integrate_particle() {
        unsafe {
            let sim = reb_simulation_create();
            let fmt_str = CString::new("M a").unwrap();
            let particle = reb_particle_from_fmt(sim, fmt_str.as_ptr(), 1, 1);
            reb_simulation_add(sim, particle);

            reb_simulation_integrate(sim, 1.0);

            reb_simulation_free(sim);
        }
    }
}
