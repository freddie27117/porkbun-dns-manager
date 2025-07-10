# Porkbun DNS Manager
### A simple way to interact with the Porkbun API to modify or create a DNS record
Since GoDaddy decided to locked out everyone with less than 50 domains from using the API, I switched to porkbun. I quickly found myself wanting a robust 'set and forget' DNS entry updater. I created this program as a simple way to achieve that.

## How to use:
1. Add the binary to your home directory (or wherever you want, I use /usr/local/bin) and from your terminal run './porkbun-manager --install'

2. Follow the prompts and insert all the relevant information.

3. That's it, you're setup. Run the binary and it will update the existing DNS entry, or create a new one.

Personally, I use a cronjob to run it nightly. Though depending on your needs you can run it more often. If you need to update the information you can run the install again, or go to .config/porkbun-manager/config.json

## Notes
- If you don't have an API key you can get one from [here.](https://porkbun.com/account/api)

- If there are any issues you can check the log file where it will have the full response from the server.

- The default TTL is 600, you don't need to change this unless you have a reason to.
