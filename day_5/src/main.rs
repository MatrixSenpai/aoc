#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cmp::{Ord, Ordering};

fn main() {
    let input = std::fs::read_to_string("sample_input.txt")
        .expect("Could not read in input");

    part_1(input);
}

fn part_1(input: String) {
    let lines: Vec<Line> = input.split("\n")
        .map(String::from)
        .map(Line::from)
        .collect();

    let mut all_points: Vec<Point> = lines.into_iter()
        .flat_map(|l| l.calculate_line())
        .collect::<Vec<Point>>();

    // println!("Order before sort: {:?}", all_points);
    // quick_sort(&mut all_points);
    // println!("Order after sort: {:?}", all_points);


    // let mut overlapping_points: Vec<Point> = Vec::new();

    // for line_i in lines.iter() {
    //     let points_i = line_i.calculate_line();
    //     for line_j in lines.iter() {
    //         if line_i == line_j { continue }
    //         for point in points_i.iter() {
    //             if line_j.check_point_overlapping(&point) && !overlapping_points.contains(&point) {
    //                 overlapping_points.push(point.clone());
    //             }
    //         }
    //     }
    // }

    // println!("{}", overlapping_points.len());
}

fn quick_sort(vec: &mut Vec<Point>) {
    let len = vec.len();
    _quick_sort(vec, 0, (len - 1) as isize);
}
fn _quick_sort(vec: &mut Vec<Point>, low: isize, high: isize) {
    if low < high {
        let p = partition(vec, low, high);
        _quick_sort(vec, low, p - 1);
        _quick_sort(vec, p + 1, high);
    }
}
fn partition(vec: &mut Vec<Point>, low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while vec[store_index as usize] < vec[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && vec[last_index as usize] > vec[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            vec.swap(store_index as usize, last_index as usize);
        }
    }
    vec.swap(store_index as usize, last_index as usize);
    store_index
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
impl From<String> for Point {
    fn from(input: String) -> Self {
        let coords: Vec<usize> = input.trim()
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect();

        Point::new(coords[0], coords[1])
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let cx = self.x.cmp(&other.x);
        let cy = self.y.cmp(&other.y);
        match cx {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                match cy {
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Less => Ordering::Less,
                }
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct Line{
    start: Point,
    end  : Point,
}
impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    fn calculate_line(&self) -> Vec<Point> {
        let calc_axis_start: usize;
        let calc_axis_end  : usize;
        let static_axis    : usize;
        let is_x           : bool;

        if !(self.start.x == self.end.x || self.start.y == self.end.y) {
            return vec![];
        }

        if self.start.x != self.end.x {
            calc_axis_start = if self.start.x < self.end.x { self.start.x } else { self.end.x };
            calc_axis_end   = if self.start.x > self.end.x { self.start.x } else { self.end.x };
            static_axis = self.start.y;
            is_x = true;
        } else if self.start.y != self.end.y {
            calc_axis_start = if self.start.y < self.end.y { self.start.y } else { self.end.y };
            calc_axis_end   = if self.start.y > self.end.y { self.start.y } else { self.end.y };
            static_axis = self.start.x;
            is_x = false;
        } else {
            return vec![self.start, self.end];
        }

        assert!(calc_axis_start < calc_axis_end);

        let mut points: Vec<Point> = Vec::new();
        for p in calc_axis_start..=calc_axis_end {
            if is_x {
                points.push(Point::new(p, static_axis));
            } else {
                points.push(Point::new(static_axis, p));
            }
        }

        points
    }

    fn check_point_overlapping(&self, point: &Point) -> bool {
        let line = self.calculate_line();
        for p in line {
            if &p == point { return true }
        }

        false
    }
}
impl From<String> for Line {
    fn from(input: String) -> Self {
        let line: Vec<Point> = input.split("->")
            .map(String::from)
            .map(Point::from)
            .collect();

        let start = line[0];
        let end = line[1];

        Line::new(start, end)
    }
}