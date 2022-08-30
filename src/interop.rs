use chrono::prelude::*;

use pyo3::prelude::*;
use pyo3::types::PyDateTime;

pub(crate) trait TryIntoPy<T>: Sized {
    fn try_into_py(self, py: Python) -> PyResult<T>;
}

impl TryIntoPy<PyObject> for chrono::NaiveDate {
    fn try_into_py(self, py: Python) -> PyResult<PyObject> {
        PyDateTime::new(
            py,
            self.year(),
            self.month() as u8,
            self.day() as u8,
            0,
            0,
            0,
            0,
            None,
        )
        .map(|dt| dt.into_py(py))
    }
}

impl TryIntoPy<PyObject> for chrono::NaiveDateTime {
    fn try_into_py(self, py: Python) -> PyResult<PyObject> {
        PyDateTime::new(
            py,
            self.year(),
            self.month() as u8,
            self.day() as u8,
            self.hour() as u8,
            self.minute() as u8,
            self.second() as u8,
            self.nanosecond() / 1000u32,
            None,
        )
        .map(|dt| dt.into_py(py))
    }
}

impl TryIntoPy<PyObject> for time::PrimitiveDateTime {
    fn try_into_py(self, py: Python) -> PyResult<PyObject> {
        PyDateTime::new(
            py,
            self.year(),
            self.month() as u8,
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.microsecond(),
            None,
        )
        .map(|dt| dt.into_py(py))
    }
}
