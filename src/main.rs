mod shapes;

fn main() {
    println!("** Working with Circle **");
    let mut circle = shapes::circle::Circle::new((0.0, 0.0), 5.0);
    println!("Circle center: {:?}", circle.get_center());
    println!("Circle radius: {}", circle.get_radius());
    circle.set_radius(10.0);
    println!("Updated Circle radius: {}", circle.get_radius());
    println!("Circle debug output: {:?}", circle);

    println!("** Working with Rectangle **");
    let mut rectangle = shapes::rectangle::Rectangle::new((-5.0, 15.0), 10.0, 5.0);
    println!("Rectangle top left: {:?}", rectangle.get_top_left());
    println!("Rectangle bottom right: {:?}", rectangle.get_bottom_right());
    rectangle.set_bottom_right((15.0, 10.0));
    println!(
        "Updated Rectangle bottom right: {:?}",
        rectangle.get_bottom_right()
    );
    println!("Updated Rectangle width: {}", rectangle.get_width());
    println!("Updated Rectangle height: {}", rectangle.get_height());

    println!("** Working with Square **");
    let mut square = shapes::square::Square::new((10.0, 20.0), 5.0);
    println!("Square top left: {:?}", square.get_top_left());
    println!("Square bottom right: {:?}", square.get_bottom_right());
    square.set_side_length(10.0);
    println!(
        "Updated Square bottom right: {:?}",
        square.get_bottom_right()
    );

    println!("*** Working with Shape Trait **");
    // #region first possibility
    let shapes: Vec<&dyn shapes::Shape> = vec![&circle, &rectangle, &square];
    // #endregion

    // #region second possibility
    // let shapes: Vec<Box<dyn shapes::Shape>> =
    //     vec![Box::new(circle), Box::new(rectangle), Box::new(square)];
    // #endregion

    // #region third possibility
    // let shapes: Vec<Box<dyn shapes::Shape>> = vec![
    //     Box::new(circle.clone()),
    //     Box::new(rectangle.clone()),
    //     Box::new(square.clone()),
    // ];
    // #endregion

    for shape in shapes {
        println!("Shape area: {}", shape.area());
        println!("Shape perimeter: {}", shape.perimeter());
        println!("Shape center: {:?}", shape.center());
        println!(
            "Shape distance from origin: {}",
            shape.distance_from_origin()
        );
    }
}
