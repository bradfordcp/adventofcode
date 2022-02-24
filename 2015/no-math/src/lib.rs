#[derive(Clone)]
pub struct PresentBox {
    pub l: u64,
    pub w: u64,
    pub h: u64,
}

impl PresentBox {
    fn sides_areas(&self) -> Vec<u64> {
        vec![self.l * self.w, self.l * self.h, self.w * self.h]
    }

    fn box_area(&self) -> u64 {
        self.sides_areas()
            .into_iter()
            .fold(0, |acc, area| acc + (2 * area))
    }

    fn slack_area(&self) -> u64 {
        self.sides_areas()
            .into_iter()
            .min()
            .expect("Couldn't find minimum area")
    }

    pub fn required_paper(&self) -> u64 {
        self.box_area() + self.slack_area()
    }

    fn side_perimeters(&self) -> Vec<u64> {
        vec![
            (self.l * 2) + (self.w * 2),
            (self.l * 2) + (self.h * 2),
            (self.w * 2) + (self.h * 2),
        ]
    }

    fn volume(&self) -> u64 {
        self.l * self.w * self.h
    }

    pub fn required_ribbon(&self) -> u64 {
        self.side_perimeters()
            .into_iter()
            .min()
            .expect("Could not find minimum perimeter")
            + self.volume()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sides_areas() {
        let present = PresentBox { l: 2, w: 3, h: 4 };

        assert_eq!(present.sides_areas(), vec![6, 8, 12]);
    }

    #[test]
    fn test_box_area() {
        let present = PresentBox { l: 2, w: 3, h: 4 };

        assert_eq!(present.box_area(), 52);
    }

    #[test]
    fn test_slack_area() {
        let present = PresentBox { l: 2, w: 3, h: 4 };

        assert_eq!(present.slack_area(), 6);
    }

    #[test]
    fn test_required_paper() {
        let present = PresentBox { l: 2, w: 3, h: 4 };
        assert_eq!(present.required_paper(), 58);

        let present = PresentBox { l: 1, w: 1, h: 10 };
        assert_eq!(present.required_paper(), 43);
    }

    #[test]
    fn test_side_perimeters() {
        let present = PresentBox { l: 2, w: 3, h: 4 };

        assert_eq!(present.side_perimeters(), vec![10, 12, 14]);
    }

    #[test]
    fn test_volume() {
        let present = PresentBox { l: 2, w: 3, h: 4 };

        assert_eq!(present.volume(), 24);
    }

    #[test]
    fn test_required_ribbon() {
        let present = PresentBox { l: 2, w: 3, h: 4 };
        assert_eq!(present.required_ribbon(), 34);

        let present = PresentBox { l: 1, w: 1, h: 10 };
        assert_eq!(present.required_ribbon(), 14);
    }
}
