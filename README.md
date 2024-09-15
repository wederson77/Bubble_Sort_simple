---

# Bubble Sort

This repository provides a detailed explanation of **Bubble Sort**, one of the simplest and most well-known sorting algorithms. Bubble Sort is often used for educational purposes due to its simplicity, though it is not the most efficient for large datasets.

## Description

Bubble Sort is a sorting algorithm that works by repeatedly iterating through the list to be sorted, comparing each pair of adjacent elements and swapping them if they are in the wrong order. The process is repeated until the list is sorted.

## How It Works

1. **Initial Iteration:** The algorithm starts at the beginning of the list and goes through all the elements.

2. **Comparison and Swap:** For each pair of adjacent elements, if the first element is larger than the second, they are swapped.

3. **Complete Pass:** After a full pass through the list, the largest element is placed in its correct position.

4. **Repetition:** The process is repeated for the remaining list, ignoring the last (already sorted) position, until no swaps are needed during a full pass.

## Characteristics

- **Time Complexity:**
  - **Worst Case:** \(O(n^2)\)
  - **Best Case:** \(O(n)\), when the list is already sorted
  - **Average Case:** \(O(n^2)\)

- **Space Complexity:** \(O(1)\), as it is an in-place algorithm that does not require additional space for the list.

- **Stability:** Bubble Sort is stable, preserving the relative order of equal elements.

- **Simple to Implement:** It is easy to understand and implement but is not efficient for large lists due to its quadratic time complexity.

## Applications

Although Bubble Sort is not used in practical applications for sorting large lists due to its inefficiency, it is often used as an introduction to the concept of sorting algorithms in computer science courses.

## Contributions

Feel free to contribute to this repository by suggesting improvements to the documentation or alternative implementations.

---
