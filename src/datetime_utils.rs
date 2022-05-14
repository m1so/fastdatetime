/// Cascade an out-of-bounds value.
#[macro_export]
macro_rules! cascade {
    (@ordinal ordinal) => {};
    (@year year) => {};

    // Cascade an out-of-bounds value from "from" to "to".
    ($from:ident in $min:literal.. $max:literal => $to:tt) => {
        #[allow(unused_comparisons, unused_assignments)]
        if $from >= $max {
            $from -= $max - $min;
            $to += 1;
        } else if $from < $min {
            $from += $max - $min;
            $to -= 1;
        }
    };

    // Special case the ordinal-to-year cascade, as it has different behavior.
    ($ordinal:ident => $year:ident) => {
        // We need to actually capture the idents. Without this, macro hygiene causes errors.
        cascade!(@ordinal $ordinal);
        cascade!(@year $year);
        #[allow(unused_assignments)]
        if $ordinal > time::util::days_in_year($year) {
            $year += 1;
            $ordinal = 1;
        } else if $ordinal == 0 {
            $year -= 1;
            $ordinal = time::util::days_in_year($year);
        }
    };
}

pub(crate) use cascade;

pub(crate) fn align_to_utc(
    datetime: time::PrimitiveDateTime,
    utc_offset: time::UtcOffset,
) -> time::PrimitiveDateTime {
    let mut second = datetime.second() as i8 - utc_offset.seconds_past_minute();
    let mut minute = datetime.minute() as i8 - utc_offset.minutes_past_hour();
    let mut hour = datetime.hour() as i8 - utc_offset.whole_hours();
    let (mut year, mut ordinal) = datetime.date().to_ordinal_date();

    crate::datetime_utils::cascade!(second in 0..60 => minute);
    crate::datetime_utils::cascade!(minute in 0..60 => hour);
    crate::datetime_utils::cascade!(hour in 0..24 => ordinal);
    crate::datetime_utils::cascade!(ordinal => year);

    time::PrimitiveDateTime::new(
        time::Date::__from_ordinal_date_unchecked(year, ordinal),
        time::Time::__from_hms_nanos_unchecked(
            hour as _,
            minute as _,
            second as _,
            datetime.nanosecond(),
        ),
    )
}
