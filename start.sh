
day=$(date +%d)
echo "Starting day $day"
cargo download $day
cargo scaffold $day
