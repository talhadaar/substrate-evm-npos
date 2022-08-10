## declare an array variable
declare -a arr=("alice" "bob" "charlie" "dave" "eve" "ferdie")

## now loop through the above array
for i in "${arr[@]}"
do
   echo "$i"
   ./target/release/nfid-node purge-chain --base-path /tmp/$i --chain local
done