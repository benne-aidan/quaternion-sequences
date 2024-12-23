#! /bin/bash
#SBATCH --time=24:00:00
#SBATCH --mem=2000M
#SBATCH --account=def-cbright

./sortpairs_split.sh wts 11
