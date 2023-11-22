# Vikid Cli

I want to know what TV in viki.com have finnish subtitles, but I trust more the rating from douban.com.

Therefore, this is a Cli to retrieve Finnish language subtitled TV Series info from viki (and wmda).

# Install

`cargo install vikid`

# How to use it

Run `vikid` without any parameter will fetch info from viki and wmda in
sequence, then save the result into the corresponding csv files.

```
Usage: vikid [OPTIONS]

Options:
  -d, --douban   Retrieve only douban info(id and rating) from wmda
  -v, --viki     Retrieve only basic info from viki
  -h, --help     Print help
  -V, --version  Print version
```

Here is collected info on 11.2023,
- [csv from viki](./result.csv)
- [csv from wmda](./result2.csv)

# A node version

There is also a node verison of this tool. It only fetch from viki without wmda(douban). 
[here](https://github.com/xixiaofinland/viki-videos-with-finnish-subtitle)
