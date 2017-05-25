const assert = require('chai').assert;
const Aligoritnm = require('./../sorting-algoritnm.js');

const aligoritnm = new Aligoritnm();

describe('JavaScript 十大排序算法', function() {

    describe('#冒泡排序', function() {
        var arr = [14, 31, 2, 65, 23, 7, 3, 342, 6, 1, 6];
        var res = aligoritnm.bubble(arr);

        for (var i = 0; i < res.length - 1; i++) {
            it(`${res[i]} should not gt ${res[i+1]}`, function(j) {
                return function() {
                    assert.isTrue(res[j] <= res[j + 1]);
                };
            }(i));
        }
    });

});
