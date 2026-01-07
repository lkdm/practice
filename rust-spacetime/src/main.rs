use hifitime::prelude::*;
use thiserror::Error;

// 1 Mars hour = 1/24 sol is 61.649 Earth minutes
// Mars shift =  8.22 Earth hours
//
// 1 sol = 24 Mars hours
// 1 Mars hour = 61m 39s Earth time
// 1 Mars minute = 61.55 Earth seconds
//
// Mars colony established 2211-11-110:00 MTC
// This moment defines Sol 0.
// - BFE: Before Founding Epoch (negative sols)
// - FE: Founding Epoch (positive sols)

pub struct MartianDateTime {
    pub year: i32,
    pub sol: u16,
    pub hour: u8,
    pub minute: u8,
    pub second: f64,
}

// Approximate number of sols per Martian year
const SOLS_PER_YEAR: u16 = 668;
const PERIODS_PER_YEAR: u16 = 24;
const BASE_SOLS_PER_PERIOD: u16 = 27;
const EXTRA_SOLS: u16 = SOLS_PER_YEAR - PERIODS_PER_YEAR as u16 * BASE_SOLS_PER_PERIOD;
const CUMULATIVE_SOLS: [u16; PERIODS_PER_YEAR as usize] = {
    let mut arr = [0u16; PERIODS_PER_YEAR as usize];
    let mut sum = 0;
    let mut i = 0;
    while i < PERIODS_PER_YEAR as usize {
        let period = (i + 1) as u8;
        arr[i] = sum;
        sum += sols_in_period_const(period);
        i += 1;
    }
    arr
};
// Base sols per period: 27.8333
// Most periods have 27 sols
// Every third period has 28 sols

#[derive(Debug, Error, Copy, Clone)]
#[error("period must be between 1-24 (given: {0})")]
pub struct InvalidPeriod(u8);

/// Determines the number of sols in a given period
///
/// Every third period has an extra sol
pub const fn sols_in_period_const(period: u8) -> u16 {
    if period == 0 || period > 24 {
        panic!("period must be 1..=24");
    }
    if period % 3 == 0 && period <= 24 {
        BASE_SOLS_PER_PERIOD + 1
    } else {
        BASE_SOLS_PER_PERIOD
    }
}

impl MartianDateTime {
    pub fn period(&self) -> u8 {
        match CUMULATIVE_SOLS.binary_search(&self.sol) {
            Ok(idx) => idx as u8 + 1, // exactly at the start of a period
            Err(idx) => idx as u8,    // sol falls before CUMULATIVE_SOLS[idx], so period = idx
        }
    }

    pub fn sol_in_period(&self) -> u16 {
        let period = self.period() as usize;
        self.sol - CUMULATIVE_SOLS[period - 1]
    }
}

fn main() {
    let utc = Epoch::now().unwrap();
    let tdb = utc.to_time_scale(TimeScale::TDB);

    let offset = tdb.to_tai_seconds() - utc.to_tai_seconds();

    println!("UTC: {}", utc);
    println!("TDB: {}", tdb);
    println!("TDB - UTC coordinate offset = {:.9} s", offset);

    // for period in 1..=24 as u8 {
    //     let p = sols_in_period(period);
    //     println!("Period: {} has {} sols", period, p)
    // }

    let mut mars = MartianDateTime {
        year: 0,
        sol: 0,
        hour: 0,
        minute: 0,
        second: 0f64,
    };

    for sol in 1..SOLS_PER_YEAR as u16 {
        mars.sol = sol;
        let p = mars.period();
        print!("{:?},", p);
    }
}
