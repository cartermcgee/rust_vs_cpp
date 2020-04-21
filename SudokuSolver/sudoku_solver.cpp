#include <iostream>
#include <chrono>

//void print_array(int array[9][9]) {
//    std::cout << "\n";
//    for(int i = 0; i < 9; i++){
//        std::cout << "[ ";
//        for(int j = 0; j < 9; j++){
//            std::cout <<  array[i][j] << " ";
//        }
//        std::cout << "]" << std::endl;
//    }
//}

bool possible(int sudoku_grid[9][9], int y, int x, int n){
    for(int i = 0; i < 9; i++){
        if(sudoku_grid[y][i] == n || sudoku_grid[i][x] == n){ return false; }
    }
    int x_square = (x / 3) * 3;
    int y_square = (y / 3) * 3;
    for(int i = 0; i < 3; i++){
        for(int j = 0; j < 3; j++){
            if(sudoku_grid[y_square +i ][x_square + j] == n){ return false; }
        }
    }
    return true;
}

void solve(int sudoku_grid[9][9]){
    for(int y = 0; y < 9; y++){
        for(int x = 0; x < 9; x++){
            if(sudoku_grid[y][x] == 0){
                for(int n = 1; n < 10; n++){
                    if(possible(sudoku_grid, y, x, n)){
                        sudoku_grid[y][x] = n;
                        solve(sudoku_grid);
                        sudoku_grid[y][x] = 0;
                    }
                }
                return;
            }
        }
    }
    //print_array(sudoku_grid);
}

int main() {
    int sudoku_grid_easy[9][9] =
            {{6,0,9,0,8,0,0,2,0},
             {0,7,0,0,1,3,6,0,0},
             {0,3,2,6,0,9,0,0,0},
             {3,2,0,0,0,0,0,0,8},
             {0,5,7,0,2,0,3,1,0},
             {8,0,0,0,0,0,0,5,2},
             {0,0,0,8,0,4,7,6,0},
             {0,0,5,1,3,0,0,4,0},
             {0,1,0,0,7,0,8,0,5}};

    int sudoku_grid_medium[9][9] =
            {{8,0,0,0,0,9,1,0,4},
             {0,3,2,0,0,0,0,0,6},
             {5,9,0,0,8,0,0,0,0},
             {0,0,0,7,9,0,0,0,0},
             {2,0,0,4,5,6,0,0,1},
             {0,0,0,0,1,8,0,0,0},
             {0,0,0,0,4,0,0,8,5},
             {6,0,0,0,0,0,7,1,0},
             {3,0,1,8,0,0,0,0,2}};

    int sudoku_grid_hard[9][9] =
            {{0,0,7,0,2,0,3,0,0},
             {0,4,0,7,0,0,9,0,0},
             {0,1,0,0,3,0,0,0,7},
             {0,0,0,0,0,1,0,6,8},
             {0,5,0,4,0,6,0,2,0},
             {9,8,0,2,0,0,0,0,0},
             {1,0,0,0,4,0,0,9,0},
             {0,0,9,0,0,3,0,5,0},
             {0,0,8,0,1,0,6,0,0}};

    std::cout << "\nSolving Easy Sudoku Puzzles..." << std::endl;
    double total_easy = 0.0;
    for(int i = 0; i < 25; i++) {
        auto start = std::chrono::high_resolution_clock::now();
        solve(sudoku_grid_easy);
        auto finish = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::microseconds>(finish - start);
        double time = duration.count() / 100.0;
        total_easy += time;
    }
    std::cout << "Average: " << total_easy / 25 << "ms" << std::endl;

    std::cout << "\nSolving Medium Sudoku Puzzles..." << std::endl;
    double total_medium = 0.0;
    for(int i = 0; i < 25; i++) {
        auto start = std::chrono::high_resolution_clock::now();
        solve(sudoku_grid_medium);
        auto finish = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::microseconds>(finish - start);
        double time = duration.count() / 100.0;
        total_medium += time;
    }
    std::cout << "Average: " << total_medium / 25 << "ms" << std::endl;

    std::cout << "\nSolving Hard Sudoku Puzzles..." << std::endl;
    double total_hard = 0.0;
    for(int i = 0; i < 25; i++) {
        auto start = std::chrono::high_resolution_clock::now();
        solve(sudoku_grid_hard);
        auto finish = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::microseconds>(finish - start);
        double time = duration.count() / 100.0;
        total_hard += time;
    }
    std::cout << "Average: " << total_hard / 25 << "ms" << std::endl;
}