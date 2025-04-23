/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd, // 必需的比较约束
{
	//TODO
    let n = array.len();
    if n <= 1 {
        return; // 处理空数组或单元素数组
    }

    for i in 0..n {
        let mut swapped = false; // 优化标志：提前终止有序数组
        for j in 0..n - i - 1 {
            // 比较相邻元素
            if array[j] > array[j + 1] {
                // 交换元素位置
                array.swap(j, j + 1);
                swapped = true; // 标记发生交换
            }
        }
        if !swapped {
            break; // 若没有交换，说明已排序完成
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}