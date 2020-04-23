#include <iostream>
#include <algorithm>

const int NUM_ITEMS = 10;
const int CAPACITY = 10;

int knapsack(int weights[NUM_ITEMS], int values[NUM_ITEMS], int n, int capacity, int memo[NUM_ITEMS + 1][CAPACITY + 1]){
    int result;
    if (memo[n][capacity] != -1){
        return memo[n][capacity];
    }

    if(n == 0 || capacity == 0){
        result = 0;
    } else if (weights[n] > capacity){
        result = knapsack(weights, values, n-1, capacity, memo);
    }else{

        int not_added = knapsack(weights, values, n-1, capacity, memo);
        int added = values[n] + knapsack(weights, values, n-1, capacity - weights[n], memo);
        result = std::max(not_added, added);
    }
    memo[n][capacity] = result;
    return result;
}


int main() {
    int v[NUM_ITEMS] = {2,4,6,2,7,9,3,2,5,10};
    int w[NUM_ITEMS] = {1,2,3,4,5,6,7,8,9,10};
    int memo[NUM_ITEMS + 1][CAPACITY + 1] = {{-1}};

    std::cout << "Max value: " << knapsack(w, v, NUM_ITEMS, CAPACITY, memo) << std::endl;
}

