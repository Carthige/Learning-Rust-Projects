Summary from the second chapter

If you want to use a other/new crate.
You will have to first go to the Cargo.toml file and type below dependencies:
Crate_name = "*"

Then type: cargo build

This will download a newest version of this crate. 
After this update look into the Cargo.lock file to see the correct version number. Fill this correct version number in into the Cargo.toml file. 

later on you allways can use the command:
Cargo update to update all your dependencies to the latest version.

You can look at the crate documentation, which contains all its methods, functions and traits of the create. The command is:
cargo doc --open

Next to new cargo commands, we learned that a function always returns a value (num), which is "Ok" if the returned value is good, or en Err object which contains the type of error. 

Match statements are constructed by taking a variable and comparing it to all elements in the tree structure and executing the lines of code for which a match was found. 