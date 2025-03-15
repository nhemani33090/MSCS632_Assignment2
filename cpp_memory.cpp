#include <iostream>

void allocateMemory() {
    int* arr = new int[1000000];  // Heap allocation
    std::cout << "Memory allocated for array.\n";
    
    delete[] arr;  // Manual deallocation
    std::cout << "Memory freed.\n";
}

int main() {
    allocateMemory();
    return 0;
}
    