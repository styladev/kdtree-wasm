var x = new (require("./pkg").KdTree)(2);

x.add([1, 1], 1);
x.add([3, 4], 3);
x.add([4, 9], 5);
x.add([7, 1], 7);
x.add([8, 8], 23);
x.add([2, 6], 8);

console.log(x.size());
console.log(x.nearest([1, 2], 6));
console.log(x.within([1, 2], 10));
