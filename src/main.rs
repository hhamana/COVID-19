use std::{
    // error::Error,
    collections::HashMap,
    path::{Path, PathBuf},
    fs
};
use csv;
use serde::Deserialize;

type HashData = HashMap<String, CountryData>;

#[derive(Debug, Deserialize)]
struct RowData {
    #[serde(alias = "Province_State", alias = "Province/State")]
    province: Option<String>,
    #[serde(alias = "Country_Region", alias = "Country/Region")]
    country: String,
    #[serde(alias = "Last_Update", alias = "Last Update")]
    updated: String,
    #[serde(rename = "Confirmed")]
    #[serde(deserialize_with = "csv::invalid_option")]
    cases: Option<usize>,
    #[serde(rename = "Deaths")]
    #[serde(deserialize_with = "csv::invalid_option")]
    deaths: Option<usize>,
    #[serde(rename = "Recovered")]
    #[serde(deserialize_with = "csv::invalid_option")]
    recovered: Option<usize>,
    // #[serde(alias = "Lat", alias = "Latitude")]
    // #[serde(deserialize_with = "csv::invalid_option")]
    // latitude: Option<f64>,
    // #[serde(alias = "Long_", alias = "Longitude")]
    // #[serde(deserialize_with = "csv::invalid_option")]
    // longitude: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
struct CountryData {
    cases: usize,
    deaths: usize,
    recovered: usize,
    active: usize,
    percentage: f64,
}

impl CountryData {
    fn new() -> CountryData {
        CountryData { 
            cases: 0, 
            deaths: 0, 
            recovered: 0, 
            active: 0,
            percentage: 0.0
        }
    }

    fn add(&mut self, cases: usize, deaths: usize, recovered: usize) {
        self.cases += cases;
        self.deaths += deaths;
        self.recovered += recovered;
        self.active = self.cases.wrapping_sub(self.recovered + self.deaths);
        self.percentage = (self.active as f64 / self.cases as f64 * 10000f64).round()/100f64;
    }
}

fn load_csv_data(file_path: PathBuf) -> Result<HashData, csv::Error> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut all_data: HashMap<String, CountryData> = HashMap::new();
    for result in rdr.deserialize() {
        let record: RowData = match result {
            Ok(data) => data,
            Err(e) => return Err(csv::Error::from(e))
        };
        // Get existing data for country, or if no country , insert the country with zeroed data
        let country_data = all_data.entry(record.country).or_insert(CountryData::new());

        // add the data to the country
        let cases = match record.cases { Some(v) => v, None =>  0 };
        let deaths = match record.deaths { Some(v) => v, None =>  0 };
        let recovered = match record.recovered { Some(v) => v, None =>  0 };
        country_data.add(cases, deaths, recovered);

    };
    all_data.insert(String::from("Europe"), aggregate_europe(&all_data));
    Ok(all_data)
}

fn aggregate_europe(data : &HashData) -> CountryData {
    let european_countries = vec![
        "Italy", "France","Spain", "Germany","Switzerland", "United Kingdom", "Netherlands", "Norway", "Belgium", "Austria", "Sweden", "Denmark",
        "Czechia", "Portugal", "Greece", "Finland", "Ireland", "Slovenia", "Estonia", "Iceland", "Poland", "Romania", "Luxembourg", "Slovakia", "Armenia", "Serbia", 
        "Bulgaria", "Croatia", "Latvia", "Albania", "Hungary", "Belarus", "Cyprus", "Georgia", "Bosnia and Herzegovina", "Malta", "North Macedonia"
    ];
    let mut europe_count = CountryData::new();
    for country in european_countries {
        match data.get(country) {
            Some(country_data) => {
                    europe_count.add(country_data.cases, country_data.deaths, country_data.recovered);
                }
            None => {
                // println!("Failed getting European country {}", country);
                ()
            }
        } 
    }
    europe_count
}

fn get_data_files() -> std::io::Result<Vec<PathBuf>> {
    // Same folder path, but how to express it depends on the OS fs API.
    #[cfg(target_os = "linux")]
    let folder_path = Path::new("/media/aquarius/月火/AzureusPC/code/Python/Scrapers/COVID-19/csse_covid_19_data/csse_covid_19_daily_reports/");
    #[cfg(target_os = "windows")]
    let folder_path = Path::new("D:\\AzureusPC\\code\\Python\\Scrapers\\COVID-19\\csse_covid_19_data\\csse_covid_19_daily_reports\\");

    // needed to only keep csv
    let csv_type = std::ffi::OsStr::new("csv");

    let folder_iterator = fs::read_dir(folder_path)?;
    let mut files = Vec::new();
    for item in folder_iterator {
        let entry = item?;
        let path = entry.path();
        let extension = path.extension();
        match extension {
            Some(ext) => {
                // Add file to Vec if it's a csv
                if ext == csv_type {
                    files.push(path)
                }
                // Do nothing otherwise
            },
            // Do nothing otherwise
            _ => ()
        }
    };
    Ok(files)
}

fn get_watchlist() -> HashMap<String, String> {
    let watchlist_file = "settings_data/watchlist.csv";
    let mut watchlist_reader = csv::Reader::from_path(watchlist_file).expect("Couldn't load watchlist countries CSV data");
    let mut watchlist = HashMap::new();
    for watchlist_country in watchlist_reader.records() {
        let record = watchlist_country.unwrap();
        let name = record[0].trim().to_owned();
        let target = record[1].trim().to_owned();
        watchlist.insert(name, target);
    }
    watchlist
}

fn get_data_from_file(file_path : PathBuf, watchlist: &HashMap<String, String>) -> Option<HashData> {
    let data = match load_csv_data(file_path) {
        Ok(csv_data) => csv_data,
        Err(err) => {
            println!("{:?}", err);
            return None
        }
    };

    // let data = aggregate_europe(csv_data);

    let mut watched_data: HashData = HashMap::new();
    for (key, target) in watchlist {
        if let Some(cd) = data.get(&key.to_string()) {
            watched_data.insert(target.to_owned(), cd.clone());
        }
    };

    if !watched_data.is_empty() {
        Some(watched_data)
    } else {
        None
    }
}

fn get_data_from_file_paths(files : Vec<PathBuf>, watchlist: HashMap<String, String>) -> Vec<HashData> {
    let mut all_data = Vec::new();
    for file_path in files {
        let hasdata = get_data_from_file(file_path, &watchlist);
        if let Some(hashdata) = hasdata {
            all_data.push(hashdata);
        }
    };
    all_data
}

fn main() {
    println!("COVID-19 Situation in the world");

    println!("Loading watchlist settings...");
    let watchlist = get_watchlist();
    
    println!("Reading directory...");
    let files = get_data_files().unwrap();

    println!("Gathering data...");
    let all_data = get_data_from_file_paths(files, watchlist);
    println!("{} days worth of data gathered", all_data.len());

    for date in all_data {

    }
}