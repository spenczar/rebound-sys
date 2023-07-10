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
            let sim = reb_create_simulation();
            assert!(!sim.is_null());
            reb_free_simulation(sim);
        }
    }

    #[test]
    fn test_integrate_particle() {
        unsafe {
            let sim = reb_create_simulation();
            let fmt_str = CString::new("M a").unwrap();
            let particle = reb_particle_new(sim, fmt_str.as_ptr(), 1, 1);
            reb_add(sim, particle);

            reb_integrate(sim, 1.0);

            reb_free_simulation(sim);
        }
    }
}
