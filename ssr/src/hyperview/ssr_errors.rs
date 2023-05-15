use thiserror::Error;

#[derive(Debug, Error)]
pub enum SsrError {
    #[error("Could not convert provided year and month")]
    YearMonthConversion,

    #[error("Invalid sensor type. Only numeric sensors are supported")]
    NonNumericSensorUsed,

    #[error("Output file already exists.")]
    OutputFileExists,
}
