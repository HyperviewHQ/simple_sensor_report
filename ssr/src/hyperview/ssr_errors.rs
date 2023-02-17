use thiserror::Error;

#[derive(Debug, Error)]
pub enum SsrError {
    #[error("Could not convert provided year and month")]
    YearMonthConversionError,

    #[error("Invalid sensor type. Only numeric sensors are supported")]
    NonNumericSensorUsedError,

    #[error("Output file already exists.")]
    OutputFileExistsError,
}
