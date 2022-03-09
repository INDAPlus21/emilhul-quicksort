# Quicksort algorithm by Emil Hultcrantz

An implementation of the quicksort algorithm (for integers at most 32-bit) in rust. Created for the Kattis problem [Quicksort](https://kth.kattis.com/problems/kth.alginda.quicksort) (Note on KTH Kattis).

 Uses insertion sort for arrays smaller than 24. Uses the median of first, middle and last element for pivot selection. If array size is larger than 128 instead use the ninther as the pivot. Sampling from first, middle and last element in each third of the array. Also implements tail call recursion optimization.

## How to run

* Clone repo
* Navigate to cloned repo
* Run with command 'cargo run' in terminal
* Input a line of integers where the first one should be the number of elements. E.g. `n e1 e2 ... en` in terminal
* Input EOF (`ctrl + d` on my machine) in terminal
* The program will print the sorted array to the terminal
