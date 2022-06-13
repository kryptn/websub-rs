
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