use uuid::Uuid;
use ulid::Ulid;
use snowflake::SnowflakeIdGenerator;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_id_generator() {
        let iterations = 1_000_000;

        // Benchmark UUID v4
        let start = Instant::now();
        for _ in 0..iterations {
            Uuid::new_v4();
        }
        let uuid_duration = start.elapsed();
        println!("UUID v4: {:?}", uuid_duration);

        // Benchmark ULID
        let start = Instant::now();
        for _ in 0..iterations {
            Ulid::new();
        }
        let ulid_duration = start.elapsed();
        println!("ULID: {:?}", ulid_duration);

        // Benchmark Snowflake
        let mut sf = SnowflakeIdGenerator::new(1, 1);
        let start = Instant::now();
        for _ in 0..iterations {
            sf.generate();
        }
        let snowflake_duration = start.elapsed();
        println!("Snowflake: {:?}", snowflake_duration);
    }
}
