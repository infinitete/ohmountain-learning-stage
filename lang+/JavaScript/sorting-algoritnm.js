// JS十大排序算法

class SortingAlgoritnm {

    // 冒泡排序
    // @param array arr
    bubble(arr) {
        var len = arr.length;

        if (len == 0) {
            throw new Exception("Paramater arr must be an array that contains some value");
        }

        for (var i = 0; i < len; i++) {
            for (var j = i+1; j < len; j++) {
                if (arr[i] > arr[j]) {
                    var tmp = arr[i];
                    arr[i]  = arr[j];
                    arr[j]  = tmp;
                }
            }
        }

        return arr;
    }
}


module.exports = SortingAlgoritnm;
