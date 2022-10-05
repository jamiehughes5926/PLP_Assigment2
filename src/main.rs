struct Date {
    days: i32
}

impl Date {

    // create Date from year/month/day triple
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        return Date { days: 0 };
    }

    // convert back to year/month/day triple
    fn ymd(&self) -> (i32, i32, i32) {
        return (0, 0, 0);
    }

}

fn main() {
    // testing from_ymd; should print Date { days: 738885 }
    println!("{:?}", Date::from_ymd(2022, 12, 31));

    // testing Add and Display
    let date = Date::from_ymd(-2, 12, 31);
    // increase date by 30 days, 60 days, etc.
    for i in 0..20 {
        // first iteration should print 2/12/31 BC
        println!("{}", date + i * 30);
    }
}