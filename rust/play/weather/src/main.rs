use std::collections::HashMap;

use chrono::{Date, DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use colorgrad::{Color, CustomGradient};
use crossterm::style::{self, Stylize};
use esbat::{Phase::*, PrincipalPhase};
use structopt::StructOpt;
use sunrise::sunrise_sunset;
use url::Url;

use wthr::{
    client::ApiClient,
    config::{Base, Config, Opt, Params, Resolution},
    error::Result,
    APP, NWS_API,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let config = Config::load(Base::FromEnv).unwrap_or_default();
    let opt = Opt::from_args();
    let params = Params::from_merge(&config, &opt)?;

    let mut client = ApiClient::builder()
        .base_url(NWS_API)
        .api_key(APP, &params.api_key)
        .build()?;

    // Find the weather station gridpoint for the location
    let point = client
        .get_point(vec![params.latitude, params.longitude])
        .await?;

    let forecast = client
        .get_forecast_from_url(match params.resolution {
            Resolution::Hourly => &point.properties.forecast_hourly,
            Resolution::Daily => &point.properties.forecast,
        })
        .await?;

    let max_wind_column_len = forecast
        .properties
        .periods
        .iter()
        .map(|x| x.wind_speed.len())
        .max()
        .unwrap_or(0);

    let max_time_column_len = forecast
        .properties
        .periods
        .iter()
        .map(|x| {
            DateTime::<Utc>::from_utc(x.start_time.naive_utc(), Utc)
                .format("%A %l%P")
                .to_string()
                .len()
        })
        .max()
        .unwrap_or(0);

    let temp_grad = CustomGradient::new()
        .colors(&[
            Color::from_rgb_u8(255, 255, 255),
            Color::from_rgb_u8(66, 66, 255),
            Color::from_rgb_u8(66, 255, 66),
            Color::from_rgb_u8(255, 130, 66),
        ])
        .domain(&[0., 32., 72., 84.])
        .build()?;

    fn sun(lat: f64, lon: f64, date: Date<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
        let (rise, set) = sunrise_sunset(lat, lon, date.year(), date.month(), date.day());
        (Utc.timestamp(rise, 0), Utc.timestamp(set, 0))
    }

    // Display time
    println!(
        "{}",
        Local::now()
            .format("%A, %B %-e, %-l:%M %P, %Y %Z")
            .to_string()
            .blue()
    );

    // Display location info
    println!(
        "{}",
        format!("Weather for {}, {}", point.city(), point.state()).blue()
    );

    // Display sun times
    let (rise, set) = sun(params.latitude, params.longitude, Utc::now().date());
    let rise = Local.from_utc_datetime(&rise.naive_utc());
    let set = Local.from_utc_datetime(&set.naive_utc());
    println!(
        "{} {}, {} {}",
        "Sunrise:".blue(),
        rise.format("%-I:%M %P"),
        "Sunset:".blue(),
        set.format("%-I:%M %P")
    );

    // Display moon phase
    //let emoji = esbat::daily_lunar_phase(Utc::now().date()).as_emoji();
    let moon = match esbat::daily_lunar_phase(Utc::now().date()) {
        NewMoon => ("New Moon", '\u{1F311}'),
        WaxingCrescent => ("Waxing Crescent", '\u{1F312}'),
        FirstQuarter => ("First Quarter", '\u{1F313}'),
        WaxingGibbous => ("Waxing Gibbous", '\u{1F314}'),
        FullMoon => ("Full Moon", '\u{1F315}'),
        WaningGibbous => ("Waning Gibbous", '\u{1F316}'),
        LastQuarter => ("Last Quarter", '\u{1F317}'),
        WaningCrescent => ("Waning Crescent", '\u{1F318}'),
    };
    //println!("\u{1F311}\u{1F312}\u{1F313}\u{1F314}\u{1F315}\u{1F316}");
    //println!("\u{1F317}\u{1F318}\u{1F31D}\u{1F31A}\u{1F30C}\u{1F31E}\u{1F31F}\u{1F320}");
    println!("{} {} {}", "Moon Phase:".blue(), moon.0, moon.1);

    // Display next full moon time
    let moon_iter = esbat::lunar_phase_iter(Utc::now()..);
    let next_full = moon_iter
        .filter_map(|x| {
            if x.0 == PrincipalPhase::FullMoon {
                Some(x.1)
            } else {
                None
            }
        })
        .take(1)
        .next();

    if let Some(full_time) = next_full {
        println!(
            "{} {}",
            "Next Full Moon:".blue(),
            Local
                .from_utc_datetime(&full_time.naive_utc())
                .format("%A, %B %-e, %-l:%M %P, %Y")
        );
    }

    let tz = point.properties.time_zone;

    // Store sunrise/sunset times for each new date
    let mut sun_times: HashMap<Date<Tz>, (DateTime<Tz>, DateTime<Tz>)> = HashMap::new();

    // Store last description to avoid printing redundant lines
    let mut last_desc = String::new();

    // Display hourly or daily forecast
    for period in forecast.properties.periods {
        let time = tz
            .from_local_datetime(&period.start_time.naive_local())
            .unwrap();
        //println!("{:?}", local_time);
        //let time = DateTime::from_utc(period.start_time.naive_utc(), Utc);
        let date = time.date();

        let (sunrise, sunset) = sun_times.entry(date).or_insert({
            let (rise, set) = sunrise_sunset(
                params.latitude,
                params.longitude,
                date.year(),
                date.month(),
                date.day(),
            );
            (tz.timestamp(rise, 0), tz.timestamp(set, 0))
        });

        // Format time for display
        //println!("{:?} {:?} {:?}", time, sunrise, sunset);
        let time_color = if (time > *sunrise && time < *sunset)
            || time.hour() == sunrise.hour()
            || time.hour() == sunset.hour()
        {
            // Daytime
            style::Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            }
        } else {
            // Nighttime
            style::Color::Rgb {
                r: 100,
                g: 0,
                b: 255,
            }
        };
        let local_time = Local.from_utc_datetime(&time.naive_utc());
        let time = format!(
            "{0: >1$}",
            local_time.format("%A %l%P").to_string(),
            max_time_column_len
        );

        // Format temperature for display
        let (r, g, b, _a) = temp_grad.at(period.temperature as f64).rgba_u8();
        let temp_color = style::Color::Rgb { r, g, b };
        let temp = format!("{}°{}", period.temperature, period.temperature_unit);

        // Don't repeat the same description in multiple lines
        let desc = match period.short_forecast == last_desc {
            true => "",
            false => {
                last_desc = period.short_forecast.clone();
                &last_desc
            }
        };

        // Weather icon
        let icon_url = Url::parse(&period.icon).unwrap();
        let mut icon = icon_url.path_segments().unwrap().last().unwrap();
        let mut pct = String::new();
        if icon.contains(',') {
            let (ic, pc) = icon.split_once(',').unwrap();
            icon = ic;
            pct = format!("{}%", pc);
        }
        //println!("{} {:?}", icon, pct);
        let icon_str = match icon {
            "few" => "\u{1f31e}",
            "sct" => "\u{1f324}",
            "tsra" => "\u{1f326} \u{1f329}",
            "tsra_hi" => "\u{1f324} \u{1f329}",
            "tsra_sct" => "\u{1f325} \u{1f329}",
            "rain_showers" => "\u{1f327}",
            "bkn" => "\u{1f325}",
            _ => "\u{1f32e}", // taco
        };

        println!(
            "{0} {1} {2: >3$} {4: <2} {5} {6} {7}",
            time.with(time_color),
            temp.with(temp_color),
            period.wind_speed,
            max_wind_column_len,
            period.wind_direction,
            icon_str,
            pct,
            desc
        );
    }

    Ok(())
}
