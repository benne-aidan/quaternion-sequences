#! /bin/bash
#SBATCH --time=24:00:00
#SBATCH --mem=2000M
#SBATCH --account=def-cbright
#SBATCH --mail-user=bennet43@uwindsor.ca
#SBATCH --mail-type=FAIL

./driver.sh 11
