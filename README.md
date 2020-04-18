# What is this
Some data analysis and filtering of COVID-19 data from https://github.com/CSSEGISandData/COVID-19 repository.
It outputs to the console a filtered view of configurable watched countries, and a json file with the same data.

# How to use
- clone this repo
- clone CSSEGIS's repository to get the same data  
The repos should be on the same root folder. If you want another directory path, you can change the `get_data_files` function to the correct paths. 
Or even better, implement a directory selection.
- Configure countries you want to see in the settings_data/watchlist.csv file  
It is in a "name", "target" format, so the same countires that is called with a different name can be merged together
ie `Mainland China, China` will group entries as `Mainland China` with `China`
- `cargo run --release`  
Or you can `cargo build --release` and then run the generated file separately if you want.
- That's it  
It generates a `all_data.json` file with all your watched countries for further analysis.


