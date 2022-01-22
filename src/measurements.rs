use std::collections::VecDeque;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Measurement {
    pub x: f64,
    pub y: f64,
}

impl Measurement {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct MeasurementIntoIterator<Iter> {
    iter: Iter,
}

impl<Iter> Iterator for MeasurementIntoIterator<Iter>
where
    Iter: Iterator<Item = Measurement>,
{
    type Item = egui::plot::Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(measurement) => Some(egui::plot::Value {
                x: measurement.x,
                y: measurement.y,
            }),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct MeasurementWindow {
    pub values: VecDeque<Measurement>,
    pub look_behind: usize,
}

impl MeasurementWindow {
    pub fn new_with_look_beind(look_behind: usize) -> Self {
        Self {
            values: VecDeque::new(),
            look_behind,
        }
    }

    pub fn add(&mut self, measurement: Measurement) {
        if let Some(last) = self.values.back() {
            if measurement.x < last.x {
                self.values.clear()
            }
        }

        self.values.push_back(measurement);

        let limit = self.values.back().unwrap().x - (self.look_behind as f64);
        while let Some(front) = self.values.front() {
            if front.x >= limit {
                break;
            }
            self.values.pop_front();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_measurements() {
        let w = MeasurementWindow::new_with_look_beind(123);
        assert_eq!(w.values.len(), 0);
        assert_eq!(w.look_behind, 123);
    }

    #[test]
    fn appends_one_value() {
        let mut w = MeasurementWindow::new_with_look_beind(100);

        w.add(Measurement::new(10.0, 20.0));
        assert_eq!(
            w.values.into_iter().eq(vec![Measurement::new(10.0, 20.0)]),
            true
        );
    }

    #[test]
    fn clears_on_out_of_order() {
        let mut w = MeasurementWindow::new_with_look_beind(100);

        w.add(Measurement::new(10.0, 20.0));
        w.add(Measurement::new(20.0, 30.0));
        w.add(Measurement::new(19.0, 100.0));
        assert_eq!(
            w.values.into_iter().eq(vec![Measurement::new(19.0, 100.0)]),
            true
        );
    }

    #[test]
    fn appends_several_values() {
        let mut w = MeasurementWindow::new_with_look_beind(100);

        for x in 1..=20 {
            w.add(Measurement::new((x as f64) * 10.0, x as f64));
        }

        assert_eq!(
            w.values.into_iter().eq(vec![
                Measurement::new(100.0, 10.0),
                Measurement::new(110.0, 11.0),
                Measurement::new(120.0, 12.0),
                Measurement::new(130.0, 13.0),
                Measurement::new(140.0, 14.0),
                Measurement::new(150.0, 15.0),
                Measurement::new(160.0, 16.0),
                Measurement::new(170.0, 17.0),
                Measurement::new(180.0, 18.0),
                Measurement::new(190.0, 19.0),
                Measurement::new(200.0, 20.0),
            ]),
            true
        );
    }
}
