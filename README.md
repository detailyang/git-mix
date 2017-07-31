# git-mix
[![Build Status](https://travis-ci.org/detailyang/git-mix.svg?branch=master)](https://travis-ci.org/detailyang/git-mix)
[![](https://crates.io/crates/git-mix)](https://crates.io/crates/git-mix)

git-mix is inspired by [git-crypt](https://github.com/AGWA/git-crypt) but implemented by rust.

# How
Based on [Git Attributes](https://git-scm.com/book/en/v2/Customizing-Git-Git-Attributes). Using the “clean” and “smudge” filters
, we can can set a filter for particular paths before they’re checked out and staged as the following:


# Install
```bash
cargo install git-mix
```

# Use

1. edit the .gitattributes to set the path you want to mix as the following:

```bsah
private/* filter=git-mix
```

2. run `git-mix gen` to generate the config for defining the filter `git-mix`

```bash
[filter = "git-mix"]
    clean = git-mix encrypt --key BiqdSyKwmnIFDKg1LzXIg5eEM3RWbdUb
    smudge = git-mix decrypt --key BiqdSyKwmnIFDKg1LzXIg5eEM3RWbdUb
```

3. append template to  .git/config

4. remember the key, or you can reset the key which is required 32 bytes 
 

Contributing
------------

To contribute to git-mix, clone this repo locally and commit your code on a separate branch.

PS: PR Welcome :rocket: :rocket: :rocket: :rocket:


Author
------

> GitHub [@detailyang](https://github.com/detailyang)


License
-------
git-mix is licensed under the [MIT] license.

[MIT]: https://github.com/detailyang/ybw/blob/master/licenses/MIT
