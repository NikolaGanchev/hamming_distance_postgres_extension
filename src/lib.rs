use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern]
fn hamming_distance(x: &[u8], y: &[u8]) -> i32 {
    return i32::try_from(hamming::distance(&x, &y)).unwrap_or_else(|_| -1);
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    use crate::hamming_distance;

    #[pg_test]
    fn test_hamming_distance_extension() {
        assert_eq!(3, hamming_distance(&32_i32.to_ne_bytes(), &45_i32.to_ne_bytes()));
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
