
day=$(date +%d)
echo "Starting day $day"
cargo scaffold $day
cargo download $day
