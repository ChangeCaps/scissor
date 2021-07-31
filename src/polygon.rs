use std::collections::{BTreeMap, BTreeSet};

use glam::Vec2;

/// Polygon defined by a list of lines.
#[derive(Clone, Debug, Default)]
pub struct Polygon {
    pub points: Vec<Vec2>,
    pub is_ccw: Option<bool>,
    pub is_convex: Option<bool>,
    pub is_simple: bool,
}

impl Polygon {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(capacity),
            is_ccw: None,
            is_convex: None,
            is_simple: false,
        }
    }

    #[inline]
    pub fn push(&mut self, point: Vec2) {
        self.points.push(point);
    }

    #[inline]
    pub fn remove_intersection(&mut self) {
        let intersections = self.intersections();

        let mut index = 0;
        let mut max = 0;

        for (i, intersections) in intersections {
            if i < max {
                continue;
            }

            if let Some(e) = intersections.into_iter().rev().next() {
                let n = e - i;

                for _ in 0..n {
                    self.points.remove(i + 1 - index);
                }

                index += n;
                max = e + 1;
            }
        }

        self.is_ccw = None;
    }

    #[inline]
    pub fn is_ccw(&mut self) -> bool {
        if let Some(is_ccw) = self.is_ccw {
            is_ccw
        } else {
            let mut sum = 0.0;

            for i in 0..self.points.len() {
                let p0 = self.points[i];
                let p1 = self.points[(i + 1) % self.points.len()];

                sum += (p1.x - p0.x) * (p1.y + p0.y);
            }

            let is_ccw = sum < 0.0;

            self.is_ccw = Some(is_ccw);

            is_ccw
        }
    }

    /// Finds self intersections using a sweep line.
    ///
    /// Should run O(n log n).
    #[inline]
    pub fn intersections(&self) -> BTreeMap<usize, BTreeSet<usize>> {
        #[derive(PartialEq)]
        struct Node2(Vec2);

        impl Eq for Node2 {}

        impl Ord for Node2 {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(&other).unwrap()
            }
        }

        impl PartialOrd for Node2 {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                if self.0.y == other.0.y {
                    self.0.x.partial_cmp(&other.0.x)
                } else {
                    self.0.y.partial_cmp(&other.0.y)
                }
            }
        }

        #[derive(Debug, PartialEq, PartialOrd)]
        struct Node(f32);

        impl Eq for Node {}

        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(&other).unwrap()
            }
        }

        let events: BTreeMap<_, _> = (0..self.points.len())
            .into_iter()
            .map(|i| (Node2(self.points[i]), i))
            .collect();

        let mut sweep_status = BTreeMap::<Node, usize>::new();

        let mut intersections: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

        for (node, i) in events {
            let node = Node(node.0.x);

            if sweep_status.remove(&node).is_none() {
                sweep_status.insert(node, i);
            }

            let prev = (i + self.points.len() - 1) % self.points.len();
            let node = Node(self.points[prev].x);

            if sweep_status.remove(&node).is_none() {
                sweep_status.insert(node, prev);
            }

            let mut i = sweep_status.values().peekable();

            for _ in 0..sweep_status.len().max(1) - 1 {
                let a = *i.next().unwrap();
                let b = **i.peek().unwrap();

                let a0 = self.points[a];
                let a1 = self.points[(a + 1) % self.points.len()];

                let b0 = self.points[b];
                let b1 = self.points[(b + 1) % self.points.len()];

                let d = (b1.y - b0.y) * (a1.x - a0.x) - (b1.x - b0.x) * (a1.y - a0.y);

                let intersects = if d != 0.0 {
                    let ua = ((b1.x - b0.x) * (a0.y - b0.y) - (b1.y - b0.y) * (a0.x - b0.x)) / d;
                    let ub = ((a1.x - a0.x) * (a0.y - b0.y) - (a1.y - a0.y) * (a0.x - b0.x)) / d;

                    ua > 0.0 && ua < 1.0 && ub > 0.0 && ub < 1.0
                } else {
                    false
                };

                if intersects {
                    intersections.entry(a).or_default().insert(b);
                    intersections.entry(b).or_default().insert(a);
                }
            }
        }

        intersections
    }

    #[inline]
    pub fn clean(&mut self) {
        let mut i = 1;

        while i <= self.points.len() {
            let p0 = self.points[i - 1];
            let p1 = self.points[i % self.points.len()];
            let p2 = self.points[(i + 1) % self.points.len()];

            let d = (p1 - p0).normalize().dot((p2 - p1).normalize());

            if d >= 0.99999 || p0 == p1 {
                self.points.remove(i);
            } else {
                i += 1;
            }
        }
    }

    #[inline]
    pub fn make_simple(&mut self) {
        if self.is_simple {
            return;
        }

        self.clean();

        if !self.is_convex() {
            self.remove_intersection();
            self.clean();
        }
    }

    #[inline]
    pub fn is_convex(&mut self) -> bool {
        if let Some(is_convex) = self.is_convex {
            is_convex
        } else {
            let mut is_convex = true;

            for i in 1..self.points.len() {
                let p0 = self.points[i - 1];
                let p1 = self.points[i % self.points.len()];
                let p2 = self.points[(i + 1) % self.points.len()];

                if (p0 - p1).extend(0.0).cross((p2 - p1).extend(0.0)).z <= 0.0 {
                    is_convex = false;
                    break;
                }
            }

            self.is_convex = Some(is_convex);

            is_convex
        }
    }

    #[inline]
    pub fn verify(&mut self) {
        self.make_simple();

        if !self.is_ccw() {
            self.points.reverse();
        }
    }

    /// Appends hole to self.
    pub fn merge_hole(&mut self, mut hole: Polygon) {
        let (i, y) = hole
            .points
            .iter()
            .map(|v| v.y)
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        let (p, _) = self
            .points
            .iter()
            .enumerate()
            .filter_map(|(_i, v)| {
                if v.y > y {
                    Some((_i, v.distance(hole.points[i])))
                } else {
                    None
                }
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        hole.points.rotate_left(i);
        hole.points.push(hole.points[0]);

        self.points.insert(p + 1, self.points[p]);

        for point in hole.points {
            self.points.insert(p + 1, point);
        }
    }

    /// Triangulates the polygon using ear clipping.
    ///
    /// # Requirements
    /// 1. **Must** contain three or more points.
    /// 2. **Must** be counter clockwise winding order.
    #[inline]
    pub fn triangulate(&self) -> Vec<usize> {
        #[inline]
        fn is_convex(points: &Vec<Vec2>, relations: &Vec<(usize, usize)>, i: usize) -> bool {
            let (prev, next) = relations[i];

            let p0 = points[prev];
            let p1 = points[i];
            let p2 = points[next];

            (p0 - p1).extend(0.0).cross((p2 - p1).extend(0.0)).z > 0.0
        }

        #[inline]
        fn is_ear(
            points: &Vec<Vec2>,
            relations: &Vec<(usize, usize)>,
            reflect: &BTreeSet<usize>,
            i: usize,
        ) -> bool {
            let (prev, next) = relations[i];

            let p0 = points[prev];
            let p1 = points[i];
            let p2 = points[next];

            let v01 = p1 - p0;
            let v12 = p2 - p1;
            let v20 = p0 - p2;

            for i in reflect {
                let p = points[*i];

                if p == p0 || p == p1 || p == p2 {
                    continue;
                }

                let v0p = p - p0;
                let v1p = p - p1;
                let v2p = p - p2;

                let c0 = v01.extend(0.0).cross(v0p.extend(0.0)).z;
                let c1 = v12.extend(0.0).cross(v1p.extend(0.0)).z;
                let c2 = v20.extend(0.0).cross(v2p.extend(0.0)).z;

                if c0 <= 0.0 && c1 <= 0.0 && c2 <= 0.0 {
                    return false;
                }
            }

            true
        }

        #[inline]
        fn reconfigure(
            points: &Vec<Vec2>,
            relations: &Vec<(usize, usize)>,
            convex: &mut BTreeSet<usize>,
            reflect: &mut BTreeSet<usize>,
            ears: &mut BTreeSet<usize>,
            i: usize,
        ) {
            if reflect.contains(&i) {
                if is_convex(points, relations, i) {
                    reflect.remove(&i);
                    convex.insert(i);

                    if is_ear(points, relations, &reflect, i) {
                        ears.insert(i);
                    }
                }
            } else {
                let is_ear = is_ear(points, relations, reflect, i);

                if is_ear {
                    ears.insert(i);
                } else {
                    ears.remove(&i);
                }
            }
        }

        let mut relations = (0..self.points.len())
            .into_iter()
            .map(|i| {
                (
                    (i + 1) % self.points.len(),
                    (i + self.points.len() - 1) % self.points.len(),
                )
            })
            .collect();

        let mut convex = BTreeSet::new();
        let mut reflect = BTreeSet::new();

        for i in 0..self.points.len() {
            if is_convex(&self.points, &relations, i) {
                convex.insert(i);
            } else {
                reflect.insert(i);
            }
        }

        let mut ears = BTreeSet::new();

        for i in &convex {
            if is_ear(&self.points, &relations, &reflect, *i) {
                ears.insert(*i);
            }
        }

        let num_indices = (self.points.len() - 2) * 3;
        let mut indices = Vec::with_capacity(num_indices);

        loop {
            let ear = if let Some(ear) = ears.iter().next() {
                *ear
            } else {
                println!("{:?}", reflect);
                panic!("could not find ear, triangulation failed, this should not happen, please open an issue")
            };

            let (prev, next) = relations[ear];

            indices.push(prev);
            indices.push(ear);
            indices.push(next);

            if indices.len() == num_indices {
                break;
            }

            convex.remove(&ear);
            ears.remove(&ear);

            relations[prev].1 = next;
            relations[next].0 = prev;

            reconfigure(
                &self.points,
                &relations,
                &mut convex,
                &mut reflect,
                &mut ears,
                prev,
            );
            reconfigure(
                &self.points,
                &relations,
                &mut convex,
                &mut reflect,
                &mut ears,
                next,
            );
        }

        indices
    }
}

impl From<Vec<Vec2>> for Polygon {
    #[inline]
    fn from(points: Vec<Vec2>) -> Self {
        Self {
            points,
            is_ccw: None,
            is_convex: None,
            is_simple: false,
        }
    }
}
