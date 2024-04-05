# blackbox-rs

blackbox-rs is a Rust crate that is to be paired with [BlackBox](https://github.com/BlackboxMC/Blackbox), which allows you to create Bukkit compatible plugins using compiled langauges such as Rust.

It is currently a work in progress.

# Contributing

Due to the massive scale of Spigot's API, BlackBox is automatically generated by the `generate.py` script. There is a system for hand written code to be added via the `additions` folder, but currently no (and probably never) support for patches. I understand the script is hard to work with, [and I apologize](https://github.com/BlackBoxMC/blackbox-rs/issues/10), but I think that, given the girgantuan scale of the API and the fact that new features are added every few months, hand-writing it is simply not feasible.

**`generate.py` uses a file that isn't in the repo.** You have to generate it via [this repo](https://github.com/BlackBoxMC/SpigotJSON) and then move the resulting .json file into this one, where it is gitignored. This is because the file as of writing this is extremely large and would make the repo even slower to clone if I included in.
