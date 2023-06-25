# permit running this chmod +x ./run_web_client.sh

cd "$(dirname "$0")"

# if you want to precent install comment the below
npm --prefix ../web_client install

# run the app
npm --prefix ../web_client run start
