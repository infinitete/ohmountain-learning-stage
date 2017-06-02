const assert = require('chai').assert;
const Algorithm = require('./../sorting-algorithm.js');

const algorithm = new Algorithm();

describe('JavaScript 十大排序算法', function() {

    describe('#冒泡排序', function() {
        var arr = [];

        for (var i=0; i<14; i++) {
            arr.push(Math.random() * 1000);
        }

        var res = algorithm.bubble(arr);

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

        var res = algorithm.selection(arr);

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

        var res = algorithm.insertion(arr);

        for (var i = 0; i < res.length - 1; i++) {
            it(`${res[i]} <= ${res[i+1]}`, function(j) {
                return function() {
                    assert.isTrue(res[j] <= res[j + 1]);
                };
            }(i));
        }
    });
});
