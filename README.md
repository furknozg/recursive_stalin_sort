# A sorting algorithm that is built upon the principle of Stalin sort

Yes, that sorting algorithm where no substantial result is returned but a partial ordered part is. The Joke algorithm in a sense,

but hear me out, there may be some merit to this algorithm. It just needs some polish to make it shine...

## How so?

Here's how it works, you first do the stalin sort, eliminating all the unordered elements and send them to the gulag, normally this means deleting the elements out of order but I believe that there are better ways of conveying a gulag such that it actually returns something substantial

Making the gulag into a seperate pool that you can implement stalin sort into, you can do this until you reach the end of the array.

Then you will have multiple pools of sorted sub arrays, meaning that you can implement merging to it semi-efficiently.

Then you can do a merging algorithm (my choice is k-way merging and am not sure if there's any good alternative)

## What this achieves

ABSOLUTELY NOTHING, but it is a funny idea to make a joke algorithm into something that actually works. The initial idea came through as a potential way of returning partially correct results (different slices of the array sorted).

Therefore achieving time efficient partial correctness with time constraints in mind, potentially for udp packet sorting in shorter time notices.

But currently the k-way merge algorithm cannot achieve that result and I am planning to implement another merge method which might achieve partially accurate completions of the array.

## Visual Representation

![stalin drawio](https://github.com/furknozg/recursive_stalin_sort/assets/72404454/024fbff2-9579-40af-bfaa-39b2b7069f9c)



PS: I know I use Comic sans, it is a good font and thats the end of this discussion.

## Time Complexity

Time: O(YES) or Theta(1==1)

Jokes aside the gulag part of the algorithm is the first order of operation which should take O(n) time by itself ideally (if the array is reverse sorted you're probably screwed and face around O(n^2)) although unlikely.

The second part, time depends on the merge algorithm chosen or implemented, I personally chose K-way sorting and like I said, am unsure if any other alternative exists in this scenario which takes O(n*log(k)) where k is the amount of branches created.
Which at the worst case (if the array is reverse sorted) is O(nlog(n)). Therefore, assuming the array is not reverse sorted in majority, probabilistically it can be O(nlog(n)) maybe, I dont know I havent done the calculations but I find it heavily unlikely. 

I believe that it is a skill issue in my part and somebody is going to break the news to me that this algorithm actually runs like O(n!) but until that day I will see this algo as O(n^2) at the worst of the worst.

PS: Also it is highly unoptimized I just took 1 hour to draft and implement this and I dont know if this algorithm actually exists with a different name but I thought it was cool to turn stalin sort into a feasible algorithm
