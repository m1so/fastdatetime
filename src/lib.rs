use chrono::format::{parse, Parsed, StrftimeItems};

use pyo3::{once_cell::GILOnceCell, prelude::*};
use time_fmt::parse::{parse_date_time_maybe_with_zone, TimeZoneSpecifier};
use time_tz::{Offset, TimeZone};

mod datetime_utils;
mod interop;

static DEFAULT_PARSER: GILOnceCell<dtparse::Parser> = GILOnceCell::new();

#[pyfunction(date_string, "/", "*", dayfirst = "false", yearfirst = "false")]
#[pyo3(name = "parse")]
fn parse_from_py(
    py: Python<'_>,
    date_string: &str,
    dayfirst: Option<bool>,
    yearfirst: Option<bool>,
) -> PyResult<interop::DateTimeWrapper> {
    let (datetime, _offset, _tokens) = DEFAULT_PARSER
        .get_or_init(py, || dtparse::Parser::default())
        .parse(
            date_string,
            dayfirst,
            yearfirst,
            false,
            false,
            None,
            false,
            &std::collections::HashMap::new(),
        )
        .map_err(|parse_error| pyo3::exceptions::PyValueError::new_err(parse_error.to_string()))?;

    Ok(interop::DateTimeWrapper::NaiveDateTime(datetime))
}

#[pyfunction]
#[pyo3(name = "strptime")]
fn strptime_from_py_chrono(
    date_string: &str,
    mut format: String,
) -> PyResult<interop::DateTimeWrapper> {
    // accounting for format difference between Python and Chrono results in ~20% perf hit
    // let chrono_format = format.replace(".%f", "%.f");

    let rewrite_idx = format.rfind(".%");
    if let Some(index) = rewrite_idx {
        unsafe {
            let format_bytes = format.as_bytes_mut();
            let p_dot: *mut u8 = &mut format_bytes[index];
            let p_percent: *mut u8 = &mut format_bytes[index + 1];
            std::ptr::swap(p_dot, p_percent);
        }
    }

    let mut parsed = Parsed::new();
    let items = StrftimeItems::new(&format);

    // try to parse the string, otherwise raise ValueError
    parse(&mut parsed, date_string, items)
        .map_err(|parse_error| pyo3::exceptions::PyValueError::new_err(parse_error.to_string()))?;

    match parsed.to_datetime() {
        // all good, convert to "UTC aligned" datetime
        Ok(datetime) => Ok(interop::DateTimeWrapper::NaiveDateTime(
            datetime.naive_utc(),
        )),
        // there was an error, let's try to convert to a partial datetime by setting defaults
        Err(_parse_error) => {
            let _ = parsed.set_nanosecond(0);
            let _ = parsed.set_second(0);
            let _ = parsed.set_minute(0);
            let _ = parsed.set_hour(0);
            let _ = parsed.set_day(1);
            let _ = parsed.set_month(1);
            let _ = parsed.set_year(1900);

            let date = parsed.to_naive_date().map_err(|parse_error| {
                pyo3::exceptions::PyValueError::new_err(format!(
                    "Unable to parse date due to {}",
                    parse_error
                ))
            })?;

            match parsed.to_naive_time() {
                Ok(time) => Ok(interop::DateTimeWrapper::NaiveDateTime(date.and_time(time))),
                Err(_parse_error) => Ok(interop::DateTimeWrapper::NaiveDate(date)),
            }
        }
    }
}

#[pyfunction]
#[pyo3(name = "strptime")]
fn strptime_from_py_time_rs(date_string: &str, format: &str) -> PyResult<interop::DateTimeWrapper> {
    // try to parse the string, otherwise raise ValueError
    let (naive_dt, tz_spec) = parse_date_time_maybe_with_zone(format, date_string)
        .map_err(|parse_error| pyo3::exceptions::PyValueError::new_err(parse_error.to_string()))?;

    let adjusted_dt = match tz_spec {
        Some(TimeZoneSpecifier::Offset(utc_offset)) => {
            datetime_utils::align_to_utc(naive_dt, utc_offset)
        }
        Some(TimeZoneSpecifier::Name(name)) => time_tz::timezones::get_by_name(name).map_or_else(
            || {
                Err(pyo3::exceptions::PyValueError::new_err(
                    "Invalid timezone: ".to_string() + name,
                ))
            },
            |tz| {
                Ok(datetime_utils::align_to_utc(
                    naive_dt,
                    tz.get_offset_utc(&time::OffsetDateTime::now_utc()).to_utc(),
                ))
            },
        )?,
        None => naive_dt,
    };

    Ok(interop::DateTimeWrapper::PrimitiveDateTime(adjusted_dt))
}

#[pymodule]
fn fastdatetime(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;

    let chrono_module = PyModule::new(py, "chrono")?;
    chrono_module.add_function(wrap_pyfunction!(strptime_from_py_chrono, chrono_module)?)?;
    module.add_submodule(chrono_module)?;

    module.add_function(wrap_pyfunction!(parse_from_py, module)?)?;
    module.add_function(wrap_pyfunction!(strptime_from_py_time_rs, module)?)?;

    Ok(())
}
