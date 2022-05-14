use chrono::prelude::*;

use pyo3::prelude::*;
use pyo3::types::PyDateTime;

pub enum DateTimeWrapper {
    NaiveDate(chrono::NaiveDate),
    NaiveDateTime(chrono::NaiveDateTime),
    PrimitiveDateTime(time::PrimitiveDateTime),
}

impl ToPyObject for DateTimeWrapper {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            DateTimeWrapper::NaiveDate(date) => PyDateTime::new(
                py,
                date.year(),
                date.month() as u8,
                date.day() as u8,
                0,
                0,
                0,
                0,
                None,
            ),
            DateTimeWrapper::NaiveDateTime(datetime) => PyDateTime::new(
                py,
                datetime.year(),
                datetime.month() as u8,
                datetime.day() as u8,
                datetime.hour() as u8,
                datetime.minute() as u8,
                datetime.second() as u8,
                datetime.nanosecond() / 1000u32,
                None,
            ),
            DateTimeWrapper::PrimitiveDateTime(datetime) => PyDateTime::new(
                py,
                datetime.year(),
                datetime.month() as u8,
                datetime.day(),
                datetime.hour(),
                datetime.minute(),
                datetime.second(),
                datetime.microsecond(),
                None,
            ),
        }
        .expect("Failed to construct datetime")
        .into()
    }
}

impl IntoPy<PyObject> for DateTimeWrapper {
    fn into_py(self, py: Python) -> PyObject {
        ToPyObject::to_object(&self, py)
    }
}
