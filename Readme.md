# GAPOC
Genetic Algorithm Proof Of Concept

I was explaining genetic algorithms to a friend and did this overnight, not serious and badly realized, lots of room to optimization that I won't bother doing.

# What's it doing
You input an objective word (optional) and random words will converge to this objective word thanks to a genetic algorithm. The distance between the words is a levenshtein, so the mutation operations are the ones allowed by levenshtein. The population size is self explanatory.

# Usage
./gapoc \[-w|--word objective\_word\] \[-p|--population population\_size\]
