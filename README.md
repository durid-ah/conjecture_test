### Conjecture Test

This is a modified version of a math assignment to test the following conjecture for as many numbers as possible within 24-hours.

"that the number of 2's in the prime factorization of nCr(2n,n)  for positive integer n is equal to the number of 1's in the binary expansion of n"

The initial assignment ran a Python script that tested every number sequentially. The current version is a rust program where the main thread passes a closure with an arbitrary range of numbers. The closure is passed to a thread pool that calls it. The main thread can queue up to three batches of numbers until it blocks waiting for the channel to have an empty spot for the next batch, this prevents the main thread from filling up the memory with batches that might not get executed. It also gives the chance for another thread to execute while it is blocking.