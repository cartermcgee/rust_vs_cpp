#include <iostream>
#include <random>
#include <chrono>

const int SIZE = 100000;

int binary_search(int array[], int lower, int upper, int target)
{
    if(upper >= lower){
        int mid = (lower + upper) / 2;
        if(array[mid] == target){
            return mid;
        }else if(array[mid] < target){
            return binary_search(array, mid + 1, upper, target);
        }else{
            return binary_search(array, lower, mid - 1, target);
        }
    }
}

int main() {
    int array[SIZE];
    std::random_device rd;
    std::mt19937 mt(rd());
    std::uniform_real_distribution<double> dist(0, SIZE);
    for(int i = 0; i < SIZE; i++){
        array[i] = i;
    }
    int search = dist(mt);
    auto start = std::chrono::high_resolution_clock::now();
    auto finish = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(finish - start);
    int index = binary_search(array, 0, SIZE, search);
    std::cout << "Found target at index " << index << std::endl << "\tTook " << duration.count() << " ms";
    return 0;
}
