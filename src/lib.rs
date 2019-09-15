use kdtree;
use wasm_bindgen::prelude::*;

use serde;
use serde_derive::Serialize;

type PointUnit = f64;
type Point = Vec<PointUnit>;
type Data = f64;

#[derive(Serialize)]
struct Nearest {
    dist: f64,
    data: f64,
}

#[wasm_bindgen]
pub struct KdTree {
    kdtree: kdtree::KdTree<PointUnit, Data, Point>,
    dimensions: usize,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

fn assert_error(cond: bool, msg: &str) {
    if cond {
        return;
    }

    error(msg);
    panic!();
}

#[wasm_bindgen]
impl KdTree {
    #[wasm_bindgen(constructor)]
    pub fn new(
        dimensions: usize,
    ) -> KdTree {
        assert_error(
            dimensions > 0,
            "Dimensions must be set and >= 1",
        );

        KdTree {
            kdtree: kdtree::KdTree::new(dimensions),
            dimensions,
        }
    }

    pub fn size(&self) -> usize {
        self.kdtree.size()
    }

    #[inline(always)]
    fn check_point(
        &self,
        point: &Point,
    ) {
        assert_error(
            point.len() == self.dimensions,
            "Point dimensions do not match KdTree dimensions",
        );
    }

    pub fn within(
        &self,
        point: Point,
        radius: PointUnit,
    ) -> JsValue {
        self.check_point(&point);

        let val: Vec<(PointUnit, &Data)> = self.kdtree.within(
            &point.clone(),
            radius,
            &kdtree::distance::squared_euclidean,
        )
        .unwrap();

        JsValue::from_serde(&val).unwrap()
    }

    pub fn nearest(
        &mut self,
        point: Point,
        num: usize,
    ) -> JsValue {
        self.check_point(&point);

        let val: Vec<(PointUnit, &Data)> = self.kdtree.nearest(
            &point.clone(),
            num,
            &kdtree::distance::squared_euclidean,
        )
        .unwrap();

        JsValue::from_serde(&val).unwrap()
    }

    pub fn add(
        &mut self,
        point: Point,
        data: Data,
    ) -> () {
        self.check_point(&point);

        self.kdtree.add(
            point.clone(),
            data,
        ).unwrap()
    }

    pub fn drop(self) {
        drop(self);
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]

    use super::KdTree;

    #[test]
    fn it_basic_test() {
        let mut kdt = KdTree::new(
            2,
        );

        kdt.add(vec!(1.0, 2.0), 3.0);
        kdt.add(vec!(6.0, 9.0), 4.0);
        kdt.add(vec!(3.0, 7.0), 5.0);
        kdt.add(vec!(7.0, 4.0), 6.0);

        let nearest =
            kdt.nearest(
                vec!(5.0, 1.0),
                1,
            )
            .unwrap()[0];

        assert_eq!(nearest.0, 13.0f64);
        assert_eq!(*nearest.1, 6.0f64);
    }

    #[test]
    #[should_panic(expected = "Point has incorrect length")]
    fn it_checks_add_len() {
        let mut kdt = KdTree::new(
            1,
        );

        kdt.add(vec!(1.0, 2.0), 3.0);
    }

    #[test]
    #[should_panic(expected = "Point has incorrect length")]
    fn it_checks_nearest_len() {
        let mut kdt = KdTree::new(
            2,
        );

        kdt.add(vec!(1.0, 2.0), 3.0);

        kdt.nearest(
            vec!(5.0, 1.0, 2.0),
            1,
        );
    }

    #[test]
    #[should_panic(expected = "Point has incorrect length")]
    fn it_checks_within_len() {
        let mut kdt = KdTree::new(
            2,
        );

        kdt.add(vec!(1.0, 2.0), 3.0);

        kdt.within(
            vec!(5.0, 1.0, 2.0),
            1.0,
        );
    }

    #[test]
    fn it_within() {
        let mut kdt = KdTree::new(
            2,
        );

        kdt.add(vec!(1.0, 2.0), 3.0);

        let within = kdt.within(
            vec!(2.0, 1.0),
            20.0,
        ).unwrap()[0];

        assert_eq!(within.0, 2.0);
        assert_eq!(*within.1, 3.0);
    }
}
