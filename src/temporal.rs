/// The number of days in year.
pub const DAYS_IN_YEAR: f64 = 365.25;

pub enum AgeInDays {
    Gestational { week: u8, day: u8 },
    Postnatal { days: u16 },
}
