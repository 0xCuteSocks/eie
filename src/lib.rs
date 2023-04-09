pub mod calibration;
pub mod estimator_executor;
pub mod intensity_estimator;
pub mod intensity_info;

#[cfg(test)]
mod tests {
    use crate::{
        calibration::aksolver_factory::{AkSolverFactory, SolverType},
        intensity_estimator::IntensityEstimator,
        intensity_info::IntensityInfo,
    };
    use serde::Deserialize;
    use std::error::Error;

    #[derive(Debug, Deserialize)]
    struct Record {
        bid: f64,
        ask: f64,
        ts: u64,
    }

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let spread_step = 0.00001;
        let n_steps = 5;
        let w = 1000 * 60 * 10; // sliding window 30 min
        let dt = 1000 * 15; // time scaling 15 sec

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("./test/tick.csv")
            .unwrap();

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_path("./serial_estimations.csv")
            .unwrap();

        let solver_type = SolverType::MultiCurve;

        let sf = AkSolverFactory::new(&solver_type);
        let mut ie = IntensityEstimator::new(spread_step, n_steps, w, dt, sf);

        for result in rdr.deserialize() {
            let td: Record = result?;

            if ie.on_tick(td.bid, td.ask, td.ts) {
                let intensity_info: IntensityInfo = ie.estimate(td.ts);

                let res = intensity_info.get_ak();

                let _ = wtr.write_record(&[
                    res.0.to_string(),
                    res.1.to_string(),
                    res.2.to_string(),
                    res.3.to_string(),
                ]);
            }
            wtr.flush()?;
        }
        Ok(())
    }
}
