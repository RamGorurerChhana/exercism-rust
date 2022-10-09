use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // use the trait time::ext::NumericalDuration to build
    // a Duration instance of billion seconds
    // 1_000_000_000.seconds().
    // PrimitiveDateTime already implements Add trait for Duration
    // so we can add PrimitiveDateTime and Duration with + operator
    // -----------------------------------------------------------
    // From doc.rs/time
    // // region: trait impls
    // impl Add<Duration> for PrimitiveDateTime {
    //     type Output = Self;

    //     fn add(self, duration: Duration) -> Self::Output {
    //         self.checked_add(duration)
    //             .expect("resulting value is out of range")
    //     }
    // }

    start + 1_000_000_000.seconds()
}
