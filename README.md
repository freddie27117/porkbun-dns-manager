# Porkbun DNS Manager
## A simple way to interact with the Porkbun API in order to and modify a DNS record
### Right now this is only tested on linux, since this is where most server environments tend to be. I may add functionality for windows in the future.

## How to use:
### Add the binary file to your home directory and run porkbun-manager --install
### go to ~/.config/porkbun-manager and edit dns-info.json to include all your relevant information
### Set up whatever cronjob you'd like, personally I use @reboot ~/porkbun-manager
### If you don't have an API key yet you can get one from here https://porkbun.com/account/api

## Notes
### The only two ways to invoke the manager right now is either no arguments (./porkbun-manager) or adding the --install flag
### NO arguments will attempt to update your DNS record, while --install will create all the necessary files
### Running --install again will delete your current json data, so only do this if you need to and make sure you have your keys saved elsewhere
