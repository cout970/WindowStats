# Window stats

This program checks every second the current active window and counts how many seconds you have used every application.
It generates a json file every run and the file is updated every 10 seconds (to avoid excessive IO operations) with updated information.

The intention of this tool is to collect usage statistics for all programs every day, so after 
a long period of time you can build a report and analise your activity.


This tools was made for personal use, but feel free to use it yourself.
 
### Requirements 
- A linux system
- Command `xdotool` to get the selected window
- Command `jcmd` to differentiate between java apps
 