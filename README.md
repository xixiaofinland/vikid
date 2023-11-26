# Vikid Cli

I want to know what TVs in viki.com have finnish subtitles, as well as their `douban id` and `douban rating`.

This is a Cli to retrieve these info from api endpoints of viki and wmda (which has doban info).

# Fetched data

In case you only need the data, here is collected info (updated: 11.2023):
- [csv from viki and wmda(with douban info)](./viki.csv)
- [csv from viki only](./viki_wmda.csv)

In case you want to install vikid and fetch data by yourself, read further:

# Install

`cargo install vikid`

# How to use it

Run `vikid` without any parameter will fetch info both from viki and wmda in
two steps.

1. Fetch data from viki api. this step is fast as viki api has no restriction
2. After step 1, fetch douban data from wmda api. This step is slow as wmda has
   30 second interval restriction. So we can fetch up to one TV series data
   every 30 seconds.

You can also execute against either step with corresponding parameters defined
below:

```
Usage: vikid [OPTIONS]

Options:
  -v, --viki     Retrieve only basic info from viki
  -w, --wmda     Assume viki csv was created, retrieve only data from wmda (i.e. douban id, douban rating)
  -h, --help     Print help
  -V, --version  Print version
```

# A node version

There is also a node verison of this tool. It only fetch from viki without wmda feature. 
[here](https://github.com/xixiaofinland/viki-videos-with-finnish-subtitle)
