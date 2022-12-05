
day=$(date +%d)
echo "Starting day $day"
cargo download $day
cargo scaffold $day
sed -i "s/[0-9][0-9]/$day/g" .vscode/launch.json
