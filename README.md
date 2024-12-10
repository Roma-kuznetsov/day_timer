# Day timer 
This program is designed to limit time on the computer.
It can work remotely without the Internet; therefore, there are options to bypass the program by changing the system time or canceling the process.

## Support OS
- *win 7 (verified)* 
- *win 8*
- *win 8.1*
- *win 10 (verified)*
- *win 11*

## Required Assembly Tools
- rustup = 1.63.0
- cargo >= 1.63.0

## Build Project for production
- Copy this repository
- Cmd => cargo build --release

# Config
The shutdown timer is initially set to 150 minutes. You can change this by adding the config.txt file to the root folder.
In the config.txt file you need to write the number 1 < n < 1000.

# Execution Process
You won't see the console; the program runs in the background.
During operation, the program will create a file current_day.txt and write data in dd-mmm format there.
Where dd is the current day of the month.
mmm - number of minutes passed for this day.
When the number mmm is greater than the number in the config.txt file, a script will be called that shuts down the computer after 30 seconds.
Upon completion of the work, a log.txt file will be created containing information about the result of the work and the date of its creation.
