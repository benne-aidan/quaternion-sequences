#! /bin/sh

if [ ! -e tmp/ ]
then
	mkdir tmp/
fi

type=$1
n=$2


export LC_ALL=C

# For each rowsum directory
for dirname in results/pairs/$type/find_$n/*;
do
	if [ -d $dirname ]
	then
		for filename in $dirname/*.pair;
		do
			echo $filename
			base_name=$(basename "$filename" .pair)
			output_dir="$dirname/$base_name"
			mkdir -p $output_dir

			split -d -l 15000 "$filename" "$output_dir/${base_name}_part_"

			for split_file in "$output_dir/${base_name}_part_"*;
			do
				# sort each file 
				sort $split_file -o $split_file
			done

			merge_out="$dirname/${base_name}.pair.sorted"

			sort -m "$output_dir"/* > "$merge_out"
		done
	fi
done