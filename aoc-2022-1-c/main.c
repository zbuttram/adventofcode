/**
 * Solution for Advent of Code 2022 Day 1 Co-Authored with GitHub Copilot (first C program)
 */

#include <stdio.h>

// print lines from a file named input.txt
int main() {
    // open the file in read mode
    FILE *fp = fopen("../input.txt", "r");
    // allocate a buffer to store each line of size 10 bytes which can store 9 characters and a null character
    char line[10];

    int count = 0;

    // allocate an array to store 3 integers
    int top_3[3];

    array_fill(top_3, 3, 0);

    while (fgets(line, 10, fp)) {
        // check if the line is empty
        if (line[0] == '\n') {
            // find the minimum of the integers in top 3
            int index;
            int min = array_min(top_3, 3, &index);

            if (count > min) {
                top_3[index] = count;
            }
            count = 0;
        } else {
            // parse the line as an integer
            int num = atoi(line);
            // add the number to the count
            count += num;
        }
    }

    // print the top 3
    // array_print(top_3, 3);

    int total = array_add(top_3, 3);

    // print the total
    printf("The total is %d", total);

    return 0;
}

// function that takes an array of integers and returns the minimum and stores its index in the index parameter
int array_min(int arr[], int n, int *index) {
    int min = arr[0];
    *index = 0;

    for (int i = 1; i < n; i++) {
        if (arr[i] < min) {
            min = arr[i];
            *index = i;
        }
    }

    return min;
}

int array_add(int arr[], int n) {
    int total = 0;

    for (int i = 0; i < n; i++) {
        total += arr[i];
    }

    return total;
}


void array_print(int arr[], int n) {
    for (int i = 0; i < n; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

void array_fill(int arr[], int n, int value) {
    for (int i = 0; i < n; i++) {
        arr[i] = value;
    }
}

// write a function that finds the maximum of an array of integers
int array_max(int arr[], int n) {
    int max = arr[0];

    for (int i = 1; i < n; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }

    return max;
}