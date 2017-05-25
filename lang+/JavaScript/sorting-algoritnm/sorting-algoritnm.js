// JS十大排序算法

class SortingAlgoritnm {

    // 冒泡排序
    // @param array arr
    // @return array
    bubble(arr) {
        var len = arr.length;

        for (var i = 0; i < len - 1; i++) {
            for (var j = i + 1; j < len; j++) {
                if (arr[i] > arr[j]) {
                    var tmp = arr[i];
                    arr[i] = arr[j];
                    arr[j] = tmp;
                }
            }
        }

        return arr;
    }

    // 选择排序
    // @param array arr
    // @return array
    selection(arr) {
        var len = arr.length;

        for (var i = 0; i < len - 1; i++) {
            var minIndex = i;
            var tmp;

            /**
             * 找到还没有排序中最小的下标
             */
            for (var j = i + 1; j < len; j++) {
                if (arr[j] < arr[minIndex]) {
                    minIndex = j;
                }
            }

            tmp = arr[i];
            arr[i] = arr[minIndex];
            arr[minIndex] = tmp;
        }

        return arr;
    }
}


module.exports = SortingAlgoritnm;
