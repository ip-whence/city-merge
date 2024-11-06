# IP Location Database Merger

This program merges the IP location data from two datasets:

- [dbip-city-ipv4-num.csv.7z](https://cdn.jsdelivr.net/npm/@ip-location-db/dbip-city-7z/dbip-city-ipv4-num.csv.7z)
- [geolite2-city-ipv4-num.csv.7z](https://cdn.jsdelivr.net/npm/@ip-location-db/geolite2-city-7z/geolite2-city-ipv4-num.csv.7z)

The merged output includes all original data from `dbip-city-ipv4-num.csv` along with the **time zone** data from `geolite2-city-ipv4-num.csv`.

The files were noted from [sapics/ip-location-db](https://github.com/sapics/ip-location-db/).

---

## Usage

1. Download both files and extract:
   - [`dbip-city-ipv4-num.csv.7z`](https://cdn.jsdelivr.net/npm/@ip-location-db/dbip-city-7z/dbip-city-ipv4-num.csv.7z)
   - [`geolite2-city-ipv4-num.csv.7z`](https://cdn.jsdelivr.net/npm/@ip-location-db/geolite2-city-7z/geolite2-city-ipv4-num.csv.7z)

2. Run the merging program to generate a new version of `dbip-city-ipv4-num.csv` with the added time zone data.

3. The output file will be in the same format as `dbip-city-ipv4-num.csv` but with two additional columns for the time zone.

---
