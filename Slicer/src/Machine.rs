use string_builder::Builder;

pub struct Machine {
    printbedX: f32,
    printbedY: f32
}

// Note: We use absolute positioning, that allows us to segment them into quadrants. - Austin Haskell
impl Machine {
    pub fn Line(self: &Self, points: Vec<(f32, f32)>, scaler: f32) -> Vec<String> {
        let mut builder = Builder::default();

        for point in points {
            if !self.IsPointInBounds(point) {
                
            } else {

            }
        }
        
        // TODO: Finish Implementation - Austin Haskell
        vec![String::from("")]
    }

    pub fn Polyline(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Arc(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Polygon(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Rectangle(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Circle(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    pub fn Elipse(self: &Self) -> Vec<String> {
        vec![String::from("")]
        // TODO: Implementation - Austin Haskell
    }

    fn IsPointInBounds(self: &Self, point: (f32, f32)) -> bool {
        false
        // TODO: Implementation - Austin Haskell
    }

    fn CalcQuadrantForPoint(self: &Self, point: (i32, i32)) -> (i32, i32) {
        (0, 0)
        // TODO: Implementation - Austin Haskell
    }

    fn SplitLineAtBoundary(self: Self, point: (i32, i32)) -> ((f32, f32), (f32, f32)) {
        ((0.0, 0.0), (0.0, 0.0))
    }

}

#[test]
fn Line_DoesNotCrossBoundary_Translates() {
    let machine = Machine {
        printbedX: 16.0,
        printbedY: 16.0
    };

    let points: Vec<(i32, i32)> = Vec::new();

    let gcode = machine.Line(points, 1.0);

    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Line_CrossesBondary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polyline_CrossesBoundary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polyline_DoesNotCrossBoundary_Translates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Arc_CrossesBondary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Arc_DoesNotCrossBondary_Translates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polygon_CrossesBoundary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Polygon_DoesNotCrossBoundary_Translates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Rectangle_CrossesBoundary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Rectangle_DoesNotCrossBoundary_Translates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Circle_CrossesBoundary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Circle_DoesNotCrossBoundary_Translates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Elipse_CrossesBondary_Splits() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn Elipse_DoesNotCrossBondary_Translates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn IsPointInBounds_PointOutside_True() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn IsPointInBounds_PointInside_False() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn CalcQuadrantForPoint_PointOnBoarder_Calculates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}

#[test]
fn CalcQuadrantForPoint_Calculates() {
    assert_eq!(0, 1);
    // TODO: Implementation - Austin Haskell
}