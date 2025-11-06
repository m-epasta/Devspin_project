# INIT TEMPLATE

- cargo run -- init my-project --yes --template nextjs

# VERIFY 

ls -la my-app/

cat my-app/devbox.yaml

ls -la my-app/frontend/
ls -la my-app/api/

# START IN NORUN

- cargo run -- start my-project --dry-run --verbose

# START IN RUN

- cargo run -- start my-project --verbose

# START WITH FILTERING

- cargo run -- start my-app --only frontend --verbose

- cargo run -- start my-app --skip api --verbose