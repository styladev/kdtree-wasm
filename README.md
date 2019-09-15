# kdtree-fast

KD-tree implementation targeting WebAssembly, written in Rust and taylored for Node.js environments.

### Build

```sh
$ make
```

### Usage

```sh
$ yarn add kdtree-fast
```

```javascript
const KdTree = require("kdtree-fast").KdTree;

const tree = new KdTree(2);

tree.add([1, 1], 1);
tree.add([3, 4], 3);
tree.add([4, 9], 5);
tree.add([7, 1], 7);
tree.add([8, 8], 23);
tree.add([2, 6], 8);

// the size of the kdtree..
tree.size();
// == 6

// the three nearest points..
tree.nearest([1, 2], 3);
// == [ [ 1, 1 ], [ 8, 3 ], [ 17, 8 ] ]

// all points within a distance of 10..
tree.within([1, 2], 10);
// == [ [ 1, 1 ], [ 8, 3 ] ]
```
