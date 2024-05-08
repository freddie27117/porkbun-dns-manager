# Porkbun DNS Manager
### A simple way to interact with the Porkbun API to modify an existing DNS record
Right now this is only tested on linux, since this is where most server environments tend to be. I may add functionality for windows in the future.

## How to use:
1 - Add the binary file to your home directory and run porkbun-manager --install

2 - Go to ~/.config/porkbun-manager and edit dns-info.json to include all your relevant information

3 - Invoke porkbun-manager to update the DNS record

4 - Set up whatever cronjob you'd like, personally I use @reboot ~/porkbun-manager

5 - If you don't have an API key yet you can get one from here https://porkbun.com/account/api

## Notes
-The only two ways to invoke the manager right now is either no arguments (./porkbun-manager) or adding the --install flag

-No arguments will attempt to update your DNS record, while --install will create all the necessary files

-Running --install again will delete your current json data, so only do this if you need to and make sure you have your keys saved elsewhere

-If there is any issues you can check the log file where it will have the full response from the server.
