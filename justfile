
build:
    cargo lambda build -p websub-lambda --release --output-format zip

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