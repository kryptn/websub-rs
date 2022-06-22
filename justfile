
build:
    cargo lambda build -p websub-challenge-response --release --output-format zip
    cargo lambda build -p websub-notify --release --output-format zip
    cargo lambda build -p websub-subscribe --release --output-format zip
    cargo lambda build -p websub-webhook --release --output-format zip
    cargo lambda build -p websub-renew --release --output-format zip

deploy:
    just infra/ apply

redeploy:
    just infra/ redeploy

build-and-deploy:
    just build
    just deploy

build-and-redeploy:
    just build
    just redeploy

add-spacex:
    cargo run -p websub-cli --release --quiet add-subscription "https://www.youtube.com/xml/feeds/videos.xml?channel_id=UCtI0Hodo5o5dUb67FeUjDeA" "https://pubsubhubbub.appspot.com/subscribe"

get-subs:
    cargo run -p websub-cli --release --quiet get-subscriptions