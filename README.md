# rstatus
A program for write computer status in X root window's name.
It is intended to work with `dwm`, but should work with any WM or program that uses the X root window's name in any way.

## Usage
This program should be runned in the background only one time, preferably when starting X server. It uses the output of the scripts you declare in `src/config.rs` in the same exact order that they appear.

You might either compile the program with `cargo` and drop the executable in your path or utilize the `execute` bash script which does the same (you may as well change the destination folder or add steps to this script) and put the following in your `.xprofile`, in you `.xinitrc` or any file where you put the auto-start programs.
````
rstatus &
````

### Scripts

The definitions in `src/config.rs` are in a tuple which contains:

 1. A string slice with the path to the script to be executed, defining scripts that are in the `$PATH` is allowed and recommended. 
 2. The interval in secs for updating the script. A time of 0 means no update, and although negative numbers are allowed this is a bug and will be removed in future versions.

#### Path
It's possible to define scripts that take multiple arguments as long as the only spaces in the string are between arguments and not in them. E.g. 
````
("xblacklight -get", 30)    // ✔✔
("echo \"Get out home!\"", 0) // ❌❌
````
 


### Modify
It was designed having in mind that you should modify the source code in order to change the behavior of the software (like suckless programs do). So, any change requires you to recompile and "install" the program. The main software logic is in `src/lib.rs`.
