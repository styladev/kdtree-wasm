{
    const x = new (require("./pkg").KdTree)(2);

    x.add([1, 1], 1);
    x.add([3, 4], 3);
    x.add([4, 9], 5);
    x.add([7, 1], 7);
    x.add([8, 8], 23);
    x.add([2, 6], 8);

    console.log('2 dim', x.size());
    console.log('2 dim', x.nearest([1, 2], 6));
    console.log('2 dim', x.within([1, 2], 10));
}

{
    const x = new (require("./pkg").KdTree)(4);

    x.add([1, 1, 7, 1], 1);
    x.add([3, 4, 8, 8], 3);
    x.add([4, 9, 2, 6], 5);
    x.add([7, 1, 1, 1], 7);
    x.add([8, 8, 3, 4], 23);
    x.add([2, 6, 4, 9], 8);

    console.log('4 dim', x.size());
    console.log('4 dim', x.nearest([1, 2, 3, 4], 6));
    console.log('4 dim', x.within([1, 2, 3, 4], 40));
}
