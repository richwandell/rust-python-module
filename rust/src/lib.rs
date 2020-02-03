use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize];
    let mut i = low - 1;
    for j in low..high {
        // If current element is smaller than the pivot
        if arr[j as usize] < pivot {
            i += 1;    // increment index of smaller element
            let temp = arr[i as usize];
            arr[i as usize] = arr[j as usize];
            arr[j as usize] = temp;
        }
    }
    let temp = arr[(i+1) as usize];
    arr[(i+1) as usize] = arr[high as usize];
    arr[high as usize] = temp;
    return i + 1;
}

fn quick_sort_fn(arr: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let pi = partition(arr, low, high);
        quick_sort_fn(arr, low, pi - 1);  // Before pi
        quick_sort_fn(arr, pi + 1, high); // After pi
    }
}

#[pyfunction]
fn quick_sort(arr: Vec<i32>) -> PyResult<Vec<i32>> {
    let mut nums = arr.to_vec();
    let length: i32 = nums.len() as i32;
    quick_sort_fn(&mut nums, 0, length - 1);
    return Ok(nums);
}


/// This module is a python module implemented in Rust.
#[pymodule]
fn wandell_rust(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(quick_sort))?;
    Ok(())
}
