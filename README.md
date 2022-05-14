<div align="center">

# fastdatetime

Like `datetime`, but fast 🚀


[Implementation](#implementation) •
[Installation](#installation) •
[FAQ](#faq) •
[Acknowledgements](#acknowledgements)

</div>

## Implementation

### `strptime`
In the pursuit of a faster `strptime` implementation, two backing [Rust crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) were chosen due to their popularity in the ecosystem – [`chrono`](https://github.com/chronotope/chrono) and [`time`](https://github.com/time-rs/time).

Both impementations are exposed under the following methods:
1. `fastdatetime.strptime` (based on [time](https://github.com/time-rs/time) along with [time-fmt](https://github.com/misawa/time-fmt))
2. `fastdatetime.chrono.strptime` (based on [chrono](https://github.com/chronotope/chrono))

Initially, `chrono` seemed liked a drop-in replacement, however due to subtle differences of `%f` specifier the format argument needs to preprocessed to conform to the [Python referece](https://docs.python.org/3/library/datetime.html#strftime-and-strptime-format-codes) which incurs slight performance overhead as the format string needs to be scanned.
Furthermore, even without this shortcoming, the parsing was slower compared to `time`.

What's also interesting is that `time` doesn't support `strptime` compatible inferface out of the box – fortunatelly [time-fmt](https://github.com/misawa/time-fmt) does (in addition, some fixes and new functionality was upstreamed to the crate). 


### `parse`
Oftentimes the date format is not known ahead of time, or it is not possible to infer it from a few samples. In Python, one would usually opt for using the `dateutil` package, which can deal with all sorts of edge cases and is very forgiving, however these guarantees come at a cost – in order for the results to not be ambiguous but accurate, the user can tweak a lot of options (is the day or year first, ...) and has to sacrifice performance. 

Enter [`dtparse` by Bradlee Speice](https://speice.io/2018/06/dateutil-parser-to-rust.html), a rewrite of `dateutil.parse` in Rust. Going full circle, `fastdatetime` just exposes Python bindings to Bradlee's excellent [crate](https://crates.io/crates/dtparse). This yields ~15x faster parsing performance (657 vs 42 Kops/sec on M1 Pro). 

## Installation
TBD

## FAQ
TBD

## Acknowledgements
* [time](https://github.com/time-rs/time) and [time-fmt](https://github.com/misawa/time-fmt) for the default `strptime` implementation
* [chrono](https://github.com/chronotope/chrono) as an alternative implementation parse `strptime` formats ([specifiers reference](https://docs.rs/chrono/latest/chrono/format/strftime/index.html))
* [dtparse](https://github.com/bspeice/dtparse) ([dateutil](https://github.com/dateutil/dateutil) implemented in Rust)

* [pyo3](https://github.com/PyO3/pyo3) for Python bindings
* [maturin](https://github.com/PyO3/maturin) for building Python wheels