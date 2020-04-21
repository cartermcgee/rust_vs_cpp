#include <iostream>
#include <random>
#include <chrono>

void swap(int* a, int* b)
{
    int t = *a;
    *a = *b;
    *b = t;
}

int partition (int arr[], int low, int high)
{
    int pivot = arr[high];
    int i = (low - 1);
    for (int j = low; j <= high- 1; j++)
    {
        if (arr[j] <= pivot)
        {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return (i + 1);
}

void quickSort(int arr[], int low, int high)
{
    if (low < high)
    {
        int pivot = partition(arr, low, high);
        quickSort(arr, low, pivot - 1);
        quickSort(arr, pivot + 1, high);
    }
}

int main()
{
    std::cout << "Sorting arrays of size 10,000,000\n";
    int total_time = 0;
    for(int i = 0; i < 25; i++)
    {
        const int size = 10000000;
        int *array = new int[size];
        std::random_device rd;
        std::mt19937 mt(rd());
        std::uniform_real_distribution<double> dist(0, size);

        for(int j = 0; j < size; j++){
            array[j] = dist(mt);
        }
        auto start = std::chrono::high_resolution_clock::now();
        quickSort(array, 0, size - 1);
        auto finish = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(finish - start);
        total_time += duration.count();
        std::cout << "\tSorted array " << i << ", took "<< duration.count() << "ms" << std::endl;
        delete [] array;
    }
    std::cout << "\t------------------------------\n" << "\tAverage: " << total_time / 25 << "ms\n\n";
}