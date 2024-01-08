use Box::{leaf, new, try_into};

struct Region {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

enum RegionQuadTree {
    Leaf(Region, Rgba),
    Branch([Box<RegionQuadTree>; 4]),
}

pub fn subdivide_until(&mut self, error_threshold: f32, min_region_length: usize) {
    loop {
        let new_quadtree = self
            .quadtree
            .subdivide(&self.image, error_threshold, min_region_length);
        match new_quadtree {
            Some(new_quadtree) => self.quadtree = new_quadtree,
            None => break,
        }
    }
}

fn subdivide(
    &self,
    image: &Image,
    error_threshold: f32,
    min_region_length: usize,
) -> Option<RegionQuadTree> {
    let region = self.region();
    if self.get_error(image) < error_threshold
        || region.width <= min_region_length
        || region.height <= min_region_length
    {
        return None;
    }

    match self {
        RegionQuadTree::Leaf(region, _) => {
            let half_width_l = (region.width as f64 / 2.0).floor() as usize;
            let half_width_r = (region.width as f64 / 2.0).ceil() as usize;
            let half_height_up = (region.height as f64 / 2.0).floor() as usize;
            let half_height_down = (region.height as f64 / 2.0).ceil() as usize;
            let children = [
                Box::new(RegionQuadTree::leaf(
                    region.x,
                    region.y,
                    half_width_l,
                    half_height_up,
                    image,
                )),
                Box::new(RegionQuadTree::leaf(
                    region.x,
                    region.y + half_height_up,
                    half_width_l,
                    half_height_down,
                    image,
                )),
                Box::new(RegionQuadTree::leaf(
                    region.x + half_width_l,
                    region.y,
                    half_width_r,
                    half_height_up,
                    image,
                )),
                Box::new(RegionQuadTree::leaf(
                    region.x + half_width_l,
                    region.y + half_height_up,
                    half_width_r,
                    half_height_down,
                    image,
                )),
            ];
            Some(RegionQuadTree::Branch(children))
        }
        RegionQuadTree::Branch(children) => {
            // call subdivide on all children = zip with existing children to usize
            // // as default later
            let sub_children: Vec<_> = children
                .iter()
                .map(|child| child.subdivide(image, error_threshold, min_region_length))
                .zip(children)
                .collect();
            // all children  returned None - avoid returning a superflous new tree.
            if sub_children.iter().all(|c| c.0.is_none()) {
                return None;
            }

            // replace any None result on children with the "old" child.
            let children = sub_children
                .into_iter()
                .map(|(nc, oc)| Box::new(nc.unwrap_or(*oc.clone())))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Some(RegionQuadTree::Branch(children))
        }
    }
}

fn main() {}
