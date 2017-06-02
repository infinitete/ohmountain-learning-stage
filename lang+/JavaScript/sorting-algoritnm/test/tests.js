const assert = require('chai').assert;
const Aligoritnm = require('./../sorting-algoritnm.js');

const aligoritnm = new Aligoritnm();

describe('JavaScript 十大排序算法', function() {

    describe('#冒泡排序', function() {
        var arr = [];

        for (var i=0; i<14; i++) {
            arr.push(Math.random() * 1000);
        }

        var res = aligoritnm.bubble(arr);

        for (var i = 0; i < res.length - 1; i++) {
            it(`${res[i]} <= ${res[i+1]}`, function(j) {
                return function() {
                    assert.isTrue(res[j] <= res[j + 1]);
                };
            }(i));
        }
    });

    describe('#选择排序', function() {
        var arr = [];

        for (var i=0; i<14; i++) {
            arr.push(Math.random() * 1000);
        }

        var res = aligoritnm.selection(arr);

        for (var i = 0; i < res.length - 1; i++) {
            it(`${res[i]} <= ${res[i+1]}`, function(j) {
                return function() {
                    assert.isTrue(res[j] <= res[j + 1]);
                };
            }(i));
        }
    });

    describe('#插入排序', function() {
        var arr = [];

        for (var i=0; i<14; i++) {
            arr.push(Math.random() * 1000);
        }

        var res = aligoritnm.insertion(arr);

        for (var i = 0; i < res.length - 1; i++) {
            it(`${res[i]} <= ${res[i+1]}`, function(j) {
                return function() {
                    assert.isTrue(res[j] <= res[j + 1]);
                };
            }(i));
        }
    });
});
