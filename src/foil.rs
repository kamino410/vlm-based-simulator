use impl_data::Point2D;

#[derive(Debug)]
pub struct Foil {
    pub name: String,
    /// Points on lower surface (order : top -> bottom).
    /// Contains both of top point and bottom point.
    pub low_ps: Vec<Point2D>,
    /// Points on upper surface (order : top -> bottom).
    /// Contains both of top point and bottom point.
    pub upp_ps: Vec<Point2D>,
    /// Points on mid line (order : top -> bottom).
    pub mid_ps: Vec<Point2D>,
}

impl Foil {
    /// Import foil data from dat file
    pub fn import(filename: &str) -> Result<Foil, &'static str> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let f =
            File::open(filename).map_err(|_| "Filed to open file (check if the file exists)")?;
        let mut buf_f = BufReader::new(&f);

        let mut name = String::new();
        buf_f
            .read_line(&mut name)
            .map_err(|_| "Failed to read line")?;
        name.pop();

        let mut low_ps = Vec::<Point2D>::new();
        let mut upp_ps = Vec::<Point2D>::new();
        let mut reached_top = false;
        for (_num, line) in buf_f.lines().enumerate() {
            let s = line.map_err(|_| "Failed to read line")?;
            let s = s.trim();
            if s.len() > 1 {
                let sp = s.split_whitespace().collect::<Vec<&str>>();
                let x = sp
                    .get(0)
                    .ok_or_else(|| "Invalid file format")
                    .and_then(|s| s.parse::<f64>().map_err(|_| "Value is not a number"))?;
                let z = sp
                    .get(1)
                    .ok_or_else(|| "Invalid file format")
                    .and_then(|s| s.parse::<f64>().map_err(|_| "Value is not a number"))?;
                if !reached_top {
                    if upp_ps.len() > 0 && upp_ps.last().unwrap().x < x {
                        low_ps.push(upp_ps.last().unwrap().clone());
                        low_ps.push(Point2D { x, z });
                        upp_ps.reverse();
                        reached_top = true;
                    } else {
                        upp_ps.push(Point2D { x, z });
                    }
                } else {
                    low_ps.push(Point2D { x, z });
                }
            }
        }
        let mid_ps = Foil::comp_mid_line(&upp_ps, &low_ps, 100);
        Ok(Foil {
            name,
            low_ps,
            upp_ps,
            mid_ps,
        })
    }

    fn comp_mid_line(
        upp_ps: &Vec<Point2D>,
        low_ps: &Vec<Point2D>,
        num_points: usize,
    ) -> Vec<Point2D> {
        let (top_x, bot_x) = (upp_ps.first().unwrap().x, upp_ps.last().unwrap().x);
        (0..num_points)
            .map(|i| top_x + (bot_x - top_x) * i as f64 / (num_points - 1) as f64)
            .map(|x| {
                let z = (Foil::interpolate_z(upp_ps, x) + Foil::interpolate_z(low_ps, x)) / 2.0;
                Point2D { x, z }
            }).collect()
    }

    /// Calculate z at given x by linear interpolation
    fn interpolate_z(ps: &Vec<Point2D>, x: f64) -> f64 {
        let (top_x, bot_x) = (ps.first().unwrap().x, ps.last().unwrap().x);
        let x = top_x + x * (bot_x - top_x);

        if x <= top_x {
            return ps.first().unwrap().z;
        }
        for i in 1..ps.len() {
            let (left, right) = (ps[i - 1], ps[i]);
            if left.x < x && x <= right.x {
                return left.z + (right.z - left.z) / (right.x - left.x) * (x - left.x);
            }
        }
        ps.last().unwrap().z
    }
}
