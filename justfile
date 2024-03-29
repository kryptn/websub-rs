
build-websub-challenge:
    cargo lambda build -p websub-challenge-response --release --output-format zip
build-websub-notify:
    cargo lambda build -p websub-notify --release --output-format zip
build-websub-subscribe:
    cargo lambda build -p websub-subscribe --release --output-format zip
build-websub-add:
    cargo lambda build -p websub-add-subscription --release --output-format zip
build-websub-webhook:
    cargo lambda build -p websub-webhook --release --output-format zip
build-websub-renew:
    cargo lambda build -p websub-renew --release --output-format zip

build:
    just build-websub-challenge
    just build-websub-notify
    just build-websub-subscribe
    just build-websub-add
    just build-websub-webhook
    just build-websub-renew

deploy:
    just infra/ apply

deploy-auto:
    just infra/ apply-auto

redeploy:
    just infra/ redeploy

build-and-deploy:
    just build
    just deploy

build-and-redeploy:
    just build
    just redeploy

add-youtube channelid:
    cargo run -p websub-cli --release --quiet add-subscription "https://pubsubhubbub.appspot.com/subscribe" "https://www.youtube.com/xml/feeds/videos.xml?channel_id={{channelid}}"

add-spacex:
    just add-youtube UCtI0Hodo5o5dUb67FeUjDeA

add-lmg:
    # ltt
    just add-youtube UC0vBXGSyV14uvJ4hECDOl0Q

    # lmg
    just add-youtube UCXuqSBlHAE6Xw-yeJA0Tunw

    # short-circuit
    just add-youtube UCXuqSBlHAE6Xw-yeJA0Tunw

    # techlinked
    just add-youtube UCXuqSBlHAE6Xw-yeJA0Tunw


get-subs:
    cargo run -p websub-cli --release --quiet get-subscriptions

fmt:
    cargo fmt
    terraform fmt --recursive

